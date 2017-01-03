#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use memory::Memory;
use super::addressing::AddressingMode;
use super::shift_base::shift_right;
use super::OpCode;

pub struct Ror;

impl OpCode for Ror {
    type Input = u8;

    fn execute<M, AM, F>(cpu: &mut Cpu<M>, am: AM, _: &F)
        where M: Memory,
              AM: AddressingMode<M, Output = Self::Input>,
              F: Fn(&Cpu<M>)
    {
        let carry_set = cpu.registers.carry_flag();
        shift_right(cpu, am, carry_set)
    }
}
