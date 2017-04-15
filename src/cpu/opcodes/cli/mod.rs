#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use cpu::opcodes::AddressingMode;
use cpu::opcodes::OpCode;
use memory::Memory;
use screen::Screen;

pub struct Cli;

impl OpCode for Cli {
    type Input = ();

fn execute<S: Screen, M: Memory<S>, AM: AddressingMode<S, M, Output = Self::Input>>(cpu: &mut Cpu<S, M>, _: AM){
        cpu.registers.set_interrupt_disable_flag(false);
        cpu.tick()
    }
}
