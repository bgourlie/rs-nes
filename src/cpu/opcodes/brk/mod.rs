#[cfg(test)]
mod spec_tests;

use cpu::Cpu;
use cpu::opcodes::OpCode;
use cpu::opcodes::addressing::AddressingMode;
use memory::Memory;

const BRK_VECTOR: u16 = 0xfffe;

pub struct Brk;

impl OpCode for Brk {
    type Input = ();

    fn execute<M: Memory, AM: AddressingMode<M, Output = Self::Input>>(cpu: &mut Cpu<M>, _: AM) {
        cpu.registers.pc += 1;
        let pc = cpu.registers.pc;
        let status = cpu.registers.status;
        cpu.push_stack16(pc);
        cpu.push_stack(status);
        let irq_handler = cpu.read_memory16(BRK_VECTOR);
        cpu.registers.pc = irq_handler;
        cpu.registers.set_interrupt_disable_flag(true);
    }
}
