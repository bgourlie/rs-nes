use cpu::*;
use cpu::opcodes::*;
use cpu::opcodes::am_test_utils::*;

use cpu::opcodes::sty::Sty;

#[test]
fn stx() {
    let mut cpu = TestCpu::new_test();
    cpu.registers.y = 0xff;
    let am = WriterAddressingMode::new();
    let write_ref = am.write_ref();
    Sty::execute(&mut cpu, am);
    assert_eq!(0xff, write_ref.get());
}
