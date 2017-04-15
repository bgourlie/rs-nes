use super::Asl;
use cpu::TestCpu;
use cpu::opcodes::*;
use cpu::opcodes::shift_tests_base::*;

fn asl(cpu: &mut TestCpu, val: u8) -> (u8, bool) {
    Asl::execute(cpu, val);
    (cpu.registers.acc, false)
}

#[test]
fn asl_1() {
    shift_left_base_1(asl);
}

#[test]
fn asl_2() {
    shift_left_base_2(asl);
}

#[test]
fn asl_3() {
    shift_left_base_3(asl);
}
