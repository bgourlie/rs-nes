#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use cpu::opcodes::OpCode;
use cpu::opcodes::addressing::AddressingMode;
use memory::Memory;

pub struct Txa;

impl OpCode for Txa {
    type Input = ();

    fn execute<M, AM, F>(cpu: &mut Cpu<M>, _: AM, tick_handler: &F)
        where M: Memory,
              AM: AddressingMode<M, Output = Self::Input>,
              F: Fn(&Cpu<M>)
    {
        cpu.registers.acc = cpu.registers.x;
        let acc = cpu.registers.acc;
        cpu.registers.set_sign_and_zero_flag(acc);
        tick_handler(cpu)
    }
}
