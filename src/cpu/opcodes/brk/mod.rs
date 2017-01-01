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
    fn execute<M: Memory, AM: AddressingMode<M>, F: Fn(&Cpu<M>)>(cpu: &mut Cpu<M>,
                                                                 _: AM,
                                                                 tick_handler: &F) {
        cpu.registers.pc += 1;
        tick_handler();
        let (pc_low_byte, pc_high_byte) = lo_hi(cpu.registers.pc);
        let status = cpu.registers.status;
        cpu.push_stack(pc_low_byte);
        tick_handler();
        cpu.push_stack(pc_high_byte);
        tick_handler();
        cpu.push_stack(status);
        tick_handler();
        let irq_handler_low = cpu.memory.load(BRK_VECTOR);
        tick_handler();
        let irq_handler_high = cpu.memory.load(BRK_VECTOR + 1);
        tick_handler();
        cpu.registers.pc = from_lo_hi(irq_handler_low, irq_handler_high);
        cpu.registers.set_interrupt_disable_flag(true);
    }
}
