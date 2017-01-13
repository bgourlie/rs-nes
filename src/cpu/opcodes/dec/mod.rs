#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use cpu::byte_utils::wrapping_dec;
use cpu::opcodes::OpCode;
use cpu::opcodes::addressing::AddressingMode;
use memory::Memory;

pub struct Dec;

impl OpCode for Dec {
    type Input = u8;

    fn execute<M, AM, F>(cpu: &mut Cpu<M>, am: AM, tick_handler: &F)
        where M: Memory,
              AM: AddressingMode<M, Output = Self::Input>,
              F: Fn(&Cpu<M>)
    {
        let val = wrapping_dec(am.read());
        am.write(cpu, val, &tick_handler);
        cpu.registers.set_sign_and_zero_flag(val);
    }
}
