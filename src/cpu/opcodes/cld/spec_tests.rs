use cpu::*;
use cpu::opcodes::OpCode;
use cpu::opcodes::addressing::Implied;
use super::Cld;

#[test]
fn test() {
    let mut cpu = TestCpu::new_test();
    cpu.registers.set_decimal_flag(true);
    Cld::execute_cycles(&mut cpu, Implied);
    assert_eq!(false, cpu.registers.decimal_flag());
}
