use cpu::*;
use cpu::opcodes::addressing::Implied;
use cpu::opcodes::OpCode;
use memory::SimpleMemory;
use super::Rts;

#[test]
fn test() {
    let mut cpu = TestCpu::new_test();
    let tick_handler = |_: &Cpu<SimpleMemory>| {};
    let pc = 0xfffe;
    cpu.registers.sp = 0xff;
    cpu.push_stack16(pc, &tick_handler);
    Rts::execute_cycles(&mut cpu, Implied);
    assert_eq!(0xffff, cpu.registers.pc);
}
