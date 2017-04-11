#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use cpu::opcodes::OpCode;
use cpu::opcodes::addressing::AddressingMode;
use memory::Memory;

pub struct Rti;

impl OpCode for Rti {
    type Input = ();

    fn execute<M: Memory, AM: AddressingMode<M, Output = Self::Input>>(cpu: &mut Cpu<M>, _: AM) {
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
