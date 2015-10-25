use cpu::*;
use constants::*;

#[test]
fn push_stack() {
    let mut cpu = Cpu6502::new();
    cpu.registers.sp = 0xff;
    let sp = cpu.registers.sp;
    cpu.push_stack(0xde);
    let mem = cpu.memory.load(STACK_LOC + sp as u16);
    assert_eq!(0xfe, cpu.registers.sp);
    assert_eq!(0xde, mem);
}

#[test]
fn push_stack16() {
    let mut cpu = Cpu6502::new();
    cpu.registers.sp = 0xff;
    let sp = cpu.registers.sp;
    cpu.push_stack16(0xdead);
    let mem = cpu.memory.load16(STACK_LOC + sp as u16 - 1);
    assert_eq!(0xfd, cpu.registers.sp);
    assert_eq!(0xdead, mem);
}

#[test]
fn pop_stack() {
    let mut cpu = Cpu6502::new();
    cpu.registers.sp = 0xfe;
    let sp = cpu.registers.sp;
    cpu.memory.store(STACK_LOC + sp as u16 + 1, 0xf0);
    let val = cpu.pop_stack();
    assert_eq!(0xff, cpu.registers.sp);
    assert_eq!(0xf0, val);
}

#[test]
fn pop_stack16() {
    let mut cpu = Cpu6502::new();
    cpu.registers.sp = 0xfd;
    let sp = cpu.registers.sp;
    cpu.memory.store16(STACK_LOC + sp as u16 + 1, 0xf00d);
    let val = cpu.pop_stack16();
    assert_eq!(0xff, cpu.registers.sp);
    assert_eq!(0xf00d, val);
}
