#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use cpu::opcodes::OpCode;
use cpu::opcodes::addressing::AddressingMode;
use memory::Memory;

pub struct Jsr;

impl OpCode for Jsr {
    type Input = u16;

    fn execute<M: Memory, AM: AddressingMode<M, Output = Self::Input>>(cpu: &mut Cpu<M>, am: AM) {
        let loc = am.read();
        let pc = cpu.registers.pc;
        cpu.push_stack16(pc - 1);
        cpu.registers.pc = loc;
        cpu.tick()
    }
}
