use cpu::*;
use cpu::opcodes::OpCode;
use cpu::opcodes::addressing::testing::WriterAddressingMode;
use cpu::opcodes::stx::Stx;

#[test]
fn stx() {
    let mut cpu = TestCpu::new_test();
    cpu.registers.x = 0xff;
    let am = WriterAddressingMode::new();
    let write_ref = am.write_ref();
    Stx::execute_cycles(&mut cpu, am);
    assert_eq!(0xff, write_ref.get());
}
