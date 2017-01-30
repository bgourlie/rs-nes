use super::Bvc;
use cpu::opcodes::OpCode;
use cpu::opcodes::branch_tests_base::*;

#[test]
fn branch_not_crossing_page_boundary_positive_offset() {
    test_branch_not_crossing_page_boundary_positive_offset(|ref mut cpu, offset| {
        cpu.registers.set_overflow_flag(false);
        Bvc::execute(cpu, offset)
    });
}

#[test]
fn branch_not_crossing_page_boundary_negative_offset() {
    test_branch_not_crossing_page_boundary_negative_offset(|ref mut cpu, offset| {
        cpu.registers.set_overflow_flag(false);
        Bvc::execute(cpu, offset)
    });
}

#[test]
fn no_branch() {
    test_no_branch(|ref mut cpu, offset| {
        cpu.registers.set_overflow_flag(true);
        Bvc::execute(cpu, offset)
    });
}
