use cpu::*;
use cpu::opcodes::OpCode;
use super::Ldx;

#[test]
fn value_set() {
    let mut cpu = TestCpu::new_test();
    Ldx::execute_cycles(&mut cpu, 0xff_u8);
    assert_eq!(0xff, cpu.registers.x);
}

#[test]
fn flags_sign_and_zero_1() {
    let mut cpu = TestCpu::new_test();
    Ldx::execute_cycles(&mut cpu, 0x0_u8);
    assert_eq!(true, cpu.registers.zero_flag());
    assert_eq!(false, cpu.registers.sign_flag());
}

#[test]
fn flags_sign_and_zero_2() {
    let mut cpu = TestCpu::new_test();
    Ldx::execute_cycles(&mut cpu, 0x1_u8);
    assert_eq!(false, cpu.registers.zero_flag());
    assert_eq!(false, cpu.registers.sign_flag());
}

#[test]
fn flags_sign_and_zero_3() {
    let mut cpu = TestCpu::new_test();
    Ldx::execute_cycles(&mut cpu, 0x80_u8);
    assert_eq!(false, cpu.registers.zero_flag());
    assert_eq!(true, cpu.registers.sign_flag());
}
