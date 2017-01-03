#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use cpu::byte_utils::{lo_hi, from_lo_hi};
use memory::Memory;
use super::addressing_mode::AddressingMode;
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
        tick_handler(cpu);
        let (pc_low_byte, pc_high_byte) = lo_hi(cpu.registers.pc);
        let status = cpu.registers.status;
        cpu.push_stack(pc_low_byte);
        tick_handler(cpu);
        cpu.push_stack(pc_high_byte);
        tick_handler(cpu);
        cpu.push_stack(status);
        tick_handler(cpu);
        let irq_handler_low = cpu.memory.load(BRK_VECTOR);
        tick_handler(cpu);
        let irq_handler_high = cpu.memory.load(BRK_VECTOR + 1);
        tick_handler(cpu);
        cpu.registers.pc = from_lo_hi(irq_handler_low, irq_handler_high);
        cpu.registers.set_interrupt_disable_flag(true);
    }
}
