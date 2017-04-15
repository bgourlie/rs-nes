#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use cpu::opcodes::AddressingMode;
use cpu::opcodes::OpCode;
use memory::Memory;
use screen::Screen;

pub struct Rti;

impl OpCode for Rti {
    type Input = ();

fn execute<S: Screen, M: Memory<S>, AM: AddressingMode<S, M, Output = Self::Input>>(cpu: &mut Cpu<S, M>, _: AM){
        // Dummy read cycle
        cpu.tick();

        // Increment stack pointer cycle
        cpu.tick();

        let stat = cpu.pop_stack();
        let pc = cpu.pop_stack16();
        cpu.registers.set_status_from_stack(stat);
        cpu.registers.pc = pc;

    }
}
