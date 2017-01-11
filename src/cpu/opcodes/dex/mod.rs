#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use cpu::byte_utils::wrapping_dec;
use memory::Memory;
use super::addressing::AddressingMode;
use super::OpCode;

pub struct Dex;

impl OpCode for Dex {
    type Input = ();

    fn execute<M, AM, F>(cpu: &mut Cpu<M>, _: AM, _: &F)
        where M: Memory,
              AM: AddressingMode<M, Output = Self::Input>,
              F: Fn(&Cpu<M>)
    {
        let val = wrapping_dec(cpu.registers.x);
        cpu.registers.x = val;
        cpu.registers.set_sign_and_zero_flag(val);
    }
}
