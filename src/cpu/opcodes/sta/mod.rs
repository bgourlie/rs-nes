#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use memory::Memory;
use super::addressing::AddressingMode;
use super::OpCode;

pub struct Sta;

impl OpCode for Sta {
    type Input = u8;

    fn execute<M, AM, F>(cpu: &mut Cpu<M>, am: AM, _: &F)
        where M: Memory,
              AM: AddressingMode<M, Output = Self::Input>,
              F: Fn(&Cpu<M>)
    {
        let acc = cpu.registers.acc;
        am.write(cpu, acc);
    }
}
