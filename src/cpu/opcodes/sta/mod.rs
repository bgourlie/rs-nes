#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use cpu::opcodes::OpCode;
use cpu::opcodes::addressing::AddressingMode;
use memory::Memory;

pub struct Sta;

impl OpCode for Sta {
    // TODO: STA doesn't actually have an input
    // Is there a compelling reason to have write-only addressing implementations?
    type Input = u8;

    fn execute<M: Memory, AM: AddressingMode<M, Output = Self::Input>>(cpu: &mut Cpu<M>, am: AM) {
        let acc = cpu.registers.acc;
        am.write(cpu, acc);
    }
}
