#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use cpu::opcodes::OpCode;
use cpu::opcodes::addressing::AddressingMode;
use cpu::opcodes::branch_base::branch;
use memory::Memory;
use screen::Screen;

pub struct Beq;

impl OpCode for Beq {
    type Input = i8;

fn execute<S: Screen, M: Memory<S>, AM: AddressingMode<S, M, Output = Self::Input>>(cpu: &mut Cpu<S, M>, am: AM){
        let zero_set = cpu.registers.zero_flag();
        branch(cpu, am, zero_set)
    }
}
