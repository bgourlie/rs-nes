mod debugger_command;
mod http_handlers;
mod breakpoint_map;
mod cpu_snapshot;

use cpu::Cpu;
use cpu::debugger::Debugger;
use cpu::debugger::http_debugger::breakpoint_map::BreakpointMap;
use cpu::debugger::http_debugger::cpu_snapshot::{CpuSnapshot, MemorySnapshot};
use cpu::debugger::http_debugger::debugger_command::{BreakReason, DebuggerCommand};
use cpu::debugger::http_debugger::http_handlers::*;
use disassembler::Instruction;
use iron::prelude::*;
use memory::Memory;
use router::Router;
use serde_json;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{SyncSender, sync_channel};
use std::thread;
use websocket::{Message as WsMessage, Server as WsServer};

const DEBUGGER_HTTP_ADDR: &'static str = "127.0.0.1:9975";
const DEBUGGER_WS_ADDR: &'static str = "127.0.0.1:9976";

pub struct HttpDebugger<Mem: Memory> {
    ws_sender: Option<SyncSender<DebuggerCommand<Mem>>>,
    breakpoints: Arc<Mutex<BreakpointMap>>,
    cpu_thread_handle: thread::Thread,
    cpu_paused: Arc<AtomicBool>,
    last_pc: u16,
    last_mem_hash: u64,

    // TODO: this won't accommodate self-modifying code or bank-switching
    // In order to accommodate it, we would need to hash then invalidate and re-disassemble when
    // memory locations where PRG resides changes.
    instructions: Arc<Vec<Instruction>>,
}

impl<Mem: Memory> HttpDebugger<Mem> {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        HttpDebugger {
            breakpoints: Arc::new(Mutex::new(BreakpointMap::new())),
            ws_sender: None,
            cpu_thread_handle: thread::current(),
            cpu_paused: Arc::new(AtomicBool::new(true)),
            last_pc: 0,
            last_mem_hash: 0,
            instructions: Arc::new(instructions),
        }
    }

    pub fn start(&mut self) -> Result<(), String> {
        if self.ws_sender.is_some() {
            panic!("Start already called.");
        }

        self.start_http_server_thread()?;
        self.start_websocket_thread()?;
        Ok(())
    }

    fn start_websocket_thread(&mut self) -> Result<(), String> {
        info!("Starting web socket server at {}", DEBUGGER_WS_ADDR);
        let (debugger_tx, client_rx) = sync_channel::<DebuggerCommand<Mem>>(0);
        self.ws_sender = Some(debugger_tx);
        let mut ws_server = WsServer::bind(DEBUGGER_WS_ADDR).map_err(|e| e.to_string())?;
        info!("Waiting for debugger to attach");
        let connection = ws_server.accept();
        info!("Debugger attached!");
        thread::spawn(move || {
            let request = connection.unwrap().read_request().unwrap();
            request.validate().unwrap();
            let response = request.accept();
            let mut sender = response.send().unwrap();

            while let Ok(debugger_msg) = client_rx.recv() {
                let message: WsMessage = WsMessage::text(serde_json::to_string(&debugger_msg)
                    .unwrap());

                if sender.send_message(&message).is_err() {
                    break;
                }
            }

            info!("Websocket thread is terminating!")
        });

        Ok(())
    }

    fn start_http_server_thread(&self) -> Result<(), String> {
        info!("Starting http debugger at {}", DEBUGGER_HTTP_ADDR);
        let cpu_thread = self.cpu_thread_handle.clone();
        let breakpoints = self.breakpoints.clone();
        let cpu_paused = self.cpu_paused.clone();
        let instructions = self.instructions.clone();

        thread::spawn(move || {
            let mut router = Router::new();
            router.get("/step", StepHandler::new(cpu_thread.clone()), "step");
            router.get("/instructions",
                       InstructionHandler::new(instructions.clone()),
                       "instructions");
            router.get("/continue",
                       ContinueHandler::new(cpu_thread.clone(), cpu_paused),
                       "continue");
            router.get("/toggle_breakpoint/:addr",
                       ToggleBreakpointHandler::new(breakpoints.clone()),
                       "toggle_breakpoint");
            Iron::new(router).http(DEBUGGER_HTTP_ADDR).unwrap();
        });

        Ok(())
    }

    fn at_breakpoint(&self, pc: u16) -> bool {
        let breakpoints = &(*self.breakpoints.lock().unwrap());
        if breakpoints.is_set(pc) {
            self.cpu_paused.compare_and_swap(false, true, Ordering::Relaxed);
            true
        } else {
            false
        }
    }
}

impl<Mem: Memory> Default for HttpDebugger<Mem> {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}

impl<Mem: Memory> Debugger<Mem> for HttpDebugger<Mem> {
    fn on_step(&mut self, cpu: &Cpu<Mem>, cycles: u64) {
        if let Some(ref sender) = self.ws_sender {
            let cpu_paused = self.cpu_paused.load(Ordering::Relaxed);
            if cpu_paused || self.at_breakpoint(cpu.registers.pc) ||
               self.last_pc == cpu.registers.pc {
                {
                    let mem_snapshot = {
                        let hash = cpu.memory.hash();
                        if hash != self.last_mem_hash {
                            self.last_mem_hash = hash;
                            debug!("Memory changed. Sending full snapshot to client");
                            MemorySnapshot::Updated(hash, cpu.memory.clone())
                        } else {
                            MemorySnapshot::NoChange(hash)
                        }
                    };

                    let snapshot = CpuSnapshot::new(mem_snapshot, cpu.registers.clone(), cycles);
                    // TODO: All this shit is getting confusing and is in need of simplication
                    // Stepping deliberately takes the least precedence when deciding which
                    // BreakReason to send if multiple break reasons exist
                    if self.last_pc == cpu.registers.pc {
                        debug!("Trap detected @ {:0>4X}. CPU thread paused.",
                               cpu.registers.pc);
                        sender.send(DebuggerCommand::Break(BreakReason::Trap, snapshot)).unwrap();
                    } else if self.at_breakpoint(cpu.registers.pc) {
                        debug!("Breakpoint hit @ {:0>4X}.  CPU thread paused.",
                               cpu.registers.pc);
                        sender.send(DebuggerCommand::Break(BreakReason::Breakpoint, snapshot))
                            .unwrap();
                    } else if cpu_paused {
                        debug!("Stepping @ {:0>4X}.  CPU thread paused.", cpu.registers.pc);
                        sender.send(DebuggerCommand::Break(BreakReason::Step, snapshot)).unwrap();
                    };
                }
                thread::park();
            }
        }
        self.last_pc = cpu.registers.pc;
    }
}
