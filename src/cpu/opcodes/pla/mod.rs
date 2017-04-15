#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use cpu::opcodes::AddressingMode;
use cpu::opcodes::OpCode;
use memory::Memory;
use screen::Screen;

pub struct Pla;

impl OpCode for Pla {
    type Input = ();

fn execute<S: Screen, M: Memory<S>, AM: AddressingMode<S, M, Output = Self::Input>>(cpu: &mut Cpu<S, M>, _: AM){
        // Dummy read
        cpu.tick();

        // Stack pointer inc cycle
        cpu.tick();

        let val = cpu.pop_stack();
        cpu.registers.set_acc(val);
    }
}
