#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use cpu::opcodes::OpCode;
use cpu::opcodes::addressing::AddressingMode;
use memory::Memory;

pub struct Php;

impl OpCode for Php {
    type Input = ();

    fn execute<M: Memory, AM: AddressingMode<M, Output = Self::Input>>(cpu: &mut Cpu<M>, _: AM) {
        // Dummy read
        cpu.tick();

        let stat = cpu.registers.status_for_stack();
        cpu.push_stack(stat);
    }
}
