use cpu::TestCpu;
use cpu::opcodes::OpCode;
use cpu::opcodes::shift_utils::spec_tests::*;
use super::Asl;

fn asl(cpu: &mut TestCpu, val: u8) -> (u8, bool) {
    Asl::execute_cycles(cpu, val);
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
