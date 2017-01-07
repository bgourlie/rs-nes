use cpu::*;
use cpu::byte_utils::from_lo_hi;
use cpu::opcodes::OpCode;
use super::Jsr;

#[test]
fn test() {
    let mut cpu = TestCpu::new_test();
    let tick_handler = |_: &Cpu<SimpleMemory>| {};
    cpu.registers.sp = 0xff;
    cpu.registers.pc = 0x102;
    Jsr::execute_cycles(&mut cpu, 0xbeef_u16);
    assert_eq!(0xfd, cpu.registers.sp);
    let pc_low = cpu.pop_stack(&tick_handler);
    let pc_high = cpu.pop_stack(&tick_handler);
    let pushed_pc = from_lo_hi(pc_low, pc_high);
    assert_eq!(0xbeef, cpu.registers.pc);
    assert_eq!(0x101, pushed_pc);
}
