extern crate rs_nes;

use rs_nes::cpu::*;

#[test]
fn jmp() {
  let mut cpu = Cpu6502::new();
  cpu.jmp(0xbeef);
  assert_eq!(0xbeef, cpu.registers.pc);
}

#[test]
fn jsr() {
  let mut cpu = Cpu6502::new();
  cpu.registers.sp = 0xff;
  cpu.registers.pc = 0x102;
  cpu.jsr(0xbeef);
  let pushed = cpu.peek_stack16();
  assert_eq!(0xbeef, cpu.registers.pc);
  assert_eq!(0x101, pushed);
}

#[test]
fn rts() {
  let mut cpu = Cpu6502::new();
  cpu.registers.sp = 0xff;
  cpu.push_stack16(0xfffe);
  cpu.rts();
  assert_eq!(0xffff, cpu.registers.pc);
}
