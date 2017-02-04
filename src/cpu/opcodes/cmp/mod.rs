#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use cpu::opcodes::OpCode;
use cpu::opcodes::addressing::AddressingMode;
use cpu::opcodes::compare_base::compare;
use memory::Memory;

pub struct Cmp;

impl OpCode for Cmp {
    type Input = u8;

    fn execute<M: Memory, AM: AddressingMode<M, Output = Self::Input>>(cpu: &mut Cpu<M>, am: AM) {
        let val = cpu.registers.acc;
        compare(cpu, am, val);
    }
}
