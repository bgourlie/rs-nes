use super::Lsr;
use cpu::TestCpu;
use cpu::opcodes::OpCode;
use cpu::opcodes::shift_tests_base::*;

fn lsr(cpu: &mut TestCpu, val: u8) -> (u8, bool) {
    Lsr::execute_cycles(cpu, val);
    (cpu.registers.acc, false)
}

#[test]
fn lsr_1() {
    shift_right_base_1(lsr);
}

#[test]
fn lsr_2() {
    shift_right_base_2(lsr);
}

#[test]
fn lsr_3() {
    shift_right_base_3(lsr);
}
