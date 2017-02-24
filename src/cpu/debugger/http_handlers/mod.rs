use super::breakpoint_map::BreakpointMap;
use asm6502::Instruction;
use cpu::registers::Registers;
use iron::{headers, status};
use iron::Handler;
use iron::modifier::Modifier;
use iron::prelude::*;
use router::{Params, Router};
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;
use serde_json;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::Thread;

impl Serialize for Registers {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("Registers", 6)?;
        state.serialize_field("acc", &self.acc)?;
        state.serialize_field("pc", &self.pc)?;
        state.serialize_field("sp", &self.sp)?;
        state.serialize_field("status", &self.status)?;
        state.serialize_field("x", &self.x)?;
        state.serialize_field("y", &self.y)?;
        state.end()
    }
}

#[derive(Serialize)]
pub struct ToggleBreakpointResponse {
    offset: u16,
    is_set: bool,
}

impl ToggleBreakpointResponse {
    pub fn new(offset: u16, is_set: bool) -> Self {
        ToggleBreakpointResponse {
            offset: offset,
            is_set: is_set,
        }
    }
}

#[derive(Serialize)]
pub struct ToggleBreakOnNmiResponse {
    is_set: bool,
}

impl ToggleBreakOnNmiResponse {
    pub fn new(is_set: bool) -> Self {
        ToggleBreakOnNmiResponse { is_set: is_set }
    }
}

pub struct ToggleBreakOnNmiHandler {
    break_on_nmi: Arc<AtomicBool>,
}

impl ToggleBreakOnNmiHandler {
    pub fn new(break_on_nmi: Arc<AtomicBool>) -> Self {
        ToggleBreakOnNmiHandler { break_on_nmi: break_on_nmi }
    }
}

impl Handler for ToggleBreakOnNmiHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        debug!("Toggle break-on-nmi request received!");
        let new_val = !self.break_on_nmi.load(Ordering::Relaxed);
        self.break_on_nmi.store(new_val, Ordering::Relaxed);
        let resp_model = ToggleBreakOnNmiResponse::new(new_val);
        let resp_body = serde_json::to_string(&resp_model).unwrap();
        let resp = response_with((status::Ok, resp_body));
        Ok(resp)
    }
}

pub struct ToggleBreakpointHandler {
    breakpoints: Arc<Mutex<BreakpointMap>>,
}

impl ToggleBreakpointHandler {
    pub fn new(breakpoints: Arc<Mutex<BreakpointMap>>) -> Self {
        ToggleBreakpointHandler { breakpoints: breakpoints }
    }
}

impl Handler for ToggleBreakpointHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        debug!("Toggle breakpoint request received!");
        if let Some(addr) = get_router(req).find("addr").and_then(|a| a.parse::<u16>().ok()) {
            let mut breakpoints = &mut (*self.breakpoints.lock().unwrap());
            let is_set = breakpoints.toggle(addr);
            let resp_model = ToggleBreakpointResponse::new(addr, is_set);
            let resp_body = serde_json::to_string(&resp_model).unwrap();
            let resp = response_with((status::Ok, resp_body));
            Ok(resp)
        } else {
            let resp = response_with((status::BadRequest));
            Ok(resp)
        }
    }
}

pub struct InstructionHandler {
    instructions: Arc<Vec<Instruction>>,
}

impl InstructionHandler {
    pub fn new(instructions: Arc<Vec<Instruction>>) -> Self {
        InstructionHandler { instructions: instructions }
    }
}

impl Handler for InstructionHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        debug!("get instruction request received!");
        let resp_body = serde_json::to_string(&self.instructions).unwrap();
        let resp = response_with((status::Ok, resp_body));
        Ok(resp)
    }
}

#[derive(Serialize)]
struct ContinueResponse {
    continued: bool,
}


#[derive(Serialize)]
pub struct StepResponse {
    stepped: bool,
}


pub struct StepHandler {
    cpu_thread_handle: Thread,
}

impl StepHandler {
    pub fn new(cpu_thread: Thread) -> Self {
        StepHandler { cpu_thread_handle: cpu_thread }
    }
}

impl Handler for StepHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        debug!("Step request received!");
        self.cpu_thread_handle.unpark();
        let resp = StepResponse { stepped: true };
        let json = serde_json::to_string(&resp).unwrap();
        Ok(response_with((status::Ok, json)))
    }
}

pub struct ContinueHandler {
    cpu_thread_handle: Thread,
    cpu_paused: Arc<AtomicBool>,
}

impl ContinueHandler {
    pub fn new(cpu_thread: Thread, is_stepping: Arc<AtomicBool>) -> Self {
        ContinueHandler {
            cpu_thread_handle: cpu_thread,
            cpu_paused: is_stepping,
        }
    }
}

impl Handler for ContinueHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        debug!("Continue request received!");
        self.cpu_paused.compare_and_swap(true, false, Ordering::Relaxed);
        self.cpu_thread_handle.unpark();
        let resp = ContinueResponse { continued: true };
        let json = serde_json::to_string(&resp).unwrap();
        Ok(response_with((status::Ok, json)))
    }
}

fn get_router<'a>(req: &'a Request) -> &'a Params {
    req.extensions.get::<Router>().unwrap()
}

fn response_with<M: Modifier<Response>>(m: M) -> Response {
    let mut resp = Response::with(m);
    resp.headers.set(headers::AccessControlAllowOrigin::Any);
    resp.headers.set(headers::ContentType::json());
    resp
}
