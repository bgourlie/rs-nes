use cpu::*;
use cpu::opcodes::*;
use cpu::opcodes::txs::Txs;

#[test]
fn tsx() {
    let mut cpu = TestCpu::new_test();
    cpu.registers.x = 0xee;
    cpu.registers.sp = 0x0;
    Txs::execute(&mut cpu, Implied);
    assert_eq!(0xee, cpu.registers.x);
}

// TODO: Tests to assert status flags
