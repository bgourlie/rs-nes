#[cfg(test)]
mod spec_tests;

use byte_utils::wrapping_dec;
use cpu::Cpu;
use cpu::opcodes::OpCode;
use cpu::opcodes::addressing::AddressingMode;
use memory::Memory;
use screen::Screen;

pub struct Dec;

impl OpCode for Dec {
    type Input = u8;

fn execute<S: Screen, M: Memory<S>, AM: AddressingMode<S, M, Output = Self::Input>>(cpu: &mut Cpu<S, M>, am: AM){
        let val = wrapping_dec(am.read());
        am.write(cpu, val);
        cpu.registers.set_sign_and_zero_flag(val);
    }
}
