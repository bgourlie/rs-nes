use cpu::*;
use cpu::opcodes::*;

/// ## Sign and zero flag tests
///
/// These tests check the presence of the sign and zero flag.
#[test]
fn flags_sign_and_zero_1() {
    let mut cpu = TestCpu::new_test();
    Lda::execute(&mut cpu, 0_u8);
    cpu.registers.set_carry_flag(true);
    Sbc::execute(&mut cpu, 0_u8);
    assert_eq!(true, cpu.registers.zero_flag());
    assert_eq!(false, cpu.registers.sign_flag());
    assert_eq!(0, cpu.registers.acc);
}

#[test]
fn flags_sign_and_zero_2() {
    let mut cpu = TestCpu::new_test();
    Lda::execute(&mut cpu, 1_u8);
    cpu.registers.set_carry_flag(true);
    Sbc::execute(&mut cpu, 1_u8);
    assert_eq!(0, cpu.registers.acc);
    assert_eq!(true, cpu.registers.zero_flag());
    assert_eq!(false, cpu.registers.sign_flag());
}

#[test]
fn flags_sign_and_zero_3() {
    let mut cpu = TestCpu::new_test();
    Lda::execute(&mut cpu, 0_u8);
    cpu.registers.set_carry_flag(true);
    Sbc::execute(&mut cpu, 1_u8);
    assert_eq!(false, cpu.registers.zero_flag());
    assert_eq!(true, cpu.registers.sign_flag());
    assert_eq!(255, cpu.registers.acc);
}

/// ## Carry and overflow flag tests
///
/// The following tests check all permutations of the
/// 6th and 7th bits of both operands, asserting that
/// the overflow and carry bit is set appropriately.
///
/// A truth table for these tests can be found here:
/// http://www.righto.com/2012/12/the-6502-overflow-flag-explained.html
// The inputs and outputs are taken from the truth table in the linked
// documentation, so I would expect them to pass.  However after fixing
// sbc to pass the functional tests these tests fail.  They passed when
// I inverted the carry bit prior to calling ADC, which caused the functional
// tests to fail, which I consider authoritative.
#[test]
fn flags_carry_and_overflow_1() {
    let mut cpu = TestCpu::new_test();
    Lda::execute(&mut cpu, 80_u8);
    cpu.registers.set_carry_flag(true);
    Sbc::execute(&mut cpu, 240_u8);
    assert_eq!(false, cpu.registers.carry_flag());
    assert_eq!(false, cpu.registers.overflow_flag());
    assert_eq!(96, cpu.registers.acc);
}

#[test]
fn flags_carry_and_overflow_2() {
    let mut cpu = TestCpu::new_test();
    Lda::execute(&mut cpu, 80_u8);
    cpu.registers.set_carry_flag(true);
    Sbc::execute(&mut cpu, 176_u8);
    assert_eq!(false, cpu.registers.carry_flag());
    assert_eq!(true, cpu.registers.overflow_flag());
    assert_eq!(160, cpu.registers.acc);
}

#[test]
fn flags_carry_and_overflow_3() {
    let mut cpu = TestCpu::new_test();
    Lda::execute(&mut cpu, 80_u8);
    cpu.registers.set_carry_flag(true);
    Sbc::execute(&mut cpu, 112_u8);
    assert_eq!(false, cpu.registers.carry_flag());
    assert_eq!(false, cpu.registers.overflow_flag());
    assert_eq!(224, cpu.registers.acc);
}

#[test]
fn flags_carry_and_overflow_4() {
    let mut cpu = TestCpu::new_test();
    Lda::execute(&mut cpu, 80_u8);
    cpu.registers.set_carry_flag(true);
    Sbc::execute(&mut cpu, 48_u8);
    assert_eq!(true, cpu.registers.carry_flag());
    assert_eq!(false, cpu.registers.overflow_flag());
    assert_eq!(32, cpu.registers.acc);
}

#[test]
fn flags_carry_and_overflow_5() {
    let mut cpu = TestCpu::new_test();
    Lda::execute(&mut cpu, 208_u8);
    cpu.registers.set_carry_flag(true);
    Sbc::execute(&mut cpu, 240_u8);
    assert_eq!(false, cpu.registers.carry_flag());
    assert_eq!(false, cpu.registers.overflow_flag());
    assert_eq!(-32, cpu.registers.acc as i8);
}

#[test]
fn flags_carry_and_overflow_6() {
    let mut cpu = TestCpu::new_test();
    Lda::execute(&mut cpu, 208_u8);
    cpu.registers.set_carry_flag(true);
    Sbc::execute(&mut cpu, 176_u8);
    assert_eq!(true, cpu.registers.carry_flag());
    assert_eq!(false, cpu.registers.overflow_flag());
    assert_eq!(32, cpu.registers.acc);
}

#[test]
fn flags_carry_and_overflow_7() {
    let mut cpu = TestCpu::new_test();
    Lda::execute(&mut cpu, 208_u8);
    cpu.registers.set_carry_flag(true);
    Sbc::execute(&mut cpu, 112_u8);
    assert_eq!(true, cpu.registers.carry_flag());
    assert_eq!(true, cpu.registers.overflow_flag());
    assert_eq!(96, cpu.registers.acc);
}

#[test]
fn flags_carry_and_overflow_8() {
    let mut cpu = TestCpu::new_test();
    Lda::execute(&mut cpu, 208_u8);
    cpu.registers.set_carry_flag(true);
    Sbc::execute(&mut cpu, 48_u8);
    assert_eq!(true, cpu.registers.carry_flag());
    assert_eq!(false, cpu.registers.overflow_flag());
    assert_eq!(160, cpu.registers.acc);
}
