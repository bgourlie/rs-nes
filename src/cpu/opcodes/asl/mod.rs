#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use memory::Memory;
use super::addressing_mode::AddressingMode;
use super::shift_utils::shift_left;
use super::OpCode;

pub struct Asl;

impl OpCode for Asl {
    fn execute<M: Memory, AM: AddressingMode<M>, F: Fn(&Cpu<M>)>(cpu: &mut Cpu<M>, am: AM, _: &F) {
        shift_left(cpu, am, false)
    }
}
