#[cfg(test)]
mod spec_tests;

use byte_utils::wrapping_inc;
use cpu::Cpu;
use cpu::opcodes::AddressingMode;
use cpu::opcodes::OpCode;
use memory::Memory;
use screen::Screen;

pub struct Inc;

impl OpCode for Inc {
    type Input = u8;

fn execute<S: Screen, M: Memory<S>, AM: AddressingMode<S, M, Output = Self::Input>>(cpu: &mut Cpu<S, M>, am: AM){
        let val = wrapping_inc(am.read());
        am.write(cpu, val);
        cpu.registers.set_sign_and_zero_flag(val);
    }
}
