use cpu::*;
use cpu::opcodes::OpCode;
use cpu::opcodes::addressing::testing::WriterAddressingMode;
use cpu::opcodes::sty::Sty;

#[test]
fn stx() {
    let mut cpu = TestCpu::new_test();
    cpu.registers.y = 0xff;
    let am = WriterAddressingMode::new();
    let write_ref = am.write_ref();
    Sty::execute(&mut cpu, am).unwrap();
    assert_eq!(0xff, write_ref.get());
}
