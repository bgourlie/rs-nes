#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use cpu::opcodes::AddressingMode;
use cpu::opcodes::OpCode;
use memory::Memory;
use screen::Screen;

pub struct Tsx;

impl OpCode for Tsx {
    type Input = ();

fn execute<S: Screen, M: Memory<S>, AM: AddressingMode<S, M, Output = Self::Input>>(cpu: &mut Cpu<S, M>, _: AM){
        cpu.registers.x = cpu.registers.sp;
        let x = cpu.registers.x;
        cpu.registers.set_sign_and_zero_flag(x);
        cpu.tick()
    }
}
