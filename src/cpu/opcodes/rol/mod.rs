#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use cpu::opcodes::OpCode;
use cpu::opcodes::addressing::AddressingMode;
use cpu::opcodes::shift_base::shift_left;
use memory::Memory;

pub struct Rol;

impl OpCode for Rol {
    type Input = u8;

    fn execute<M: Memory, AM: AddressingMode<M, Output = Self::Input>>(cpu: &mut Cpu<M>,
                                                                       am: AM)
                                                                       -> Result<(), ()> {
        let carry_set = cpu.registers.carry_flag();
        shift_left(cpu, am, carry_set)
    }
}
