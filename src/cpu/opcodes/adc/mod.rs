#[cfg(test)]
mod spec_tests;

#[cfg(test)]
mod timing_tests;

use cpu::Cpu;
use memory::Memory;
use super::addressing::AddressingMode;
use super::arithmetic_base::adc_base;
use super::OpCode;

pub struct Adc;

impl OpCode for Adc {
    type Input = u8;

    fn execute<M, AM, F>(cpu: &mut Cpu<M>, am: AM, _: &F)
        where M: Memory,
              AM: AddressingMode<M, Output = Self::Input>,
              F: Fn(&Cpu<M>)
    {
        let left = cpu.registers.acc;
        let right = am.read();
        adc_base(cpu, left, right);
    }
}
