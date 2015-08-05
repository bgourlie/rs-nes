extern crate rs_nes;

use rs_nes::cpu::*;

/// ## Sign and zero flag tests
///
/// These tests check the presence of the sign and zero flag.

#[test]
fn sbc_flags_sign_and_zero_1() {
  let mut cpu = Cpu6502::new();

  // operands
  const LOP: u8 = 0x0;
  const ROP: u8 = 0x0;

  // decimal values sanity check
  assert_eq!(0, LOP);
  assert_eq!(0, ROP);
  assert_eq!(0, LOP as i8);
  assert_eq!(0, ROP as i8);

  cpu.lda(LOP);
  cpu.sbc(ROP);
  assert_eq!(true, cpu.registers.get_flag(rs_nes::cpu::FL_ZERO));
  assert_eq!(false, cpu.registers.get_flag(rs_nes::cpu::FL_SIGN));
  assert_eq!(0x0, cpu.registers.acc);

  // decimal values sannity check
  assert_eq!(0, cpu.registers.acc);
  assert_eq!(0, cpu.registers.acc as i8);
}

#[test]
fn sbc_flags_sign_and_zero_2() {
  let mut cpu = Cpu6502::new();

  // operands
  const LOP: u8 = 0x1;
  const ROP: u8 = 0x1;

  // decimal values sanity check
  assert_eq!(1, LOP);
  assert_eq!(1, ROP);
  assert_eq!(1, LOP as i8);
  assert_eq!(1, ROP as i8);

  cpu.lda(LOP);
  cpu.sbc(ROP);
  assert_eq!(0x0, cpu.registers.acc);
  assert_eq!(true, cpu.registers.get_flag(rs_nes::cpu::FL_ZERO));
  assert_eq!(false, cpu.registers.get_flag(rs_nes::cpu::FL_SIGN));

  // decimal values sannity check
  assert_eq!(0, cpu.registers.acc);
  assert_eq!(0, cpu.registers.acc as i8);
}

#[test]
fn sbc_flags_sign_and_zero_3() {
  let mut cpu = Cpu6502::new();

  // operands
  const LOP: u8 = 0x0;
  const ROP: u8 = 0x1;

  // decimal values sanity check
  assert_eq!(0, LOP);
  assert_eq!(1, ROP);
  assert_eq!(0, LOP as i8);
  assert_eq!(1, ROP as i8);

  cpu.lda(LOP);
  cpu.sbc(ROP);
  assert_eq!(false, cpu.registers.get_flag(rs_nes::cpu::FL_ZERO));
  assert_eq!(true, cpu.registers.get_flag(rs_nes::cpu::FL_SIGN));
  assert_eq!(0xff, cpu.registers.acc);

  // decimal values sannity check
  assert_eq!(255, cpu.registers.acc);
  assert_eq!(-1, cpu.registers.acc as i8);
}

/// ## Carry and overflow flag tests
///
/// The following tests check all permutations of the
/// 6th and 7th bits of both operands, asserting that
/// the overflow and carry bit is set appROPriately.
///
/// A truth table for these tests can be found here:
/// http://www.righto.com/2012/12/the-6502-overflow-flag-explained.html
///
/// ONE THING TO REMEMBER FOR THE SBC TRUTH TABLE -- The implied "borrow" flag
/// is set when the *actual* carry flag is clear, and vice-versa.

#[test]
fn sbc_flags_carry_and_overflow_1() {
  let mut cpu = Cpu6502::new();

  // operands
  const LOP: u8 = 0x50;
  const ROP: u8 = 0xf0;

  // decimal values sanity check
  assert_eq!(80, LOP);
  assert_eq!(240, ROP);
  assert_eq!(80, LOP as i8);
  assert_eq!(-16, ROP as i8);

  cpu.lda(LOP);
  cpu.sbc(ROP);
  assert_eq!(false, cpu.registers.get_flag(rs_nes::cpu::FL_CARRY));
  assert_eq!(false, cpu.registers.get_flag(rs_nes::cpu::FL_OVERFLOW));
  assert_eq!(0x60, cpu.registers.acc);

  // decimal values sanity check
  assert_eq!(96, cpu.registers.acc);
  assert_eq!(96, cpu.registers.acc as i8);
}

#[test]
fn sbc_flags_carry_and_overflow_2() {
  let mut cpu = Cpu6502::new();

  // operands
  const LOP: u8 = 0x50;
  const ROP: u8 = 0xb0;

  // decimal values sanity check
  assert_eq!(80, LOP);
  assert_eq!(176, ROP);
  assert_eq!(80, LOP as i8);
  assert_eq!(-80, ROP as i8);

  cpu.lda(LOP);
  cpu.sbc(ROP);
  assert_eq!(false, cpu.registers.get_flag(rs_nes::cpu::FL_CARRY));
  assert_eq!(true, cpu.registers.get_flag(rs_nes::cpu::FL_OVERFLOW));
  assert_eq!(0xa0, cpu.registers.acc);

  // decimal values sanity check
  assert_eq!(160, cpu.registers.acc);
  assert_eq!(-96, cpu.registers.acc as i8);
}

#[test]
fn sbc_flags_carry_and_overflow_3() {
  let mut cpu = Cpu6502::new();

  // operands
  const LOP: u8 = 0x50;
  const ROP: u8 = 0x70;

  // decimal values sanity check
  assert_eq!(80, LOP);
  assert_eq!(112, ROP);
  assert_eq!(80, LOP as i8);
  assert_eq!(112, ROP as i8);

  cpu.lda(LOP);
  cpu.sbc(ROP);
  assert_eq!(false, cpu.registers.get_flag(rs_nes::cpu::FL_CARRY));
  assert_eq!(false, cpu.registers.get_flag(rs_nes::cpu::FL_OVERFLOW));
  assert_eq!(0xe0, cpu.registers.acc);

  // decimal values sanity check
  assert_eq!(224, cpu.registers.acc);
  assert_eq!(-32, cpu.registers.acc as i8);
}

#[test]
fn sbc_flags_carry_and_overflow_4() {
  let mut cpu = Cpu6502::new();

  // operands
  const LOP: u8 = 0x50;
  const ROP: u8 = 0x30;

  // decimal values sanity check
  assert_eq!(80, LOP);
  assert_eq!(48, ROP);
  assert_eq!(80, LOP as i8);
  assert_eq!(48, ROP as i8);

  cpu.lda(LOP);
  cpu.sbc(ROP);
  assert_eq!(true, cpu.registers.get_flag(rs_nes::cpu::FL_CARRY));
  assert_eq!(false, cpu.registers.get_flag(rs_nes::cpu::FL_OVERFLOW));
  assert_eq!(0x20, cpu.registers.acc);

  // decimal values sanity check
  assert_eq!(32, cpu.registers.acc);
  assert_eq!(32, cpu.registers.acc as i8);
}

#[test]
fn sbc_flags_carry_and_overflow_5() {
  let mut cpu = Cpu6502::new();

  // operands
  const LOP: u8 = 0xd0;
  const ROP: u8 = 0xf0;

  // decimal values sanity check
  assert_eq!(208, LOP);
  assert_eq!(240, ROP);
  assert_eq!(-48, LOP as i8);
  assert_eq!(-16, ROP as i8);

  cpu.lda(LOP);
  cpu.sbc(ROP);
  assert_eq!(false, cpu.registers.get_flag(rs_nes::cpu::FL_CARRY));
  assert_eq!(false, cpu.registers.get_flag(rs_nes::cpu::FL_OVERFLOW));
  assert_eq!(0xe0, cpu.registers.acc);

  // decimal values sanity check
  assert_eq!(224, cpu.registers.acc);
  assert_eq!(-32, cpu.registers.acc as i8);
}

#[test]
fn sbc_flags_carry_and_overflow_6() {
  let mut cpu = Cpu6502::new();

  // operands
  const LOP: u8 = 0xd0;
  const ROP: u8 = 0xb0;

  // decimal values sanity check
  assert_eq!(208, LOP);
  assert_eq!(176, ROP);
  assert_eq!(-48, LOP as i8);
  assert_eq!(-80, ROP as i8);

  cpu.lda(LOP);
  cpu.sbc(ROP);
  assert_eq!(true, cpu.registers.get_flag(rs_nes::cpu::FL_CARRY));
  assert_eq!(false, cpu.registers.get_flag(rs_nes::cpu::FL_OVERFLOW));
  assert_eq!(0x20, cpu.registers.acc);

  // decimal values sanity check
  assert_eq!(32, cpu.registers.acc);
  assert_eq!(32, cpu.registers.acc as i8);
}

#[test]
fn sbc_flags_carry_and_overflow_7() {
  let mut cpu = Cpu6502::new();

  // operands
  const LOP: u8 = 0xd0;
  const ROP: u8 = 0x70;

  // decimal values sanity check
  assert_eq!(208, LOP);
  assert_eq!(112, ROP);
  assert_eq!(-48, LOP as i8);
  assert_eq!(112, ROP as i8);

  cpu.lda(LOP);
  cpu.sbc(ROP);
  assert_eq!(true, cpu.registers.get_flag(rs_nes::cpu::FL_CARRY));
  assert_eq!(true, cpu.registers.get_flag(rs_nes::cpu::FL_OVERFLOW));
  assert_eq!(0x60, cpu.registers.acc);

  // decimal values sanity check
  assert_eq!(96, cpu.registers.acc);
  assert_eq!(96, cpu.registers.acc as i8);
}

#[test]
fn sbc_flags_carry_and_overflow_8() {
  let mut cpu = Cpu6502::new();

  // operands
  const LOP: u8 = 0xd0;
  const ROP: u8 = 0x30;

  // decimal values sanity check
  assert_eq!(208, LOP);
  assert_eq!(48, ROP);
  assert_eq!(-48, LOP as i8);
  assert_eq!(48, ROP as i8);

  cpu.lda(LOP);
  cpu.sbc(ROP);
  assert_eq!(true, cpu.registers.get_flag(rs_nes::cpu::FL_CARRY));
  assert_eq!(false, cpu.registers.get_flag(rs_nes::cpu::FL_OVERFLOW));
  assert_eq!(0xa0, cpu.registers.acc);

  // decimal values sanity check
  assert_eq!(160, cpu.registers.acc);
  assert_eq!(-96, cpu.registers.acc as i8);
}
