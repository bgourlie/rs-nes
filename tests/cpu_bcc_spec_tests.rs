extern crate rs_nes;

use rs_nes::cpu::*;

/// # BCC
///
/// Branch on carry clear

#[test]
fn bcc_test_branch_not_crossing_page_boundary_positive_rel_addr() {
  let mut cpu = Cpu6502::new();

  const pc_start: u16 = 0xaa88;
  const rel_addr: i8 = 0x5;

  // decimal values sanity check
  assert_eq!(43656, pc_start);
  assert_eq!(5, rel_addr);

  cpu.registers.set_flag(FL_CARRY, false);
  cpu.registers.pc = pc_start;

  let cycles = cpu.bcc(rel_addr);

  assert_eq!(1, cycles);
  assert_eq!(0xaa8d, cpu.registers.pc);

  // decimal values sanity check
  assert_eq!(43661, cpu.registers.pc);
}

#[test]
fn bcc_test_branch_not_crossing_page_boundary_negative_rel_addr() {
  let mut cpu = Cpu6502::new();

  const pc_start: u16 = 0xaa88;
  const rel_addr: i8 = 0xffff;

  // decimal values sanity check
  assert_eq!(43656, pc_start);
  assert_eq!(-1, rel_addr);

  cpu.registers.set_flag(FL_CARRY, false);
  cpu.registers.pc = pc_start;
  
  let cycles = cpu.bcc(rel_addr);

  assert_eq!(1, cycles);
  assert_eq!(0xaa87, cpu.registers.pc);

  // decimal values sanity check
  assert_eq!(43655, cpu.registers.pc);
}

#[test]
fn bcc_test_branch_crossing_page_boundary_positive_rel_addr() {
  let mut cpu = Cpu6502::new();

  const pc_start: u16 = 0xaa88;
  const rel_addr: i8 = 0x7f;

  // decimal values sanity check
  assert_eq!(43656, pc_start);
  assert_eq!(127, rel_addr);

  cpu.registers.set_flag(FL_CARRY, false);
  cpu.registers.pc = pc_start;

  let cycles = cpu.bcc(rel_addr);

  assert_eq!(2, cycles);
  assert_eq!(0xab07, cpu.registers.pc);

  // decimal values sanity check
  assert_eq!(43783, cpu.registers.pc);
}

#[test]
fn bcc_test_branch_crossing_page_boundary_negative_rel_addr() {
  let mut cpu = Cpu6502::new();

  const pc_start: u16 = 0xaa00;
  const rel_addr: i8 = 0x80;

  // decimal values sanity check
  assert_eq!(43520, pc_start);
  assert_eq!(-128, rel_addr);

  cpu.registers.set_flag(FL_CARRY, false);
  cpu.registers.pc = pc_start;
  
  let cycles = cpu.bcc(rel_addr);

  assert_eq!(0xa980, cpu.registers.pc);
  assert_eq!(2, cycles);

  // decimal values sanity check
  assert_eq!(43392, cpu.registers.pc);
}

#[test]
fn bcc_test_no_branch() {
  let mut cpu = Cpu6502::new();
  cpu.registers.pc = 30;
  cpu.registers.set_flag(FL_CARRY, true);
  let cycles = cpu.bcc(-20);

  // don't adjust pc when carry is set
  assert_eq!(30, cpu.registers.pc);

  // no additional cycle when not branching
  assert_eq!(0, cycles);
}
