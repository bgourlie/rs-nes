use cpu::*;
use cpu::opcodes::OpCode;
use cpu::opcodes::addressing::Implied;
use cpu::opcodes::txa::Txa;

#[test]
fn txa() {
    let mut cpu = TestCpu::new_test();
    cpu.registers.x = 0xee;
    cpu.registers.acc = 0x0;
    Txa::execute(&mut cpu, Implied).unwrap();
    assert_eq!(0xee, cpu.registers.acc);
}

// TODO: Tests to assert status flags
