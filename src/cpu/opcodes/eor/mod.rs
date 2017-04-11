#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use cpu::opcodes::OpCode;
use cpu::opcodes::addressing::AddressingMode;
use memory::Memory;

pub struct Eor;

impl OpCode for Eor {
    type Input = u8;

    fn execute<M: Memory, AM: AddressingMode<M, Output = Self::Input>>(cpu: &mut Cpu<M>, am: AM) {
        let rhs = am.read();
        let lhs = cpu.registers.acc;
        let res = lhs ^ rhs;
        cpu.registers.set_acc(res);
    }
}
