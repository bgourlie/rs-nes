#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use memory::Memory;
use super::addressing::AddressingMode;
use super::OpCode;

pub struct Tya;

impl OpCode for Tya {
    type Input = ();

    fn execute<M, AM, F>(cpu: &mut Cpu<M>, _: AM, _: &F)
        where M: Memory,
              AM: AddressingMode<M, Output = Self::Input>,
              F: Fn(&Cpu<M>)
    {
        cpu.registers.acc = cpu.registers.y;
        let acc = cpu.registers.acc;
        cpu.registers.set_sign_and_zero_flag(acc);
    }
}
