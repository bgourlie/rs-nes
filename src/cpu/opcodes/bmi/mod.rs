#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use memory::Memory;
use super::addressing_mode::AddressingMode;
use super::branch_utils::branch;
use super::OpCode;

pub struct Bmi;

impl OpCode for Bmi {
    fn execute<M: Memory, AM: AddressingMode<M>, F: Fn(&Cpu<M>)>(cpu: &mut Cpu<M>,
                                                                 am: AM,
                                                                 tick_handler: &F) {
        let sign_set = cpu.registers.sign_flag();
        branch(cpu, am, tick_handler, sign_set);
    }
}
