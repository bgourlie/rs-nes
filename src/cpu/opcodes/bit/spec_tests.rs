use super::Bit;
use cpu::*;
use cpu::opcodes::OpCode;

#[test]
fn zero_flag_behavior() {
    let mut cpu = TestCpu::new_test();
    cpu.registers.acc = 0;
    Bit::execute(&mut cpu, 0_u8);
    assert_eq!(true, cpu.registers.zero_flag());

    let mut cpu = TestCpu::new_test();
    cpu.registers.acc = 0b11110000;
    Bit::execute(&mut cpu, 0b00001111_u8);
    assert_eq!(true, cpu.registers.zero_flag());

    let mut cpu = TestCpu::new_test();
    cpu.registers.acc = 0b00111100;
    Bit::execute(&mut cpu, 0b00011000_u8);
    assert_eq!(false, cpu.registers.zero_flag());
}

#[test]
fn sign_flag_behavior() {
    let mut cpu = TestCpu::new_test();
    Bit::execute(&mut cpu, 0b01111111_u8);
    assert_eq!(false, cpu.registers.sign_flag());

    let mut cpu = TestCpu::new_test();
    Bit::execute(&mut cpu, 0b10000000_u8);
    assert_eq!(true, cpu.registers.sign_flag());
}

#[test]
fn overflow_flag_behavior() {
    let mut cpu = TestCpu::new_test();
    Bit::execute(&mut cpu, 0b10111111_u8);
    assert_eq!(false, cpu.registers.overflow_flag());

    let mut cpu = TestCpu::new_test();
    Bit::execute(&mut cpu, 0b01000000_u8);
    assert_eq!(true, cpu.registers.overflow_flag());
}
