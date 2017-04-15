#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use cpu::opcodes::OpCode;
use cpu::opcodes::addressing::AddressingMode;
use cpu::opcodes::shift_base::shift_left;
use memory::Memory;
use screen::Screen;

pub struct Rol;

impl OpCode for Rol {
    type Input = u8;

fn execute<S: Screen, M: Memory<S>, AM: AddressingMode<S, M, Output = Self::Input>>(cpu: &mut Cpu<S, M>, am: AM){
        let carry_set = cpu.registers.carry_flag();
        shift_left(cpu, am, carry_set)
    }
}
