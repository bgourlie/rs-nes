#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use memory::Memory;
use super::OpCode;
use super::addressing::AddressingMode;

pub struct Ldy;

impl OpCode for Ldy {
    type Input = u8;

    fn execute<M, AM, F>(cpu: &mut Cpu<M>, am: AM, _: &F)
        where M: Memory,
              AM: AddressingMode<M, Output = Self::Input>,
              F: Fn(&Cpu<M>)
    {
        let val = am.read();
        cpu.registers.y = val;
        cpu.registers.set_sign_and_zero_flag(val);
    }
}
