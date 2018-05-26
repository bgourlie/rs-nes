use cpu::opcodes::*;
use cpu::*;

#[test]
fn sei() {
    let mut cpu = TestCpu::new_test();
    cpu.registers.set_interrupt_disable_flag(false);
    Sei::execute(&mut cpu, Implied);
    assert_eq!(true, cpu.registers.interrupt_disable_flag());
}

#[test]
fn clc() {
    let mut cpu = TestCpu::new_test();
    cpu.registers.set_carry_flag(true);
    Clc::execute(&mut cpu, Implied);
    assert_eq!(false, cpu.registers.carry_flag());
}

#[test]
fn cld() {
    let mut cpu = TestCpu::new_test();
    cpu.registers.set_decimal_flag(true);
    Cld::execute(&mut cpu, Implied);
    assert_eq!(false, cpu.registers.decimal_flag());
}

#[test]
fn cli() {
    let mut cpu = TestCpu::new_test();
    cpu.registers.set_interrupt_disable_flag(true);
    Cli::execute(&mut cpu, Implied);
    assert_eq!(false, cpu.registers.interrupt_disable_flag());
}

#[test]
fn clv() {
    let mut cpu = TestCpu::new_test();
    cpu.registers.set_overflow_flag(true);
    Clv::execute(&mut cpu, Implied);
    assert_eq!(false, cpu.registers.overflow_flag());
}

#[test]
fn sec() {
    let mut cpu = TestCpu::new_test();
    cpu.registers.set_carry_flag(false);
    Sec::execute(&mut cpu, Implied);
    assert_eq!(true, cpu.registers.carry_flag());
}

#[test]
fn sed() {
    let mut cpu = TestCpu::new_test();
    cpu.registers.set_decimal_flag(false);
    Sed::execute(&mut cpu, Implied);
    assert_eq!(true, cpu.registers.decimal_flag());
}
