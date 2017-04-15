use super::Clv;
use cpu::*;
use cpu::opcodes::*;

#[test]
fn test() {
    let mut cpu = TestCpu::new_test();
    cpu.registers.set_overflow_flag(true);
    Clv::execute(&mut cpu, Implied);
    assert_eq!(false, cpu.registers.overflow_flag());
}
