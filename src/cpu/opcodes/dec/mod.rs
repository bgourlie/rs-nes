#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use cpu::byte_utils::wrapping_dec;
use memory::Memory;
use super::addressing::AddressingMode;
use super::OpCode;

pub struct Dec;

impl OpCode for Dec {
    type Input = u8;

    fn execute<M, AM, F>(cpu: &mut Cpu<M>, am: AM, _: &F)
        where M: Memory,
              AM: AddressingMode<M, Output = Self::Input>,
              F: Fn(&Cpu<M>)
    {
        let val = wrapping_dec(am.read());
        am.write(cpu, val);
        cpu.registers.set_sign_and_zero_flag(val);
    }
}
