#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use cpu::opcodes::OpCode;
use cpu::opcodes::addressing::AddressingMode;
use cpu::opcodes::arithmetic_base::adc_base;
use memory::Memory;
use screen::Screen;

pub struct Sbc;

impl OpCode for Sbc {
    type Input = u8;

fn execute<S: Screen, M: Memory<S>, AM: AddressingMode<S, M, Output = Self::Input>>(cpu: &mut Cpu<S, M>, am: AM){
        let lhs = cpu.registers.acc;
        let rhs = am.read();
        let rhs = !rhs;
        adc_base(cpu, lhs, rhs)
    }
}
