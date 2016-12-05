mod debugger_command;
mod http_handlers;
mod breakpoint_map;
mod cpu_snapshot;

use std::thread;
use std::sync::mpsc::{sync_channel, SyncSender};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use serde_json;
use iron::prelude::*;
use router::Router;
use websocket::{Server as WsServer, Message as WsMessage};

use super::Debugger;
use memory::Memory;
use cpu::registers::Registers;
use self::debugger_command::{DebuggerCommand, BreakReason};
use self::http_handlers::{ToggleBreakpointHandler, ContinueHandler, StepHandler};
use self::breakpoint_map::BreakpointMap;
use self::cpu_snapshot::CpuSnapshot;

const DEBUGGER_HTTP_ADDR: &'static str = "127.0.0.1:9975";
const DEBUGGER_WS_ADDR: &'static str = "127.0.0.1:9976";

pub struct HttpDebugger {
    ws_sender: Option<SyncSender<DebuggerCommand>>,
    breakpoints: Arc<Mutex<BreakpointMap>>,
    cpu_thread_handle: thread::Thread,
    is_stepping: Arc<AtomicBool>,
    prg_offset: u16,
    last_pc: u16,
}

impl HttpDebugger {
    pub fn new(prg_offset: u16) -> Self {
        HttpDebugger {
            breakpoints: Arc::new(Mutex::new(BreakpointMap::new())),
            ws_sender: None,
            cpu_thread_handle: thread::current(),
            is_stepping: Arc::new(AtomicBool::new(true)),
            last_pc: 0,
            prg_offset: prg_offset,
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
        let (debugger_tx, client_rx) = sync_channel::<DebuggerCommand>(0);
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
        let is_stepping = self.is_stepping.clone();

        thread::spawn(move || {
            let mut router = Router::new();
            router.get("/step", StepHandler::new(cpu_thread.clone()), "step");
            router.get("/continue",
                       ContinueHandler::new(cpu_thread.clone(), is_stepping),
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
            self.is_stepping.compare_and_swap(false, true, Ordering::Relaxed);
            true
        } else {
            false
        }
    }
}

impl Default for HttpDebugger {
    fn default() -> Self {
        Self::new(0)
    }
}

impl<Mem: Memory> Debugger<Mem> for HttpDebugger {
    fn on_step(&mut self, mem: &Mem, registers: &Registers, cycles: u64) {
        if let Some(ref sender) = self.ws_sender {
            let is_stepping = self.is_stepping.load(Ordering::Relaxed);
            if is_stepping || self.at_breakpoint(registers.pc) || self.last_pc == registers.pc {
                {
                    let snapshot = CpuSnapshot::new(mem.clone(),
                                                    registers.clone(),
                                                    cycles,
                                                    self.prg_offset as usize);

                    let break_reason = if self.last_pc == registers.pc {
                        info!("Trap detected @ {:0>4X}. CPU thread paused.", registers.pc);
                        BreakReason::Trap
                    } else if is_stepping {
                        info!("Stepping @ {:0>4X}.  CPU thread paused.", registers.pc);
                        BreakReason::Step
                    } else {
                        info!("Breakpoint hit @ {:0>4X}.  CPU thread paused.",
                              registers.pc);
                        BreakReason::Breakpoint
                    };
                    sender.send(DebuggerCommand::Break(break_reason, snapshot)).unwrap();
                }
                thread::park();
            }
        }
        self.last_pc = registers.pc;
    }
}
