use cpu::*;
use cpu::opcodes::OpCode;
use cpu::opcodes::addressing::Implied;
use super::Cli;

#[test]
fn test() {
    let mut cpu = TestCpu::new_test();
    cpu.registers.set_interrupt_disable_flag(true);
    Cli::execute_cycles(&mut cpu, Implied);
    assert_eq!(false, cpu.registers.interrupt_disable_flag());
}
