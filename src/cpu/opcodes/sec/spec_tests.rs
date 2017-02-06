use super::Sec;
use cpu::*;
use cpu::opcodes::OpCode;
use cpu::opcodes::addressing::Implied;

#[test]
fn test() {
    let mut cpu = TestCpu::new_test();
    cpu.registers.set_carry_flag(false);
    Sec::execute(&mut cpu, Implied).unwrap();
    assert_eq!(true, cpu.registers.carry_flag());
}
