use cpu::*;
use cpu::opcodes::*;

#[test]
fn tya() {
    let mut cpu = TestCpu::new_test();
    cpu.registers.y = 0xee;
    cpu.registers.acc = 0x0;
    Tya::execute(&mut cpu, Implied);
    assert_eq!(0xee, cpu.registers.acc);
}

// TODO: Tests to assert status flags
