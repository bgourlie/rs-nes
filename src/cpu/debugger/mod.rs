pub mod http_debugger;

use cpu::Cpu;
use memory::Memory;
use std::marker::PhantomData;

pub trait Debugger<M: Memory> {
    fn on_step(&mut self, _: &Cpu<M>, _: u64) {}
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
