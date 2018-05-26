// TODO: Tests to assert status flags

use cpu::opcodes::*;
use cpu::*;

#[test]
fn tax() {
    let mut cpu = TestCpu::new_test();
    cpu.registers.acc = 0xff;
    cpu.registers.x = 0x0;
    Tax::execute(&mut cpu, Implied);
    assert_eq!(0xff, cpu.registers.x);
}

#[test]
fn tay() {
    let mut cpu = TestCpu::new_test();
    cpu.registers.acc = 0xff;
    cpu.registers.y = 0x0;
    Tay::execute(&mut cpu, Implied);
    assert_eq!(0xff, cpu.registers.y);
}

#[test]
fn tsx() {
    let mut cpu = TestCpu::new_test();
    cpu.registers.sp = 0xee;
    cpu.registers.x = 0x0;
    Tsx::execute(&mut cpu, Implied);
    assert_eq!(0xee, cpu.registers.x);
}

#[test]
fn txa() {
    let mut cpu = TestCpu::new_test();
    cpu.registers.x = 0xee;
    cpu.registers.acc = 0x0;
    Txa::execute(&mut cpu, Implied);
    assert_eq!(0xee, cpu.registers.acc);
}

#[test]
fn txs() {
    let mut cpu = TestCpu::new_test();
    cpu.registers.x = 0xee;
    cpu.registers.sp = 0x0;
    Txs::execute(&mut cpu, Implied);
    assert_eq!(0xee, cpu.registers.x);
}

#[test]
fn tya() {
    let mut cpu = TestCpu::new_test();
    cpu.registers.y = 0xee;
    cpu.registers.acc = 0x0;
    Tya::execute(&mut cpu, Implied);
    assert_eq!(0xee, cpu.registers.acc);
}
