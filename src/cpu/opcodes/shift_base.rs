use cpu::Cpu;
use cpu::opcodes::addressing::AddressingMode;
use memory::Memory;
use screen::Screen;

pub fn shift_left<S: Screen, M: Memory<S>, AM: AddressingMode<S, M, Output = u8>>(cpu: &mut Cpu<S, M>,
                                                                 am: AM,
lsb: bool){
    let val = am.read();
    let carry = (val & 0x80) != 0;
    let res = if lsb { (val << 1) | 0x1 } else { val << 1 };
    cpu.registers.set_carry_flag(carry);
    cpu.registers.set_sign_and_zero_flag(res);
    am.write(cpu, res)
}

pub fn shift_right<S: Screen, M: Memory<S>, AM: AddressingMode<S, M, Output = u8>>(cpu: &mut Cpu<S, M>,
                                                                  am: AM,
msb: bool){
    let val = am.read();
    let carry = (val & 0x1) != 0;
    let res = if msb { (val >> 1) | 0x80 } else { val >> 1 };
    cpu.registers.set_carry_flag(carry);
    cpu.registers.set_sign_and_zero_flag(res);
    am.write(cpu, res)
}
