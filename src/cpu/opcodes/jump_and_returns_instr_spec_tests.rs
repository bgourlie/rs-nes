// TODO: RTI, BRK tests

use cpu::*;
use cpu::opcodes::*;

#[test]
fn jmp() {
    let mut cpu = TestCpu::new_test();
    Jmp::execute(&mut cpu, 0xbeef_u16);
    assert_eq!(0xbeef, cpu.registers.pc);
}

#[test]
fn jsr() {
    let mut cpu = TestCpu::new_test();
    cpu.registers.sp = 0xff;
    cpu.registers.pc = 0x102;
    Jsr::execute(&mut cpu, 0xbeef_u16);
    assert_eq!(0xfd, cpu.registers.sp);
    let pushed_pc = cpu.pop_stack16();
    assert_eq!(0xbeef, cpu.registers.pc);
    assert_eq!(0x101, pushed_pc);
}

#[test]
fn rts() {
    let mut cpu = TestCpu::new_test();
    let pc = 0xfffe;
    cpu.registers.sp = 0xff;
    cpu.push_stack16(pc);
    Rts::execute(&mut cpu, Implied);
    assert_eq!(0xffff, cpu.registers.pc);
}
