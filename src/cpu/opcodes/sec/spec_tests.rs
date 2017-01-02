use cpu::*;
use cpu::opcodes::OpCode;
use cpu::opcodes::addressing_mode::Implied;
use super::Sec;

#[test]
fn test() {
    let mut cpu = TestCpu::new_test();
    cpu.registers.set_carry_flag(false);
    Sec::execute_cycles(&mut cpu, Implied);
    assert_eq!(true, cpu.registers.carry_flag());
}
