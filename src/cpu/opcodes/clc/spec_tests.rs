use super::Clc;
use cpu::*;
use cpu::opcodes::OpCode;
use cpu::opcodes::addressing::Implied;

#[test]
fn test() {
    let mut cpu = TestCpu::new_test();
    cpu.registers.set_carry_flag(true);
    Clc::execute_cycles(&mut cpu, Implied);
    assert_eq!(false, cpu.registers.carry_flag());
}
