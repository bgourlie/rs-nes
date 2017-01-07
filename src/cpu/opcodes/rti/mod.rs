#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use memory::Memory;
use super::addressing::AddressingMode;
use super::OpCode;

pub struct Rti;

impl OpCode for Rti {
    type Input = ();

    fn execute<M, AM, F>(cpu: &mut Cpu<M>, _: AM, tick_handler: &F)
        where M: Memory,
              AM: AddressingMode<M, Output = Self::Input>,
              F: Fn(&Cpu<M>)
    {
        let stat = cpu.pop_stack(&tick_handler);
        let pc = cpu.pop_stack16(&tick_handler);
        cpu.registers.status = stat;
        cpu.registers.pc = pc;
    }
}
