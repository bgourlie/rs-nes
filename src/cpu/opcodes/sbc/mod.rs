#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use cpu::opcodes::OpCode;
use cpu::opcodes::addressing::AddressingMode;
use cpu::opcodes::arithmetic_base::adc_base;
use memory::Memory;

pub struct Sbc;

impl OpCode for Sbc {
    type Input = u8;

    fn execute<M: Memory, AM: AddressingMode<M, Output = Self::Input>>(cpu: &mut Cpu<M>,
                                                                       am: AM)
                                                                       -> Result<(), ()> {
        let lhs = cpu.registers.acc;
        let rhs = am.read();
        let rhs = !rhs;
        adc_base(cpu, lhs, rhs)
    }
}
