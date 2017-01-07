#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use cpu::byte_utils::from_lo_hi;
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
        let pc_low = cpu.pop_stack(&tick_handler);
        let pc_high = cpu.pop_stack(&tick_handler);
        let pc = from_lo_hi(pc_low, pc_high);
        cpu.registers.status = stat;
        cpu.registers.pc = pc;
    }
}
