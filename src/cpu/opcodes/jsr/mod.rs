#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use cpu::byte_utils::lo_hi;
use memory::Memory;
use super::addressing::AddressingMode;
use super::OpCode;

pub struct Jsr;

impl OpCode for Jsr {
    type Input = u16;

    fn execute<M, AM, F>(cpu: &mut Cpu<M>, am: AM, tick_handler: &F)
        where M: Memory,
              AM: AddressingMode<M, Output = Self::Input>,
              F: Fn(&Cpu<M>)
    {
        let loc = am.read();
        let pc = cpu.registers.pc;
        let (pc_low, pc_high) = lo_hi(pc - 1);
        cpu.push_stack(pc_low);
        tick_handler(cpu);
        cpu.push_stack(pc_high);
        tick_handler(cpu);
        cpu.registers.pc = loc;
        tick_handler(cpu);
    }
}
