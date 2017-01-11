#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use memory::Memory;
use super::addressing::AddressingMode;
use super::OpCode;

pub struct Rts;

impl OpCode for Rts {
    type Input = ();

    fn execute<M, AM, F>(cpu: &mut Cpu<M>, _: AM, tick_handler: &F)
        where M: Memory,
              AM: AddressingMode<M, Output = Self::Input>,
              F: Fn(&Cpu<M>)
    {
        let pc = cpu.pop_stack16(&tick_handler);
        cpu.registers.pc = pc + 1;
    }
}
