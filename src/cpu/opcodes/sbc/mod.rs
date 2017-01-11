#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use memory::Memory;
use super::addressing::AddressingMode;
use super::arithmetic_base::adc_base;
use super::OpCode;

pub struct Sbc;

impl OpCode for Sbc {
    type Input = u8;

    fn execute<M, AM, F>(cpu: &mut Cpu<M>, am: AM, _: &F)
        where M: Memory,
              AM: AddressingMode<M, Output = Self::Input>,
              F: Fn(&Cpu<M>)
    {
        let lhs = cpu.registers.acc;
        let rhs = am.read();
        let rhs = !rhs;
        adc_base(cpu, lhs, rhs);
    }
}
