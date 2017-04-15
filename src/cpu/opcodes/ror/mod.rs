#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use cpu::opcodes::OpCode;
use cpu::opcodes::addressing::AddressingMode;
use cpu::opcodes::shift_base::shift_right;
use memory::Memory;
use screen::Screen;

pub struct Ror;

impl OpCode for Ror {
    type Input = u8;

fn execute<S: Screen, M: Memory<S>, AM: AddressingMode<S, M, Output = Self::Input>>(cpu: &mut Cpu<S, M>, am: AM){
        let carry_set = cpu.registers.carry_flag();
        shift_right(cpu, am, carry_set)
    }
}
