use cpu::*;
use cpu::opcodes::OpCode;
use cpu::opcodes::addressing_mode::Implied;
use super::Sei;

#[test]
fn test() {
    let mut cpu = TestCpu::new_test();
    cpu.registers.set_interrupt_disable_flag(false);
    Sei::execute_cycles(&mut cpu, Implied);
    assert_eq!(true, cpu.registers.interrupt_disable_flag());
}
