extern crate rs_nes;

use rs_nes::cpu::*;

#[test]
fn adc_flags_check_001() {
  let mut cpu = Cpu6502::new();
  cpu.adc(0x00, 0x00);
  assert_eq!(false, cpu.registers.get_flag(rs_nes::cpu::FL_CARRY));
  assert_eq!(false, cpu.registers.get_flag(rs_nes::cpu::FL_OVERFLOW));
  assert_eq!(true, cpu.registers.get_flag(rs_nes::cpu::FL_ZERO));
  assert_eq!(false, cpu.registers.get_flag(rs_nes::cpu::FL_SIGN));
  assert_eq!(0x0, cpu.registers.acc)
}

#[test]
fn adc_flags_check_002() {
  let mut cpu = Cpu6502::new();
  cpu.adc(0xff, 0xff);
  assert_eq!(true, cpu.registers.get_flag(rs_nes::cpu::FL_CARRY));
  assert_eq!(true, cpu.registers.get_flag(rs_nes::cpu::FL_OVERFLOW));
  assert_eq!(false, cpu.registers.get_flag(rs_nes::cpu::FL_ZERO));
  assert_eq!(true, cpu.registers.get_flag(rs_nes::cpu::FL_SIGN));
  assert_eq!(0xfe, cpu.registers.acc)
}

