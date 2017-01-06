use cpu::*;
use cpu::opcodes::addressing::Implied;
use cpu::opcodes::OpCode;
use super::Tsx;

#[test]
fn tsx() {
    let mut cpu = TestCpu::new_test();
    cpu.registers.sp = 0xee;
    cpu.registers.x = 0x0;
    Tsx::execute_cycles(&mut cpu, Implied);
    assert_eq!(0xee, cpu.registers.x);
}

// TODO: Tests to assert status flags