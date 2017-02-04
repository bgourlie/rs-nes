#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use cpu::opcodes::OpCode;
use cpu::opcodes::addressing::AddressingMode;
use cpu::opcodes::branch_base::branch;
use memory::Memory;

pub struct Bvc;

impl OpCode for Bvc {
    type Input = i8;

    fn execute<M: Memory, AM: AddressingMode<M, Output = Self::Input>>(cpu: &mut Cpu<M>, am: AM) {
        let sign_clear = !cpu.registers.overflow_flag();
        branch(cpu, am, sign_clear);
    }
}
