use cpu::Cpu;
use memory::Memory;
use super::addressing_mode::AddressingMode;

pub fn compare<M: Memory, AM: AddressingMode<M>>(cpu: &mut Cpu<M>, am: AM, lhs: u8) {
    let rhs = am.operand();
    let res = lhs as i32 - rhs as i32;
    cpu.registers.set_carry_flag(res & 0x100 == 0);
    cpu.registers.set_sign_and_zero_flag(res as u8);
}
