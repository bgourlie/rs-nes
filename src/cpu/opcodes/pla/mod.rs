#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use cpu::opcodes::OpCode;
use cpu::opcodes::addressing::AddressingMode;
use memory::Memory;

pub struct Pla;

impl OpCode for Pla {
    type Input = ();

    fn execute<M: Memory, AM: AddressingMode<M, Output = Self::Input>>(cpu: &mut Cpu<M>,
                                                                       _: AM)
                                                                       -> Result<(), ()> {
        // Dummy read
        cpu.tick()?;

        // Stack pointer inc cycle
        cpu.tick()?;

        let val = cpu.pop_stack()?;
        cpu.registers.set_acc(val);
        Ok(())
    }
}
