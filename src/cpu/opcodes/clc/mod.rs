#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use memory::Memory;
use super::addressing_mode::AddressingMode;
use super::OpCode;

pub struct Clc;

impl OpCode for Clc {
    fn execute<M: Memory, AM: AddressingMode<M>, F: Fn(&Cpu<M>)>(cpu: &mut Cpu<M>,
                                                                 _: AM,
                                                                 tick_handler: &F) {
        cpu.registers.set_carry_flag(false);
        tick_handler(cpu)
    }
}
