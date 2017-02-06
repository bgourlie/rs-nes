#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use cpu::opcodes::OpCode;
use cpu::opcodes::addressing::AddressingMode;
use memory::Memory;

pub struct Tay;

impl OpCode for Tay {
    type Input = ();

    fn execute<M: Memory, AM: AddressingMode<M, Output = Self::Input>>(cpu: &mut Cpu<M>,
                                                                       _: AM)
                                                                       -> Result<(), ()> {
        cpu.registers.y = cpu.registers.acc;
        let y = cpu.registers.y;
        cpu.registers.set_sign_and_zero_flag(y);
        cpu.tick()
    }
}
