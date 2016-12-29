use cpu::*;
use cpu::opcodes::Instruction;
use super::Adc;

/// ## Sign and zero flag tests
///
/// These tests check the presence of the sign and zero flag.
#[test]
fn adc_flags_sign_and_zero_1() {
    let mut cpu = TestCpu::new_test();

    // operands
    const LOP: u8 = 0x0;
    const ROP: u8 = 0x0;

    cpu.lda(LOP);
    let adc = Adc::new(ROP);
    adc.execute(&mut cpu);
    assert_eq!(true, cpu.registers.zero_flag());
    assert_eq!(false, cpu.registers.sign_flag());
    assert_eq!(0x0, cpu.registers.acc);
}

#[test]
fn adc_flags_sign_and_zero_2() {
    let mut cpu = TestCpu::new_test();

    // operands
    const LOP: u8 = 0x0;
    const ROP: u8 = 0x1;

    cpu.lda(LOP);
    let adc = Adc::new(ROP);
    adc.execute(&mut cpu);
    assert_eq!(false, cpu.registers.zero_flag());
    assert_eq!(false, cpu.registers.sign_flag());
    assert_eq!(0x1, cpu.registers.acc);
}

#[test]
fn adc_flags_sign_and_zero_3() {
    let mut cpu = TestCpu::new_test();

    // operands
    const LOP: u8 = 0x0;
    const ROP: u8 = 0xff;

    cpu.lda(LOP);
    let adc = Adc::new(ROP);
    adc.execute(&mut cpu);
    assert_eq!(false, cpu.registers.zero_flag());
    assert_eq!(true, cpu.registers.sign_flag());
    assert_eq!(0xff, cpu.registers.acc);
}

/// ## Carry and overflow flag tests
///
/// The following tests check all permutations of the
/// 6th and 7th bits of both operands, asserting that
/// the overflow and carry bit is set appROPriately.
///
/// A truth table for these tests can be found here:
/// http://www.righto.com/2012/12/the-6502-overflow-flag-explained.html
#[test]
fn adc_flags_carry_and_overflow_1() {
    let mut cpu = TestCpu::new_test();

    // operands
    const LOP: u8 = 0x50;
    const ROP: u8 = 0x10;

    cpu.lda(LOP);
    let adc = Adc::new(ROP);
    adc.execute(&mut cpu);
    assert_eq!(false, cpu.registers.carry_flag());
    assert_eq!(false, cpu.registers.overflow_flag());
    assert_eq!(0x60, cpu.registers.acc);
}

#[test]
fn adc_flags_carry_and_overflow_2() {
    let mut cpu = TestCpu::new_test();

    // operands
    const LOP: u8 = 0x50;
    const ROP: u8 = 0x50;

    cpu.lda(LOP);
    let adc = Adc::new(ROP);
    adc.execute(&mut cpu);
    assert_eq!(false, cpu.registers.carry_flag());
    assert_eq!(true, cpu.registers.overflow_flag());
    assert_eq!(0xa0, cpu.registers.acc);
}

#[test]
fn adc_flags_carry_and_overflow_3() {
    let mut cpu = TestCpu::new_test();

    // operands
    const LOP: u8 = 0x50;
    const ROP: u8 = 0x90;

    cpu.lda(LOP);
    let adc = Adc::new(ROP);
    adc.execute(&mut cpu);
    assert_eq!(false, cpu.registers.carry_flag());
    assert_eq!(false, cpu.registers.overflow_flag());
    assert_eq!(0xe0, cpu.registers.acc);
}

#[test]
fn adc_flags_carry_and_overflow_4() {
    let mut cpu = TestCpu::new_test();

    // operands
    const LOP: u8 = 0x50;
    const ROP: u8 = 0xd0;

    cpu.lda(LOP);
    let adc = Adc::new(ROP);
    adc.execute(&mut cpu);
    assert_eq!(true, cpu.registers.carry_flag());
    assert_eq!(false, cpu.registers.overflow_flag());
    assert_eq!(0x20, cpu.registers.acc);
}

#[test]
fn adc_flags_carry_and_overflow_5() {
    let mut cpu = TestCpu::new_test();

    // operands
    const LOP: u8 = 0xd0;
    const ROP: u8 = 0x10;

    cpu.lda(LOP);
    let adc = Adc::new(ROP);
    adc.execute(&mut cpu);
    assert_eq!(false, cpu.registers.carry_flag());
    assert_eq!(false, cpu.registers.overflow_flag());
    assert_eq!(0xe0, cpu.registers.acc);
}

#[test]
fn adc_flags_carry_and_overflow_6() {
    let mut cpu = TestCpu::new_test();

    // operands
    const LOP: u8 = 0xd0;
    const ROP: u8 = 0x50;

    cpu.lda(LOP);
    let adc = Adc::new(ROP);
    adc.execute(&mut cpu);
    assert_eq!(true, cpu.registers.carry_flag());
    assert_eq!(false, cpu.registers.overflow_flag());
    assert_eq!(0x20, cpu.registers.acc);
}

#[test]
fn adc_flags_carry_and_overflow_7() {
    let mut cpu = TestCpu::new_test();

    // operands
    const LOP: u8 = 0xd0;
    const ROP: u8 = 0x90;

    cpu.lda(LOP);
    let adc = Adc::new(ROP);
    adc.execute(&mut cpu);
    assert_eq!(true, cpu.registers.carry_flag());
    assert_eq!(true, cpu.registers.overflow_flag());
    assert_eq!(0x60, cpu.registers.acc);
}

#[test]
fn adc_flags_carry_and_overflow_8() {
    let mut cpu = TestCpu::new_test();

    // operands
    const LOP: u8 = 0xd0;
    const ROP: u8 = 0xd0;

    cpu.lda(LOP);
    let adc = Adc::new(ROP);
    adc.execute(&mut cpu);
    assert_eq!(true, cpu.registers.carry_flag());
    assert_eq!(false, cpu.registers.overflow_flag());
    assert_eq!(0xa0, cpu.registers.acc);
}
