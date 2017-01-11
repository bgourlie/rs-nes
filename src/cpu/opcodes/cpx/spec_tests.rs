use cpu::opcodes::OpCode;
use cpu::opcodes::compare_tests_base::*;
use super::Cpx;

#[test]
fn equal_flag_check() {
    equal_flag_check_base(|ref mut cpu, lhs, rhs| {
        cpu.registers.x = lhs;
        Cpx::execute_cycles(cpu, rhs);
    });
}

#[test]
fn greater_than_flag_check() {
    greater_than_flag_check_base(|ref mut cpu, lhs, rhs| {
        cpu.registers.x = lhs;
        Cpx::execute_cycles(cpu, rhs);
    });
}

#[test]
fn less_than_flag_check() {
    less_than_flag_check_base(|ref mut cpu, lhs, rhs| {
        cpu.registers.x = lhs;
        Cpx::execute_cycles(cpu, rhs);
    });
}
