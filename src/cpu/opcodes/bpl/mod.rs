#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use memory::Memory;
use super::addressing_mode::AddressingMode;
use super::branch_base::branch;
use super::OpCode;

pub struct Bpl;

impl OpCode for Bpl {
    fn execute<M: Memory, AM: AddressingMode<M>, F: Fn(&Cpu<M>)>(cpu: &mut Cpu<M>,
                                                                 am: AM,
                                                                 tick_handler: &F) {
        let sign_clear = !cpu.registers.sign_flag();
        branch(cpu, am, tick_handler, sign_clear);
    }
}
