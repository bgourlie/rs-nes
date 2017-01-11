use cpu::*;
use cpu::opcodes::OpCode;
use super::Jsr;

#[test]
fn test() {
    let mut cpu = TestCpu::new_test();
    let tick_handler = |_: &TestCpu| {};
    cpu.registers.sp = 0xff;
    cpu.registers.pc = 0x102;
    Jsr::execute_cycles(&mut cpu, 0xbeef_u16);
    assert_eq!(0xfd, cpu.registers.sp);
    let pushed_pc = cpu.pop_stack16(&tick_handler);
    assert_eq!(0xbeef, cpu.registers.pc);
    assert_eq!(0x101, pushed_pc);
}
