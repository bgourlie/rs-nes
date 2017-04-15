#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use cpu::opcodes::AddressingMode;
use cpu::opcodes::OpCode;
use memory::Memory;
use screen::Screen;

pub struct Sty;

impl OpCode for Sty {
    // TODO: STY doesn't actually have an input
    // Is there a compelling reason to have write-only addressing implementations?
    type Input = u8;

fn execute<S: Screen, M: Memory<S>, AM: AddressingMode<S, M, Output = Self::Input>>(cpu: &mut Cpu<S, M>, am: AM){
        let y = cpu.registers.y;
        am.write(cpu, y)
    }
}
