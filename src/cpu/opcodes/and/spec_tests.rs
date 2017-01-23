use super::And;
use cpu::*;
use cpu::opcodes::OpCode;

#[test]
fn and() {
    let mut cpu = TestCpu::new_test();
    cpu.registers.acc = 0_u8;
    And::execute_cycles(&mut cpu, 255_u8);
    assert_eq!(0, cpu.registers.acc);
    assert_eq!(true, cpu.registers.zero_flag());
    assert_eq!(false, cpu.registers.sign_flag());

    let mut cpu = TestCpu::new_test();
    cpu.registers.acc = 0b11110000_u8;
    And::execute_cycles(&mut cpu, 0b10101010_u8);
    assert_eq!(0b10100000, cpu.registers.acc);
    assert_eq!(false, cpu.registers.zero_flag());
    assert_eq!(true, cpu.registers.sign_flag());
}
