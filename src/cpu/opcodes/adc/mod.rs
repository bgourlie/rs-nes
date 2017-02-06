#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use cpu::opcodes::OpCode;
use cpu::opcodes::addressing::AddressingMode;
use cpu::opcodes::arithmetic_base::adc_base;
use memory::Memory;

pub struct Adc;

impl OpCode for Adc {
    type Input = u8;

    fn execute<M: Memory, AM: AddressingMode<M, Output = Self::Input>>(cpu: &mut Cpu<M>,
                                                                       am: AM)
                                                                       -> Result<(), ()> {
        let left = cpu.registers.acc;
        let right = am.read();
        adc_base(cpu, left, right)?;
        Ok(())
    }
}
