#[cfg(test)]
mod spec_tests;

use byte_utils::wrapping_dec;
use cpu::Cpu;
use cpu::opcodes::OpCode;
use cpu::opcodes::addressing::AddressingMode;
use memory::Memory;
use screen::Screen;

pub struct Dey;

impl OpCode for Dey {
    type Input = ();

fn execute<S: Screen, M: Memory<S>, AM: AddressingMode<S, M, Output = Self::Input>>(cpu: &mut Cpu<S, M>, _: AM){
        let val = wrapping_dec(cpu.registers.y);
        cpu.registers.y = val;
        cpu.registers.set_sign_and_zero_flag(val);
        cpu.tick()
    }
}
