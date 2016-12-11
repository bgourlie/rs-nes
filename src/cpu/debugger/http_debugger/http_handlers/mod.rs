use std::thread::Thread;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};

use iron::prelude::*;
use iron::{status, headers};
use iron::Handler;
use iron::modifier::Modifier;
use router::{Router, Params};
use serde::{Serialize, Serializer};
use serde_json;

use disassembler::Instruction;
use super::breakpoint_map::BreakpointMap;

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

struct ContinueResponse {
    continued: bool,
}

impl Serialize for ContinueResponse {
    fn serialize<S: Serializer>(&self, serializer: &mut S) -> Result<(), S::Error> {
        let mut state = serializer.serialize_struct("ContinueResponse", 1)?;
        serializer.serialize_struct_elt(&mut state, "continued", self.continued)?;
        serializer.serialize_struct_end(state)
    }
}

pub struct StepResponse {
    stepped: bool,
}

impl Serialize for StepResponse {
    fn serialize<S: Serializer>(&self, serializer: &mut S) -> Result<(), S::Error> {
        let mut state = serializer.serialize_struct("StepResponse", 1)?;
        serializer.serialize_struct_elt(&mut state, "stepped", self.stepped)?;
        serializer.serialize_struct_end(state)
    }
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
