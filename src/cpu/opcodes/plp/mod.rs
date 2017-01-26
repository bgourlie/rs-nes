#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use cpu::opcodes::OpCode;
use cpu::opcodes::addressing::AddressingMode;
use memory::Memory;

pub struct Plp;

impl OpCode for Plp {
    type Input = ();

    fn execute<M, AM, F>(cpu: &mut Cpu<M>, _: AM, tick_handler: &F)
        where M: Memory,
              AM: AddressingMode<M, Output = Self::Input>,
              F: Fn(&Cpu<M>)
    {
        // Dummy read
        tick_handler(cpu);

        // Stack pointer inc cycle
        tick_handler(cpu);

        let val = cpu.pop_stack(&tick_handler);
        cpu.registers.set_status_from_stack(val);
    }
}
