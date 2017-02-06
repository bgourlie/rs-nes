use super::Jmp;
use cpu::*;
use cpu::opcodes::OpCode;

#[test]
fn test() {
    let mut cpu = TestCpu::new_test();
    Jmp::execute(&mut cpu, 0xbeef_u16).unwrap();
    assert_eq!(0xbeef, cpu.registers.pc);
}
