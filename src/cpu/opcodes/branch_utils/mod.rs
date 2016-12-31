#[cfg(test)]
pub mod spec_tests;

use cpu::Cpu;
use memory::Memory;
use super::addressing_mode::AddressingMode;

pub fn branch<M: Memory, AM: AddressingMode<M>>(cpu: &mut Cpu<M>, am: AM, condition: bool) -> u8 {
    if condition {
        let rel_addr = am.operand() as i8;
        let old_pc = cpu.registers.pc;
        cpu.registers.pc = (cpu.registers.pc as i32 + rel_addr as i32) as u16;
        if cpu.registers.page_boundary_crossed(old_pc) {
            2
        } else {
            1
        }
    } else {
        0
    }
}
