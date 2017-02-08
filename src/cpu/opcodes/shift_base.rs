use cpu::Cpu;
use cpu::opcodes::addressing::AddressingMode;
use errors::*;
use memory::Memory;

pub fn shift_left<M: Memory, AM: AddressingMode<M, Output = u8>>(cpu: &mut Cpu<M>,
                                                                 am: AM,
                                                                 lsb: bool)
                                                                 -> Result<()> {
    let val = am.read();
    let carry = (val & 0x80) != 0;
    let res = if lsb { (val << 1) | 0x1 } else { val << 1 };
    cpu.registers.set_carry_flag(carry);
    cpu.registers.set_sign_and_zero_flag(res);
    am.write(cpu, res)
}

pub fn shift_right<M: Memory, AM: AddressingMode<M, Output = u8>>(cpu: &mut Cpu<M>,
                                                                  am: AM,
                                                                  msb: bool)
                                                                  -> Result<()> {
    let val = am.read();
    let carry = (val & 0x1) != 0;
    let res = if msb { (val >> 1) | 0x80 } else { val >> 1 };
    cpu.registers.set_carry_flag(carry);
    cpu.registers.set_sign_and_zero_flag(res);
    am.write(cpu, res)
}
