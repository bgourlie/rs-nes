use cpu::*;
use cpu::opcodes::addressing::testing::WriterAddressingMode;
use cpu::opcodes::inc_dec_tests_base::*;
use cpu::opcodes::OpCode;
use super::Sta;

#[test]
fn sta() {
    let mut cpu = TestCpu::new_test();
    cpu.registers.acc = 0xff;
    let am = WriterAddressingMode::new();
    let write_ref = am.write_ref();
    Sta::execute_cycles(&mut cpu, am);
    assert_eq!(0xff, write_ref.get());
}