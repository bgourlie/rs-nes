#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use cpu::byte_utils::wrapping_inc;
use cpu::opcodes::OpCode;
use cpu::opcodes::addressing::AddressingMode;
use memory::Memory;

pub struct Inx;

impl OpCode for Inx {
    type Input = ();

    fn execute<M, AM, F>(cpu: &mut Cpu<M>, _: AM, _: &F)
        where M: Memory,
              AM: AddressingMode<M, Output = Self::Input>,
              F: Fn(&Cpu<M>)
    {
        let val = wrapping_inc(cpu.registers.x);
        cpu.registers.x = val;
        cpu.registers.set_sign_and_zero_flag(val);
    }
}
