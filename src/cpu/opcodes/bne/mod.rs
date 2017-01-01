#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use memory::Memory;
use super::addressing_mode::AddressingMode;
use super::branch_utils::branch;
use super::OpCode;

pub struct Bne;

impl OpCode for Bne {
    fn execute<M: Memory, AM: AddressingMode<M>, F: Fn(&Cpu<M>)>(cpu: &mut Cpu<M>,
                                                                 am: AM,
                                                                 tick_handler: &F) {
        let zero_clear = !cpu.registers.zero_flag();
        branch(cpu, am, tick_handler, zero_clear);
    }
}
