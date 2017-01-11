#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use memory::Memory;
use super::addressing::AddressingMode;
use super::OpCode;

pub struct Tsx;

impl OpCode for Tsx {
    type Input = ();

    fn execute<M, AM, F>(cpu: &mut Cpu<M>, _: AM, _: &F)
        where M: Memory,
              AM: AddressingMode<M, Output = Self::Input>,
              F: Fn(&Cpu<M>)
    {
        cpu.registers.x = cpu.registers.sp;
        let x = cpu.registers.x;
        cpu.registers.set_sign_and_zero_flag(x);
    }
}
