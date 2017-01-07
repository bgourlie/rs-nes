#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use memory::Memory;
use super::addressing::AddressingMode;
use super::OpCode;

const BRK_VECTOR: u16 = 0xfffe;

pub struct Brk;

impl OpCode for Brk {
    type Input = ();

    fn execute<M, AM, F>(cpu: &mut Cpu<M>, _: AM, tick_handler: &F)
        where M: Memory,
              AM: AddressingMode<M, Output = Self::Input>,
              F: Fn(&Cpu<M>)
    {
        cpu.registers.pc += 1;
        let pc = cpu.registers.pc;
        let status = cpu.registers.status;
        cpu.push_stack16(pc, &tick_handler);
        cpu.push_stack(status, &tick_handler);
        let irq_handler = cpu.read_memory16(BRK_VECTOR, &tick_handler);
        cpu.registers.pc = irq_handler;
        cpu.registers.set_interrupt_disable_flag(true);
    }
}
