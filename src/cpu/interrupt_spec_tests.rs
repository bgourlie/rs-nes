use cpu::*;
use memory::*;
use constants::*;

#[test]
fn test_reset() {
    let mut cpu = TestCpu::new_test();
    cpu.memory.store16(RESET_VECTOR, 0xdead);
    cpu.reset();
    assert_eq!(cpu.registers.pc, 0xdead);
}

#[test]
fn test_nmi() {
    let mut cpu = TestCpu::new_test();
    cpu.memory.store16(NMI_VECTOR, 0xdead);
    cpu.nmi();
    assert_eq!(cpu.registers.pc, 0xdead);
}
