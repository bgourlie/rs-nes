use cpu::*;
use cpu::opcodes::*;

#[test]
fn test() {
    let mut cpu = TestCpu::new_test();
    let pc = 0xfffe;
    cpu.registers.sp = 0xff;
    cpu.push_stack16(pc);
    Rts::execute(&mut cpu, Implied);
    assert_eq!(0xffff, cpu.registers.pc);
}
