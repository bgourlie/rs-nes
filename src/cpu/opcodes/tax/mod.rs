#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use cpu::opcodes::OpCode;
use cpu::opcodes::addressing::AddressingMode;
use memory::Memory;
use screen::Screen;

pub struct Tax;

impl OpCode for Tax {
    type Input = ();

fn execute<S: Screen, M: Memory<S>, AM: AddressingMode<S, M, Output = Self::Input>>(cpu: &mut Cpu<S, M>, _: AM){
        cpu.registers.x = cpu.registers.acc;
        let x = cpu.registers.x;
        cpu.registers.set_sign_and_zero_flag(x);
        cpu.tick()
    }
}
