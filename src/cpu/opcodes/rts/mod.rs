#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use cpu::opcodes::AddressingMode;
use cpu::opcodes::OpCode;
use memory::Memory;
use screen::Screen;

pub struct Rts;

impl OpCode for Rts {
    type Input = ();

fn execute<S: Screen, M: Memory<S>, AM: AddressingMode<S, M, Output = Self::Input>>(cpu: &mut Cpu<S, M>, _: AM){
        // Dummy read cycle
        cpu.tick();

        // Stack increment cycle
        cpu.tick();

        let pc = cpu.pop_stack16();
        cpu.registers.pc = pc + 1;

        // increment PC cycle
        cpu.tick()
    }
}
