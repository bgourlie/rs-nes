use super::Sec;
use cpu::*;
use cpu::opcodes::*;

#[test]
fn test() {
    let mut cpu = TestCpu::new_test();
    cpu.registers.set_carry_flag(false);
    Sec::execute(&mut cpu, Implied);
    assert_eq!(true, cpu.registers.carry_flag());
}
