use super::Jmp;
use cpu::*;
use cpu::opcodes::*;

#[test]
fn test() {
    let mut cpu = TestCpu::new_test();
    Jmp::execute(&mut cpu, 0xbeef_u16);
    assert_eq!(0xbeef, cpu.registers.pc);
}
