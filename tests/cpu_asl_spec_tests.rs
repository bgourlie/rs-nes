extern crate rs_nes;

use rs_nes::cpu::*;

/// ## Carry flag tests
///
/// The 7th bit should be stored in the carry flag

#[test]
fn asl_flags_carry_1() {
  let mut cpu = Cpu6502::new();

  cpu.lda(0x80);
  cpu.asl();

  assert_eq!(true, cpu.registers.get_flag(FL_CARRY));
}

#[test]
fn asl_flags_carry_2() {
  let mut cpu = Cpu6502::new();

  cpu.lda(0x70);
  cpu.asl();

  assert_eq!(false, cpu.registers.get_flag(FL_CARRY));
}

/// ## Sign and zero flags

#[test]
fn asl_flags_sign_and_zero_1() {
  let mut cpu = Cpu6502::new();

  cpu.lda(0x80);
  cpu.asl();

  assert_eq!(false, cpu.registers.get_flag(FL_SIGN));
  assert_eq!(true, cpu.registers.get_flag(FL_ZERO));
}

#[test]
fn asl_flags_sign_and_zero_2() {
  let mut cpu = Cpu6502::new();

  cpu.lda(0x40);
  cpu.asl();

  assert_eq!(true, cpu.registers.get_flag(FL_SIGN));
  assert_eq!(false, cpu.registers.get_flag(FL_ZERO));
}
