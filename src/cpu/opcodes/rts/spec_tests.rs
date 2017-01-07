use cpu::*;
use cpu::opcodes::addressing::Implied;
use cpu::byte_utils::lo_hi;
use cpu::opcodes::OpCode;
use memory::SimpleMemory;
use super::Rts;

#[test]
fn test() {
    let mut cpu = TestCpu::new_test();
    let tick_handler = |_: &Cpu<SimpleMemory>| {};
    let (pc_low, pc_high) = lo_hi(0xfffe);
    cpu.registers.sp = 0xff;
    cpu.push_stack(pc_high, &tick_handler);
    cpu.push_stack(pc_low, &tick_handler);
    Rts::execute_cycles(&mut cpu, Implied);
    assert_eq!(0xffff, cpu.registers.pc);
}
