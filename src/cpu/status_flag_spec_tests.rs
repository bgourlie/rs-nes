use cpu::*;

// TODO: assert that sed and cld panic (clear/set decimal flag)

#[test]
fn clc() {
    let mut cpu = TestCpu::new_test();
    cpu.registers.set_carry_flag(true);
    cpu.clc();
    assert_eq!(false, cpu.registers.carry_flag());
}

#[test]
fn cli() {
    let mut cpu = TestCpu::new_test();
    cpu.registers.set_interrupt_disable_flag(true);
    cpu.cli();
    assert_eq!(false, cpu.registers.interrupt_disable_flag());
}

#[test]
fn clv() {
    let mut cpu = TestCpu::new_test();
    cpu.registers.set_overflow_flag(true);
    cpu.clv();
    assert_eq!(false, cpu.registers.overflow_flag());
}

#[test]
fn sec() {
    let mut cpu = TestCpu::new_test();
    cpu.registers.set_carry_flag(false);
    cpu.sec();
    assert_eq!(true, cpu.registers.carry_flag());
}

#[test]
fn sei() {
    let mut cpu = TestCpu::new_test();
    cpu.registers.set_interrupt_disable_flag(false);
    cpu.sei();
    assert_eq!(true, cpu.registers.interrupt_disable_flag());
}
