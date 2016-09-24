use cpu::*;
use memory::*;
use constants::*;

// TODO: assert that sed and cld panic (clear/set decimal flag)

#[test]
fn clc() {
    let mem = SimpleMemory::new();
    let mut cpu = Cpu::new(mem);
    cpu.registers.set_flag(FL_CARRY, true);
    cpu.clc();
    assert_eq!(false, cpu.registers.get_flag(FL_CARRY));
}

#[test]
fn cli() {
    let mem = SimpleMemory::new();
    let mut cpu = Cpu::new(mem);
    cpu.registers.set_flag(FL_INTERRUPT_DISABLE, true);
    cpu.cli();
    assert_eq!(false, cpu.registers.get_flag(FL_INTERRUPT_DISABLE));
}

#[test]
fn clv() {
    let mem = SimpleMemory::new();
    let mut cpu = Cpu::new(mem);
    cpu.registers.set_flag(FL_OVERFLOW, true);
    cpu.clv();
    assert_eq!(false, cpu.registers.get_flag(FL_OVERFLOW));
}

#[test]
fn sec() {
    let mem = SimpleMemory::new();
    let mut cpu = Cpu::new(mem);
    cpu.registers.set_flag(FL_CARRY, false);
    cpu.sec();
    assert_eq!(true, cpu.registers.get_flag(FL_CARRY));
}

#[test]
fn sei() {
    let mem = SimpleMemory::new();
    let mut cpu = Cpu::new(mem);
    cpu.registers.set_flag(FL_INTERRUPT_DISABLE, false);
    cpu.sei();
    assert_eq!(true, cpu.registers.get_flag(FL_INTERRUPT_DISABLE));
}
