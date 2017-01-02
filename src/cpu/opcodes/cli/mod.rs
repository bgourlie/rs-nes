#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use memory::Memory;
use super::addressing_mode::AddressingMode;
use super::OpCode;

pub struct Cli;

impl OpCode for Cli {
    fn execute<M: Memory, AM: AddressingMode<M>, F: Fn(&Cpu<M>)>(cpu: &mut Cpu<M>,
                                                                 _: AM,
                                                                 tick_handler: &F) {
        cpu.registers.set_interrupt_disable_flag(false);
        tick_handler(cpu)
    }
}
