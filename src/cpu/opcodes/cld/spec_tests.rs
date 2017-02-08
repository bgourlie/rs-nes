use super::Cld;
use cpu::*;
use cpu::opcodes::OpCode;
use cpu::opcodes::addressing::Implied;

#[test]
fn test() {
    let mut cpu = TestCpu::new_test();
    cpu.registers.set_decimal_flag(true);
    Cld::execute(&mut cpu, Implied).unwrap();
    assert_eq!(false, cpu.registers.decimal_flag());
}
