#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use cpu::byte_utils::from_lo_hi;
use memory::Memory;
use super::addressing::AddressingMode;
use super::OpCode;

pub struct Rts;

impl OpCode for Rts {
    type Input = ();

    fn execute<M, AM, F>(cpu: &mut Cpu<M>, _: AM, _: &F)
        where M: Memory,
              AM: AddressingMode<M, Output = Self::Input>,
              F: Fn(&Cpu<M>)
    {
        let pc_low = cpu.pop_stack();
        let pc_high = cpu.pop_stack();
        let pc = from_lo_hi(pc_low, pc_high);
        cpu.registers.pc = pc + 1;
    }
}
