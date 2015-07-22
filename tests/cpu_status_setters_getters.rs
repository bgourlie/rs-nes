extern crate rs_nes;

use rs_nes::cpu::*;

#[test]
fn sec_test() {
  let mut cpu = Cpu6502::new();
  cpu.registers.set_flag(FL_CARRY, false);
  cpu.sec();
  assert_eq!(true, cpu.registers.get_flag(FL_CARRY));
}
