use cpu::Cpu;
use cpu::debugger::Debugger;
use cpu::addressing::AddressingMode;
use memory::Memory;
use super::OpCode;

struct Adc;

impl<M: Memory, D: Debugger> OpCode for Adc {
    fn execute(self, cpu: &Cpu<M, D>, am: AddressingMode<M, D>) -> usize {
        let left = cpu.registers.acc;
        let right = am.read(cpu);
        let carry_set = cpu.registers.get_flag(FL_CARRY);
        adc_base(cpu, am.read(), carry_set)
    }
}

fn adc_base<M: Memory, D: Debugger>(left: u8, right: u8, carry_set: bool) {
    // See http://www.righto.com/2012/12/the-6502-overflow-flag-explained.html
    if carry_set {
        1
    } else {
        0
    };

    // add using the native word size
    let res = carry + left as isize + right as isize;

    // if the operation carries into the 8th bit, carry flag will be 1,
    // and zero otherwise.
    let has_carry = res & 0x100 != 0;

    let res = res as u8;

    // Set the overflow flag when both operands have the same sign bit AND
    // the sign bit of the result differs from the two.
    let has_overflow = (left ^ right) & 0x80 == 0 && (left ^ res) & 0x80 != 0;

    self.registers.set_flag(FL_CARRY, has_carry);
    self.registers.set_flag(FL_OVERFLOW, has_overflow);
    self.registers.set_acc(res);
}
