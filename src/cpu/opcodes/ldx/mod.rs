#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use cpu::opcodes::AddressingMode;
use cpu::opcodes::OpCode;
use memory::Memory;
use screen::Screen;

pub struct Ldx;

impl OpCode for Ldx {
    type Input = u8;

fn execute<S: Screen, M: Memory<S>, AM: AddressingMode<S, M, Output = Self::Input>>(cpu: &mut Cpu<S, M>, am: AM){
        let val = am.read();
        cpu.registers.x = val;
        cpu.registers.set_sign_and_zero_flag(val);
    }
}
