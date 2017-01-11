#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use memory::Memory;
use super::addressing::AddressingMode;
use super::OpCode;

pub struct Bit;

impl OpCode for Bit {
    type Input = u8;

    fn execute<M, AM, F>(cpu: &mut Cpu<M>, am: AM, _: &F)
        where M: Memory,
              AM: AddressingMode<M, Output = Self::Input>,
              F: Fn(&Cpu<M>)
    {
        let lhs = cpu.registers.acc;
        let rhs = am.read();
        let res = lhs & rhs;

        cpu.registers.set_zero_flag(res == 0);
        cpu.registers.set_overflow_flag(rhs & 0x40 != 0);
        cpu.registers.set_sign_flag(rhs & 0x80 != 0);
    }
}
