use super::Sta;
use cpu::*;
use cpu::opcodes::*;
use cpu::opcodes::am_test_utils::*;

#[test]
fn sta() {
    let mut cpu = TestCpu::new_test();
    cpu.registers.acc = 0xff;
    let am = WriterAddressingMode::new();
    let write_ref = am.write_ref();
    Sta::execute(&mut cpu, am);
    assert_eq!(0xff, write_ref.get());
}
