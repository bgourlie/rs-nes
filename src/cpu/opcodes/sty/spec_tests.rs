use cpu::*;
use cpu::opcodes::addressing::testing::WriterAddressingMode;
use cpu::opcodes::OpCode;
use super::Sty;

#[test]
fn stx() {
    let mut cpu = TestCpu::new_test();
    cpu.registers.y = 0xff;
    let am = WriterAddressingMode::new();
    let write_ref = am.write_ref();
    Sty::execute_cycles(&mut cpu, am);
    assert_eq!(0xff, write_ref.get());
}
