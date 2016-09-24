use cpu::*;
use memory::*;

#[test]
fn sta_test() {
    let mem = SimpleMemory::new();
    let mut cpu = Cpu::new(mem);
    assert_eq!(0x0, cpu.memory.load(0x0));
    cpu.registers.acc = 0xff;
    cpu.sta(0x0);
    assert_eq!(0xff, cpu.memory.load(0x0));
}

#[test]
fn stx_test() {
    let mem = SimpleMemory::new();
    let mut cpu = Cpu::new(mem);
    assert_eq!(0x0, cpu.memory.load(0x0));
    cpu.registers.irx = 0xff;
    cpu.stx(0x0);
    assert_eq!(0xff, cpu.memory.load(0x0));
}

#[test]
fn sty_test() {
    let mem = SimpleMemory::new();
    let mut cpu = Cpu::new(mem);
    assert_eq!(0x0, cpu.memory.load(0x0));
    cpu.registers.iry = 0xff;
    cpu.sty(0x0);
    assert_eq!(0xff, cpu.memory.load(0x0));
}
