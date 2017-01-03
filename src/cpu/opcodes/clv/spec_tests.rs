use cpu::*;
use cpu::opcodes::OpCode;
use cpu::opcodes::addressing::Implied;
use super::Clv;

#[test]
fn test() {
    let mut cpu = TestCpu::new_test();
    cpu.registers.set_overflow_flag(true);
    Clv::execute_cycles(&mut cpu, Implied);
    assert_eq!(false, cpu.registers.overflow_flag());
}
