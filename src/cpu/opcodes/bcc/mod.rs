#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use memory::Memory;
use super::addressing::AddressingMode;
use super::branch_base::branch;
use super::OpCode;

pub struct Bcc;

impl OpCode for Bcc {
    type Input = i8;

    fn execute<M, AM, F>(cpu: &mut Cpu<M>, am: AM, tick_handler: &F)
        where M: Memory,
              AM: AddressingMode<M, Output = Self::Input>,
              F: Fn(&Cpu<M>)
    {
        let carry_clear = !cpu.registers.carry_flag();
        branch(cpu, am, tick_handler, carry_clear);
    }
}
