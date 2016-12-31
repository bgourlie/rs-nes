pub mod http_debugger;

use std::marker::PhantomData;
use cpu::Registers;
use memory::Memory;

pub trait Debugger<M: Memory> {
    fn on_step(&mut self, _: &M, _: &Registers, _: u64) {}
}

pub struct NoOpDebugger<M>(PhantomData<M>);

impl<M> NoOpDebugger<M> {
    pub fn new() -> Self {
        NoOpDebugger(PhantomData)
    }
}

impl<M: Memory> Debugger<M> for NoOpDebugger<M> {}

pub struct CliDebugger;

impl<M: Memory> Debugger<M> for CliDebugger {}
