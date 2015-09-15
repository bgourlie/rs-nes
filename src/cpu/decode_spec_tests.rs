use cpu::*;
use memory::*;

#[test]
fn adc_immed() {
  let mut cpu = Cpu6502::new();
  // ADC #B0
  cpu.memory.store(0x800, 0xa9);
  cpu.memory.store(0x801, 0xb0);
  cpu.registers.pc = 0x800;
  cpu.step();
  assert_eq!(2, cpu.cycles);
  assert_eq!(0x802, cpu.registers.pc);
  assert_eq!(0xb0, cpu.registers.acc);
}

#[test]
fn adc_zero_page() {
  let mut cpu = Cpu6502::new();
  // ADC $AA
  cpu.memory.store(0x800, 0xa5);
  cpu.memory.store(0x801, 0xaa);
  cpu.memory.store(0xaa, 0xb0);
  cpu.registers.pc = 0x800;
  cpu.step();
  assert_eq!(3, cpu.cycles);
  assert_eq!(0x802, cpu.registers.pc);
  assert_eq!(0xb0, cpu.registers.acc);
}

#[test]
fn adc_zero_page_x() {
  let mut cpu = Cpu6502::new();
  // ADC $AA,X
  cpu.memory.store(0x800, 0xb5);
  cpu.memory.store(0x801, 0xaa);
  cpu.memory.store(0xab, 0xb0);
  cpu.registers.irx = 0x1;
  cpu.registers.pc = 0x800;
  cpu.step();
  assert_eq!(4, cpu.cycles);
  assert_eq!(0x802, cpu.registers.pc);
  assert_eq!(0xb0, cpu.registers.acc);
}

#[test]
fn adc_zero_page_x_wrapping_behavior() {
  let mut cpu = Cpu6502::new();
  // ADC $AA,X
  cpu.memory.store(0x800, 0xb5);
  cpu.memory.store(0x801, 0xff);
  cpu.memory.store(0x0, 0xb0);
  cpu.registers.irx = 0x1;
  cpu.registers.pc = 0x800;
  cpu.step();
  assert_eq!(4, cpu.cycles);
  assert_eq!(0x802, cpu.registers.pc);
  assert_eq!(0xb0, cpu.registers.acc);
}

#[test]
fn adc_absolute() {
  let mut cpu = Cpu6502::new();
  // ADC $AA
  cpu.memory.store(0x800, 0xad);
  cpu.memory.store16(0x801, 0xbeef);
  cpu.memory.store16(0xbeef, 0xb0);
  cpu.registers.pc = 0x800;
  cpu.step();
  assert_eq!(4, cpu.cycles);
  assert_eq!(0x803, cpu.registers.pc);
  assert_eq!(0xb0, cpu.registers.acc);
}
