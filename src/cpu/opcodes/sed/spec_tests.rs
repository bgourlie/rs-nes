use cpu::*;
use cpu::opcodes::OpCode;
use cpu::opcodes::addressing_mode::Implied;
use super::Sed;

#[test]
fn test() {
    let mut cpu = TestCpu::new_test();
    cpu.registers.set_decimal_flag(false);
    Sed::execute_cycles(&mut cpu, Implied);
    assert_eq!(true, cpu.registers.decimal_flag());
}
