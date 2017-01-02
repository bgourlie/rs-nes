#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use memory::Memory;
use super::addressing_mode::AddressingMode;
use super::branch_base::branch;
use super::OpCode;

pub struct Beq;

impl OpCode for Beq {
    fn execute<M: Memory, AM: AddressingMode<M>, F: Fn(&Cpu<M>)>(cpu: &mut Cpu<M>,
                                                                 am: AM,
                                                                 tick_handler: &F) {
        let zero_set = cpu.registers.zero_flag();
        branch(cpu, am, tick_handler, zero_set);
    }
}
