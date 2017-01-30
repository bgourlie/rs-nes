#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use cpu::opcodes::OpCode;
use cpu::opcodes::addressing::AddressingMode;
use memory::Memory;

pub struct Ldy;

impl OpCode for Ldy {
    type Input = u8;

    fn execute<M: Memory, AM: AddressingMode<M, Output = Self::Input>>(cpu: &mut Cpu<M>, am: AM) {
        let val = am.read();
        cpu.registers.y = val;
        cpu.registers.set_sign_and_zero_flag(val);
    }
}
