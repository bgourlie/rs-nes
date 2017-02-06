use cpu::*;
use cpu::opcodes::OpCode;
use cpu::opcodes::addressing::Implied;
use cpu::opcodes::tay::Tay;

#[test]
fn tax() {
    let mut cpu = TestCpu::new_test();
    cpu.registers.acc = 0xff;
    cpu.registers.y = 0x0;
    Tay::execute(&mut cpu, Implied).unwrap();
    assert_eq!(0xff, cpu.registers.y);
}

// TODO: Tests to assert status flags
