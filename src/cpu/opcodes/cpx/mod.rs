#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use cpu::opcodes::OpCode;
use cpu::opcodes::addressing::AddressingMode;
use cpu::opcodes::compare_base::compare;
use errors::*;
use memory::Memory;

pub struct Cpx;

impl OpCode for Cpx {
    type Input = u8;

    fn execute<M: Memory, AM: AddressingMode<M, Output = Self::Input>>(cpu: &mut Cpu<M>,
                                                                       am: AM)
                                                                       -> Result<()> {
        let val = cpu.registers.x;
        compare(cpu, am, val);
        Ok(())
    }
}
