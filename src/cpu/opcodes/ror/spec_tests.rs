use cpu::TestCpu;
use cpu::opcodes::OpCode;
use cpu::opcodes::shift_tests_base::*;
use super::Ror;

fn ror(cpu: &mut TestCpu, val: u8) -> (u8, bool) {
    Ror::execute_cycles(cpu, val);
    (cpu.registers.acc, true)
}

#[test]
fn ror_1() {
    shift_right_base_1(ror);
}

#[test]
fn ror_2() {
    shift_right_base_2(ror);
}

#[test]
fn ror_3() {
    shift_right_base_3(ror);
}
