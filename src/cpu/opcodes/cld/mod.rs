#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use cpu::opcodes::OpCode;
use cpu::opcodes::addressing::AddressingMode;
use errors::*;
use memory::Memory;

pub struct Cld;

impl OpCode for Cld {
    type Input = ();

    fn execute<M: Memory, AM: AddressingMode<M, Output = Self::Input>>(cpu: &mut Cpu<M>,
                                                                       _: AM)
                                                                       -> Result<()> {
        cpu.registers.set_decimal_flag(false);
        cpu.tick()
    }
}
