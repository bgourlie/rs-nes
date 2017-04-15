#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use cpu::opcodes::AddressingMode;
use cpu::opcodes::OpCode;
use cpu::opcodes::compare_base::compare;
use memory::Memory;
use screen::Screen;

pub struct Cmp;

impl OpCode for Cmp {
    type Input = u8;

fn execute<S: Screen, M: Memory<S>, AM: AddressingMode<S, M, Output = Self::Input>>(cpu: &mut Cpu<S, M>, am: AM){
        let val = cpu.registers.acc;
        compare(cpu, am, val);
    }
}
