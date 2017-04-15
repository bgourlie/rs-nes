use super::Cld;
use cpu::*;
use cpu::opcodes::*;

#[test]
fn test() {
    let mut cpu = TestCpu::new_test();
    cpu.registers.set_decimal_flag(true);
    Cld::execute(&mut cpu, Implied);
    assert_eq!(false, cpu.registers.decimal_flag());
}
