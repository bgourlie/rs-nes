#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use memory::Memory;
use super::addressing_mode::AddressingMode;
use super::OpCode;

pub struct Sei;

impl OpCode for Sei {
    fn execute<M: Memory, AM: AddressingMode<M>, F: Fn(&Cpu<M>)>(cpu: &mut Cpu<M>,
                                                                 _: AM,
                                                                 tick_handler: &F) {
        cpu.registers.set_interrupt_disable_flag(true);
        tick_handler(cpu)
    }
}
