#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use cpu::opcodes::AddressingMode;
use cpu::opcodes::OpCode;
use cpu::opcodes::shift_base::shift_right;
use memory::Memory;
use screen::Screen;

pub struct Lsr;

impl OpCode for Lsr {
    type Input = u8;

fn execute<S: Screen, M: Memory<S>, AM: AddressingMode<S, M, Output = Self::Input>>(cpu: &mut Cpu<S, M>, am: AM){
        shift_right(cpu, am, false)
    }
}
