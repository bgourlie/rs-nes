use cpu::*;
use cpu::opcodes::OpCode;
use cpu::opcodes::addressing::Implied;
use cpu::opcodes::rts::Rts;

#[test]
fn test() {
    let mut cpu = TestCpu::new_test();
    let pc = 0xfffe;
    cpu.registers.sp = 0xff;
    cpu.push_stack16(pc).unwrap();
    Rts::execute(&mut cpu, Implied).unwrap();
    assert_eq!(0xffff, cpu.registers.pc);
}
