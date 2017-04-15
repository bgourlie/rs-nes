#[cfg(test)]
mod spec_tests;

use byte_utils::wrapping_dec;
use cpu::Cpu;
use cpu::opcodes::OpCode;
use cpu::opcodes::addressing::AddressingMode;
use memory::Memory;
use screen::Screen;

pub struct Dex;

impl OpCode for Dex {
    type Input = ();

fn execute<S: Screen, M: Memory<S>, AM: AddressingMode<S, M, Output = Self::Input>>(cpu: &mut Cpu<S, M>, _: AM){
        let val = wrapping_dec(cpu.registers.x);
        cpu.registers.x = val;
        cpu.registers.set_sign_and_zero_flag(val);
        cpu.tick()
    }
}
