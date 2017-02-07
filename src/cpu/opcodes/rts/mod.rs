#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use cpu::opcodes::OpCode;
use cpu::opcodes::addressing::AddressingMode;
use errors::*;
use memory::Memory;

pub struct Rts;

impl OpCode for Rts {
    type Input = ();

    fn execute<M: Memory, AM: AddressingMode<M, Output = Self::Input>>(cpu: &mut Cpu<M>,
                                                                       _: AM)
                                                                       -> Result<()> {
        // Dummy read cycle
        cpu.tick()?;

        // Stack increment cycle
        cpu.tick()?;

        let pc = cpu.pop_stack16()?;
        cpu.registers.pc = pc + 1;

        // increment PC cycle
        cpu.tick()
    }
}
