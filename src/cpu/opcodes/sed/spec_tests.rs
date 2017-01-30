use super::Sed;
use cpu::*;
use cpu::opcodes::OpCode;
use cpu::opcodes::addressing::Implied;

#[test]
fn test() {
    let mut cpu = TestCpu::new_test();
    cpu.registers.set_decimal_flag(false);
    Sed::execute(&mut cpu, Implied);
    assert_eq!(true, cpu.registers.decimal_flag());
}
