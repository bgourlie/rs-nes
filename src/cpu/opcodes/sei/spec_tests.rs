use super::Sei;
use cpu::*;
use cpu::opcodes::*;

#[test]
fn test() {
    let mut cpu = TestCpu::new_test();
    cpu.registers.set_interrupt_disable_flag(false);
    Sei::execute(&mut cpu, Implied);
    assert_eq!(true, cpu.registers.interrupt_disable_flag());
}
