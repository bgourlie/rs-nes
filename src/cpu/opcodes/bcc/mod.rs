#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use memory::Memory;
use super::addressing_mode::AddressingMode;
use super::branch_utils::branch;
use super::OpCode;

pub struct Bcc;

impl OpCode for Bcc {
    fn execute<M: Memory, AM: AddressingMode<M>>(cpu: &mut Cpu<M>, am: AM) {
        let carry_clear = !cpu.registers.carry_flag();
        branch(cpu, am, carry_clear);
    }
}
