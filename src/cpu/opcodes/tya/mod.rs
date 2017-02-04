#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use cpu::opcodes::OpCode;
use cpu::opcodes::addressing::AddressingMode;
use memory::Memory;

pub struct Tya;

impl OpCode for Tya {
    type Input = ();

    fn execute<M: Memory, AM: AddressingMode<M, Output = Self::Input>>(cpu: &mut Cpu<M>, _: AM) {
        cpu.registers.acc = cpu.registers.y;
        let acc = cpu.registers.acc;
        cpu.registers.set_sign_and_zero_flag(acc);
        cpu.tick()
    }
}
