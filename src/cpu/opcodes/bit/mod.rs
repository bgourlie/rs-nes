#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use memory::Memory;
use super::addressing_mode::AddressingMode;
use super::OpCode;

pub struct Bit;

impl OpCode for Bit {
    fn execute<M: Memory, AM: AddressingMode<M>, F: Fn(&Cpu<M>)>(cpu: &mut Cpu<M>, am: AM, _: &F) {
        let lhs = cpu.registers.acc;
        let rhs = am.operand();
        let res = lhs & rhs;

        cpu.registers.set_zero_flag(res == 0);
        cpu.registers.set_overflow_flag(rhs & 0x40 != 0);
        cpu.registers.set_sign_flag(rhs & 0x80 != 0);
    }
}
