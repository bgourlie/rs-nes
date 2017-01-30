mod debugger_command;
mod http_handlers;
mod breakpoint_map;
mod cpu_snapshot;

use chan::{self, Receiver, Sender};
use cpu::Cpu;
use cpu::debugger::breakpoint_map::BreakpointMap;
use cpu::debugger::cpu_snapshot::{CpuSnapshot, MemorySnapshot};
use cpu::debugger::debugger_command::{BreakReason, DebuggerCommand};
use cpu::debugger::http_handlers::*;
use disassembler::{Instruction, InstructionDecoder};
use iron::prelude::*;
use memory::{ADDRESSABLE_MEMORY, Memory};
use router::Router;
use serde_json;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use websocket::{Message as WsMessage, Server as WsServer};

const DEBUGGER_HTTP_ADDR: &'static str = "127.0.0.1:9975";
const DEBUGGER_WS_ADDR: &'static str = "127.0.0.1:9976";

pub struct HttpDebugger<Mem: Memory> {
    ws_tx: Sender<DebuggerCommand>,
    ws_rx: Receiver<DebuggerCommand>,
    cpu: Arc<Mutex<Cpu<Mem>>>,
    breakpoints: Arc<Mutex<BreakpointMap>>,
    cpu_thread_handle: thread::Thread,
    cpu_paused: Arc<AtomicBool>,
    last_pc: u16,
    last_mem_hash: u64,
    instructions: Arc<Vec<Instruction>>, // TODO: https://github.com/bgourlie/rs-nes/issues/9
}

impl<Mem: Memory> HttpDebugger<Mem> {
    pub fn new(cpu: Cpu<Mem>) -> Self {
        let mut buf = Vec::new();
        cpu.memory.dump(&mut buf);
        let instructions = InstructionDecoder::new(&buf, cpu.registers.pc as usize).collect();
        let (ws_sender, ws_receiver) = chan::sync(0);
        HttpDebugger {
            ws_tx: ws_sender,
            ws_rx: ws_receiver,
            cpu: Arc::new(Mutex::new(cpu)),
            breakpoints: Arc::new(Mutex::new(BreakpointMap::new())),
            cpu_thread_handle: thread::current(),
            cpu_paused: Arc::new(AtomicBool::new(true)),
            last_pc: 0,
            last_mem_hash: 0,
            instructions: Arc::new(instructions),
        }
    }

    pub fn start(&mut self) -> Result<(), String> {
        self.start_http_server_thread()?;
        self.start_websocket_thread()?;
        Ok(())
    }

    pub fn step(&mut self) {
        let cpu = &mut (*self.cpu.lock().unwrap());
        if let Some(break_reason) = self.break_reason(cpu) {
            {
                let mem_snapshot = self.memory_snapshot(cpu);
                if let MemorySnapshot::Updated(hash, _) = mem_snapshot {
                    debug!("Memory altered");
                    self.last_mem_hash = hash;
                }

                let snapshot = CpuSnapshot::new(mem_snapshot, cpu.registers.clone());
                self.ws_tx.send(DebuggerCommand::Break(break_reason, snapshot));
            }
            thread::park();
        }
        self.last_pc = cpu.registers.pc;
        cpu.step();
    }

    fn break_reason(&self, cpu: &Cpu<Mem>) -> Option<BreakReason> {
        // Stepping deliberately takes the least precedence when determining the break reason
        if self.last_pc == cpu.registers.pc {
            debug!("Trap detected @ {:0>4X}. CPU thread paused.",
                   cpu.registers.pc);
            Some(BreakReason::Trap)
        } else if self.at_breakpoint(cpu.registers.pc) {
            debug!("Breakpoint hit @ {:0>4X}. CPU thread paused.",
                   cpu.registers.pc);
            Some(BreakReason::Breakpoint)
        } else if self.cpu_paused.load(Ordering::Relaxed) {
            debug!("Stepping @ {:0>4X}. CPU thread paused.", cpu.registers.pc);
            Some(BreakReason::Step)
        } else {
            None
        }
    }

    fn memory_snapshot(&self, cpu: &Cpu<Mem>) -> MemorySnapshot {
        let hash = cpu.memory.hash();
        if hash != self.last_mem_hash {
            let mut buf = Vec::with_capacity(ADDRESSABLE_MEMORY);
            cpu.memory.dump(&mut buf);
            MemorySnapshot::Updated(hash, buf)
        } else {
            MemorySnapshot::NoChange(hash)
        }
    }

    fn start_websocket_thread(&mut self) -> Result<(), String> {
        info!("Starting web socket server at {}", DEBUGGER_WS_ADDR);
        let mut ws_server = WsServer::bind(DEBUGGER_WS_ADDR).map_err(|e| e.to_string())?;
        info!("Waiting for debugger to attach");
        let connection = ws_server.accept();
        info!("Debugger attached!");
        let ws_rx = self.ws_rx.clone();
        thread::spawn(move || {
            let request = connection.unwrap().read_request().unwrap();
            request.validate().unwrap();
            let response = request.accept();
            let mut sender = response.send().unwrap();

            while let Some(debugger_msg) = ws_rx.recv() {
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
