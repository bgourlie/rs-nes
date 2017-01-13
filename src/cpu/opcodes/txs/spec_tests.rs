use cpu::*;
use cpu::opcodes::OpCode;
use cpu::opcodes::addressing::Implied;
use cpu::opcodes::txs::Txs;

#[test]
fn tsx() {
    let mut cpu = TestCpu::new_test();
    cpu.registers.x = 0xee;
    cpu.registers.sp = 0x0;
    Txs::execute_cycles(&mut cpu, Implied);
    assert_eq!(0xee, cpu.registers.x);
}

// TODO: Tests to assert status flags
