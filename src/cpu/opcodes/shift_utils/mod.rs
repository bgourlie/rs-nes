#[cfg(test)]
pub mod spec_tests;

use cpu::Cpu;
use memory::Memory;
use super::addressing_mode::AddressingMode;

pub fn shift_left<M: Memory, AM: AddressingMode<M>>(cpu: &mut Cpu<M>, am: AM, lsb: bool) {
    let val = am.operand();
    let carry = (val & 0x80) != 0;
    let res = if lsb { (val << 1) | 0x1 } else { val << 1 };
    cpu.registers.set_carry_flag(carry);
    cpu.registers.set_sign_and_zero_flag(res);
    am.write(cpu, res);
}
