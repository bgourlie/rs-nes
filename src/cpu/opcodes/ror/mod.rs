#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use cpu::opcodes::OpCode;
use cpu::opcodes::addressing::AddressingMode;
use cpu::opcodes::shift_base::shift_right;
use memory::Memory;

pub struct Ror;

impl OpCode for Ror {
    type Input = u8;

    fn execute<M, AM, F>(cpu: &mut Cpu<M>, am: AM, tick_handler: &F)
        where M: Memory,
              AM: AddressingMode<M, Output = Self::Input>,
              F: Fn(&Cpu<M>)
    {
        let carry_set = cpu.registers.carry_flag();
        shift_right(cpu, am, carry_set, &tick_handler)
    }
}
