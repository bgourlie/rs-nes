use super::Sta;
use cpu::*;
use cpu::opcodes::OpCode;
use cpu::opcodes::addressing::testing::WriterAddressingMode;

#[test]
fn sta() {
    let mut cpu = TestCpu::new_test();
    cpu.registers.acc = 0xff;
    let am = WriterAddressingMode::new();
    let write_ref = am.write_ref();
    Sta::execute(&mut cpu, am).unwrap();
    assert_eq!(0xff, write_ref.get());
}
