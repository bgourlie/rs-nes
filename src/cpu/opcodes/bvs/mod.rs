#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use cpu::opcodes::OpCode;
use cpu::opcodes::addressing::AddressingMode;
use cpu::opcodes::branch_base::branch;
use errors::*;
use memory::Memory;

pub struct Bvs;

impl OpCode for Bvs {
    type Input = i8;

    fn execute<M: Memory, AM: AddressingMode<M, Output = Self::Input>>(cpu: &mut Cpu<M>,
                                                                       am: AM)
                                                                       -> Result<()> {
        let sign_clear = cpu.registers.overflow_flag();
        branch(cpu, am, sign_clear)
    }
}
