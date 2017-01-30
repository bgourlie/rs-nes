use cpu::*;
use cpu::opcodes::OpCode;
use cpu::opcodes::addressing::Implied;
use cpu::opcodes::tax::Tax;

#[test]
fn tax() {
    let mut cpu = TestCpu::new_test();
    cpu.registers.acc = 0xff;
    cpu.registers.x = 0x0;
    Tax::execute(&mut cpu, Implied);
    assert_eq!(0xff, cpu.registers.x);
}

// TODO: Tests to assert status flags
