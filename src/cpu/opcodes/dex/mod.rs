#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use cpu::byte_utils::wrapping_dec;
use cpu::opcodes::OpCode;
use cpu::opcodes::addressing::AddressingMode;
use memory::Memory;

pub struct Dex;

impl OpCode for Dex {
    type Input = ();

    fn execute<M: Memory, AM: AddressingMode<M, Output = Self::Input>>(cpu: &mut Cpu<M>, _: AM) {
        let val = wrapping_dec(cpu.registers.x);
        cpu.registers.x = val;
        cpu.registers.set_sign_and_zero_flag(val);
        cpu.tick()
    }
}
