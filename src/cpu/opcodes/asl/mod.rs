#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use memory::Memory;
use super::addressing_mode::AddressingMode;
use super::shift_utils::shift_left;
use super::OpCode;

pub struct Asl;

impl OpCode for Asl {
    fn execute<M: Memory, AM: AddressingMode<M>>(cpu: &mut Cpu<M>, am: AM) {
        shift_left(cpu, am, false)
    }
}
