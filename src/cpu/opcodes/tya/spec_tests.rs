use cpu::*;
use cpu::opcodes::OpCode;
use cpu::opcodes::addressing::Implied;
use cpu::opcodes::tya::Tya;

#[test]
fn tya() {
    let mut cpu = TestCpu::new_test();
    cpu.registers.y = 0xee;
    cpu.registers.acc = 0x0;
    Tya::execute(&mut cpu, Implied).unwrap();
    assert_eq!(0xee, cpu.registers.acc);
}

// TODO: Tests to assert status flags
