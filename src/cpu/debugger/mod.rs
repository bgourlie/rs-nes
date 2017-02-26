mod debugger_command;
mod http_handlers;
mod breakpoint_map;
mod cpu_snapshot;

use asm6502::{Instruction, InstructionDecoder};
use byte_utils::from_lo_hi;
use chan::{self, Receiver, Sender};
use cpu::Cpu;
use cpu::debugger::breakpoint_map::BreakpointMap;
use cpu::debugger::cpu_snapshot::{CpuSnapshot, MemorySnapshot};
use cpu::debugger::debugger_command::{BreakReason, DebuggerCommand};
use cpu::debugger::http_handlers::*;
use errors::*;
use iron::prelude::*;
use memory::{ADDRESSABLE_MEMORY, Memory};
use router::Router;
use screen::Screen;
use serde::Serialize;
use serde_json;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;
use websocket::{Message as WsMessage, Server as WsServer};

const DEBUGGER_HTTP_ADDR: &'static str = "127.0.0.1:9975";
const DEBUGGER_WS_ADDR: &'static str = "127.0.0.1:9976";

#[derive(Eq, PartialEq)]
pub enum InterruptHandler {
    None,
    Irq,
    Nmi,
}

pub struct HttpDebugger<Mem: Memory, S: Screen + Serialize> {
    ws_tx: Sender<DebuggerCommand<S>>,
    ws_rx: Receiver<DebuggerCommand<S>>,
    cpu: Cpu<Mem>,
    breakpoints: Arc<Mutex<BreakpointMap>>,
    cpu_thread_handle: thread::Thread,
    cpu_paused: Arc<AtomicBool>,
    break_on_nmi: Arc<AtomicBool>,
    last_pc: u16,
    last_mem_hash: u64,
    screen: Rc<S>,
    instructions: Arc<Vec<Instruction>>, // TODO: https://github.com/bgourlie/rs-nes/issues/9
}

impl<Mem: Memory, S: Screen + Serialize> HttpDebugger<Mem, S> {
    pub fn new(cpu: Cpu<Mem>, screen: Rc<S>) -> Self {
        let mut buf = Vec::new();
        cpu.memory.dump(&mut buf);
        let instructions = InstructionDecoder::new(&buf, cpu.registers.pc).collect();
        let (ws_sender, ws_receiver) = chan::sync(0);
        HttpDebugger {
            ws_tx: ws_sender,
            ws_rx: ws_receiver,
            cpu: cpu,
            breakpoints: Arc::new(Mutex::new(BreakpointMap::new())),
            cpu_thread_handle: thread::current(),
            cpu_paused: Arc::new(AtomicBool::new(true)),
            break_on_nmi: Arc::new(AtomicBool::new(false)),
            last_pc: 0,
            last_mem_hash: 0,
            instructions: Arc::new(instructions),
            screen: screen,
        }
    }

    pub fn start(&mut self) -> Result<()> {
        self.start_http_server_thread()?;
        self.start_websocket_thread()?;
        Ok(())
    }

    pub fn step(&mut self) -> Result<()> {
        if let Some(break_reason) = self.break_reason() {
            {
                let snapshot = self.cpu_snapshot();
                self.ws_tx.send(DebuggerCommand::Break(break_reason, snapshot));
            }
            thread::park();
        }
        self.last_pc = self.cpu.registers.pc;
        let result = self.cpu.step();

        if let Err(Error(ErrorKind::Crash(ref reason), _)) = result {
            let snapshot = self.cpu_snapshot();
            self.ws_tx.send(DebuggerCommand::Crash(reason.clone(), snapshot));

            // Give the web socket thread enough time to send the Crash message
            thread::sleep(Duration::from_millis(100));
        }

        result
    }

    fn break_reason(&self) -> Option<BreakReason> {
        if self.interrupt_handler() == InterruptHandler::Nmi &&
           self.break_on_nmi.load(Ordering::Relaxed) {
            debug!("Break on NMI. CPU thread paused.");
            self.cpu_paused.compare_and_swap(false, true, Ordering::Relaxed);
            Some(BreakReason::Nmi)
        } else if self.last_pc == self.cpu.registers.pc {
            debug!("Trap detected @ {:0>4X}. CPU thread paused.",
                   self.cpu.registers.pc);
            Some(BreakReason::Trap)
        } else if self.at_breakpoint(self.cpu.registers.pc) {
            debug!("Breakpoint hit @ {:0>4X}. CPU thread paused.",
                   self.cpu.registers.pc);
            Some(BreakReason::Breakpoint)
        } else if self.cpu_paused.load(Ordering::Relaxed) {
            // Stepping deliberately takes the least precedence when determining the break reason
            debug!("Stepping @ {:0>4X}. CPU thread paused.",
                   self.cpu.registers.pc);
            Some(BreakReason::Step)
        } else {
            None
        }
    }

    fn cpu_snapshot(&mut self) -> CpuSnapshot<S> {
        let hash = self.cpu.memory.hash();
        let mem_snapshot = if hash != self.last_mem_hash {
            debug!("Memory altered");
            let mut buf = Vec::with_capacity(ADDRESSABLE_MEMORY);
            self.cpu.memory.dump(&mut buf);
            self.last_mem_hash = hash;
            MemorySnapshot::Updated(hash, buf)
        } else {
            MemorySnapshot::NoChange(hash)
        };

        let screen: S = (self.screen.as_ref()).clone();
        CpuSnapshot::new(mem_snapshot,
                         self.cpu.registers.clone(),
                         screen,
                         self.cpu.cycles)
    }

    fn start_websocket_thread(&mut self) -> Result<()> {
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

    fn start_http_server_thread(&self) -> Result<()> {
        info!("Starting http debugger at {}", DEBUGGER_HTTP_ADDR);
        let cpu_thread = self.cpu_thread_handle.clone();
        let breakpoints = self.breakpoints.clone();
        let cpu_paused = self.cpu_paused.clone();
        let break_on_nmi = self.break_on_nmi.clone();
        let instructions = self.instructions.clone();

        thread::spawn(move || {
            let mut router = Router::new();
            router.get("/step", StepHandler::new(cpu_thread.clone()), "step");
            router.get("/instructions",
                       InstructionHandler::new(instructions),
                       "instructions");
            router.get("/continue",
                       ContinueHandler::new(cpu_thread, cpu_paused),
                       "continue");
            router.get("/toggle_breakpoint/:addr",
                       ToggleBreakpointHandler::new(breakpoints),
                       "toggle_breakpoint");

            router.get("/toggle_break_on_nmi",
                       ToggleBreakOnNmiHandler::new(break_on_nmi),
                       "toggle_break_on_nmi");
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

    fn interrupt_handler(&self) -> InterruptHandler {
        if self.cpu.registers.pc == self.peek_mem(super::NMI_VECTOR) {
            InterruptHandler::Nmi
        } else if self.cpu.registers.pc == self.peek_mem(super::BREAK_VECTOR) {
            InterruptHandler::Irq
        } else {
            InterruptHandler::None
        }
    }

    fn peek_mem(&self, addr: u16) -> u16 {
        let low_byte = self.cpu.memory.read(addr).unwrap();
        let high_byte = self.cpu.memory.read(addr + 1).unwrap();
        from_lo_hi(low_byte, high_byte)
    }
}
