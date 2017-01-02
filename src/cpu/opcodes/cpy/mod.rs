#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use memory::Memory;
use super::compare_utils::compare;
use super::addressing_mode::AddressingMode;
use super::OpCode;

pub struct Cpy;

impl OpCode for Cpy {
    fn execute<M: Memory, AM: AddressingMode<M>, F: Fn(&Cpu<M>)>(cpu: &mut Cpu<M>, am: AM, _: &F) {
        let val = cpu.registers.y;
        compare(cpu, am, val);
    }
}
