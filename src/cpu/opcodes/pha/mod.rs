#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use cpu::opcodes::OpCode;
use cpu::opcodes::addressing::AddressingMode;
use errors::*;
use memory::Memory;

pub struct Pha;

impl OpCode for Pha {
    type Input = ();

    fn execute<M: Memory, AM: AddressingMode<M, Output = Self::Input>>(cpu: &mut Cpu<M>,
                                                                       _: AM)
                                                                       -> Result<()> {
        // Dummy read
        cpu.tick()?;

        let acc = cpu.registers.acc;
        cpu.push_stack(acc)
    }
}
