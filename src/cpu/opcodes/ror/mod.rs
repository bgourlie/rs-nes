#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use cpu::opcodes::OpCode;
use cpu::opcodes::addressing::AddressingMode;
use cpu::opcodes::shift_base::shift_right;
use errors::*;
use memory::Memory;

pub struct Ror;

impl OpCode for Ror {
    type Input = u8;

    fn execute<M: Memory, AM: AddressingMode<M, Output = Self::Input>>(cpu: &mut Cpu<M>,
                                                                       am: AM)
                                                                       -> Result<()> {
        let carry_set = cpu.registers.carry_flag();
        shift_right(cpu, am, carry_set)
    }
}
