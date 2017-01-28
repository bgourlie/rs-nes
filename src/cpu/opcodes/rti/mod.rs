#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use cpu::opcodes::OpCode;
use cpu::opcodes::addressing::AddressingMode;
use memory::Memory;

pub struct Rti;

impl OpCode for Rti {
    type Input = ();

    fn execute<M, AM, F>(cpu: &mut Cpu<M>, _: AM, tick_handler: &F)
        where M: Memory,
              AM: AddressingMode<M, Output = Self::Input>,
              F: Fn(&Cpu<M>)
    {
        // Dummy read cycle
        tick_handler(cpu);

        // Increment stack pointer cycle
        tick_handler(cpu);

        let stat = cpu.pop_stack(&tick_handler);
        let pc = cpu.pop_stack16(&tick_handler);
        cpu.registers.status = stat;
        cpu.registers.pc = pc;
    }
}
