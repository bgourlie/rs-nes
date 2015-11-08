use cpu::*;
use memory::*;

#[test]
fn jmp() {
    let mut mem = SimpleMemory::new();
    let mut cpu = Cpu6502::new(&mut mem);
    cpu.jmp(0xbeef);
    assert_eq!(0xbeef, cpu.registers.pc);
}

#[test]
fn jsr() {
    let mut mem = SimpleMemory::new();
    let mut cpu = Cpu6502::new(&mut mem);
    cpu.registers.sp = 0xff;
    cpu.registers.pc = 0x102;
    cpu.jsr(0xbeef);
    let pushed = cpu.peek_stack16();
    assert_eq!(0xbeef, cpu.registers.pc);
    assert_eq!(0x101, pushed);
}

#[test]
fn rts() {
    let mut mem = SimpleMemory::new();
    let mut cpu = Cpu6502::new(&mut mem);
    cpu.registers.sp = 0xff;
    cpu.push_stack16(0xfffe);
    cpu.rts();
    assert_eq!(0xffff, cpu.registers.pc);
}
