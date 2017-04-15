#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use cpu::opcodes::AddressingMode;
use cpu::opcodes::OpCode;
use memory::Memory;
use screen::Screen;

pub struct Tay;

impl OpCode for Tay {
    type Input = ();

fn execute<S: Screen, M: Memory<S>, AM: AddressingMode<S, M, Output = Self::Input>>(cpu: &mut Cpu<S, M>, _: AM){
        cpu.registers.y = cpu.registers.acc;
        let y = cpu.registers.y;
        cpu.registers.set_sign_and_zero_flag(y);
        cpu.tick()
    }
}
