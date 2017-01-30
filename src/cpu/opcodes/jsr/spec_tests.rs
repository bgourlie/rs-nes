use super::Jsr;
use cpu::*;
use cpu::opcodes::OpCode;

#[test]
fn test() {
    let mut cpu = TestCpu::new_test();
    cpu.registers.sp = 0xff;
    cpu.registers.pc = 0x102;
    Jsr::execute(&mut cpu, 0xbeef_u16);
    assert_eq!(0xfd, cpu.registers.sp);
    let pushed_pc = cpu.pop_stack16();
    assert_eq!(0xbeef, cpu.registers.pc);
    assert_eq!(0x101, pushed_pc);
}
