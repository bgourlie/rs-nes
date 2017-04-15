#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use cpu::opcodes::OpCode;
use cpu::opcodes::addressing::AddressingMode;
use memory::Memory;
use screen::Screen;

pub struct Stx;

impl OpCode for Stx {
    // TODO: STX doesn't actually have an input
    // Is there a compelling reason to have write-only addressing implementations?
    type Input = u8;

fn execute<S: Screen, M: Memory<S>, AM: AddressingMode<S, M, Output = Self::Input>>(cpu: &mut Cpu<S, M>, am: AM){
        let x = cpu.registers.x;
        am.write(cpu, x)
    }
}
