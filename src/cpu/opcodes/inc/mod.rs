#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use cpu::byte_utils::wrapping_inc;
use memory::Memory;
use super::addressing_mode::AddressingMode;
use super::OpCode;

pub struct Inc;

impl OpCode for Inc {
    fn execute<M: Memory, AM: AddressingMode<M>, F: Fn(&Cpu<M>)>(cpu: &mut Cpu<M>, am: AM, _: &F) {
        let val = wrapping_inc(am.operand());
        am.write(cpu, val);
        cpu.registers.set_sign_and_zero_flag(val);
    }
}
