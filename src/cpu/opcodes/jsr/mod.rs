#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use cpu::opcodes::OpCode;
use cpu::opcodes::addressing::AddressingMode;
use memory::Memory;

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
        cpu.push_stack16(pc - 1, &tick_handler);
        cpu.registers.pc = loc;
        tick_handler(cpu);
    }
}
