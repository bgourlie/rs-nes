use cpu::*;
use memory::*;

#[test]
fn sta_test() {
    let mut cpu = TestCpu::new_test();
    assert_eq!(0x0, cpu.memory.load(0x0));
    cpu.registers.acc = 0xff;
    cpu.sta(0x0_u16);
    assert_eq!(0xff, cpu.memory.load(0x0));
}

#[test]
fn stx_test() {
    let mut cpu = TestCpu::new_test();
    assert_eq!(0x0, cpu.memory.load(0x0));
    cpu.registers.x = 0xff;
    cpu.stx(0x0_u16);
    assert_eq!(0xff, cpu.memory.load(0x0));
}

#[test]
fn sty_test() {
    let mut cpu = TestCpu::new_test();
    assert_eq!(0x0, cpu.memory.load(0x0));
    cpu.registers.y = 0xff;
    cpu.sty(0x0_u16);
    assert_eq!(0xff, cpu.memory.load(0x0));
}
