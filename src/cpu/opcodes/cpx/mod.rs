#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use memory::Memory;
use super::compare_base::compare;
use super::addressing_mode::AddressingMode;
use super::OpCode;

pub struct Cpx;

impl OpCode for Cpx {
    fn execute<M: Memory, AM: AddressingMode<M>, F: Fn(&Cpu<M>)>(cpu: &mut Cpu<M>, am: AM, _: &F) {
        let val = cpu.registers.x;
        compare(cpu, am, val);
    }
}
