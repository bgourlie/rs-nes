#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use cpu::opcodes::OpCode;
use cpu::opcodes::addressing::AddressingMode;
use memory::Memory;
use screen::Screen;

pub struct Bit;

impl OpCode for Bit {
    type Input = u8;

fn execute<S: Screen, M: Memory<S>, AM: AddressingMode<S, M, Output = Self::Input>>(cpu: &mut Cpu<S, M>, am: AM){
        let lhs = cpu.registers.acc;
        let rhs = am.read();
        let res = lhs & rhs;

        cpu.registers.set_zero_flag(res == 0);
        cpu.registers.set_overflow_flag(rhs & 0x40 != 0);
        cpu.registers.set_sign_flag(rhs & 0x80 != 0);
    }
}
