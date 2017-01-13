#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use cpu::opcodes::OpCode;
use cpu::opcodes::addressing::AddressingMode;
use memory::Memory;

pub struct Sec;

impl OpCode for Sec {
    type Input = ();

    fn execute<M, AM, F>(cpu: &mut Cpu<M>, _: AM, tick_handler: &F)
        where M: Memory,
              AM: AddressingMode<M, Output = Self::Input>,
              F: Fn(&Cpu<M>)
    {
        cpu.registers.set_carry_flag(true);
        tick_handler(cpu)
    }
}
