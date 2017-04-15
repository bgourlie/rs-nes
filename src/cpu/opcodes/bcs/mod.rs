#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use cpu::opcodes::AddressingMode;
use cpu::opcodes::OpCode;
use cpu::opcodes::branch_base::branch;
use memory::Memory;
use screen::Screen;

pub struct Bcs;

impl OpCode for Bcs {
    type Input = i8;

fn execute<S: Screen, M: Memory<S>, AM: AddressingMode<S, M, Output = Self::Input>>(cpu: &mut Cpu<S, M>, am: AM){
        let carry_set = cpu.registers.carry_flag();
        branch(cpu, am, carry_set)
    }
}
