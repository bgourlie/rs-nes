#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use cpu::byte_utils::wrapping_dec;
use memory::Memory;
use super::addressing_mode::AddressingMode;
use super::OpCode;

pub struct Dec;

impl OpCode for Dec {
    fn execute<M: Memory, AM: AddressingMode<M>, F: Fn(&Cpu<M>)>(cpu: &mut Cpu<M>, am: AM, _: &F) {
        let val = wrapping_dec(am.operand());
        am.write(cpu, val);
        cpu.registers.set_sign_and_zero_flag(val);
    }
}
