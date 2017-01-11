#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use memory::Memory;
use super::addressing::AddressingMode;
use super::OpCode;

pub struct Sty;

impl OpCode for Sty {
    // TODO: STY doesn't actually have an input
    // Is there a compelling reason to have write-only addressing implementations?
    type Input = u8;

    fn execute<M, AM, F>(cpu: &mut Cpu<M>, am: AM, tick_handler: &F)
        where M: Memory,
              AM: AddressingMode<M, Output = Self::Input>,
              F: Fn(&Cpu<M>)
    {
        let y = cpu.registers.y;
        am.write(cpu, y, &tick_handler);
    }
}
