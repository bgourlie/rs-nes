use cpu::Cpu;
use memory::Memory;

pub fn adc_base<M: Memory>(cpu: &mut Cpu<M>, lhs: u8, rhs: u8) {

    if cpu.registers.decimal_flag() {
        panic!("Attempted decimal mode arithmetic");
    } else {
        // See http://www.righto.com/2012/12/the-6502-overflow-flag-explained.html
        let carry = if cpu.registers.carry_flag() { 1 } else { 0 };

        // add using the native word size
        let res = carry + lhs as isize + rhs as isize;

        // if the operation carries into the 8th bit, carry flag will be 1,
        // and zero otherwise.
        let has_carry = res & 0x100 != 0;

        let res = res as u8;

        // Set the overflow flag when both operands have the same sign bit AND
        // the sign bit of the result differs from the two.
        let has_overflow = (lhs ^ rhs) & 0x80 == 0 && (lhs ^ res) & 0x80 != 0;

        cpu.registers.set_carry_flag(has_carry);
        cpu.registers.set_overflow_flag(has_overflow);
        cpu.registers.set_acc(res);
    }
}
