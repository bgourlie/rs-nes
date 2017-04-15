use super::Clc;
use cpu::*;
use cpu::opcodes::*;

#[test]
fn test() {
    let mut cpu = TestCpu::new_test();
    cpu.registers.set_carry_flag(true);
    Clc::execute(&mut cpu, Implied);
    assert_eq!(false, cpu.registers.carry_flag());
}
