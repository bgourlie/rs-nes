use cpu::opcodes::OpCode;
use cpu::opcodes::branch_tests_base::*;
use super::Beq;

#[test]
fn branch_not_crossing_page_boundary_positive_offset() {
    test_branch_not_crossing_page_boundary_positive_offset(|ref mut cpu, offset| {
        cpu.registers.set_zero_flag(true);
        Beq::execute_cycles(cpu, offset)
    });
}

#[test]
fn branch_not_crossing_page_boundary_negative_offset() {
    test_branch_not_crossing_page_boundary_negative_offset(|ref mut cpu, offset| {
        cpu.registers.set_zero_flag(true);
        Beq::execute_cycles(cpu, offset)
    });
}

#[test]
fn branch_crossing_page_boundary_positive_offset() {
    test_branch_crossing_page_boundary_positive_offset(|ref mut cpu, offset| {
        cpu.registers.set_zero_flag(true);
        Beq::execute_cycles(cpu, offset)
    });
}

#[test]
fn branch_crossing_page_boundary_negative_rel_addr() {
    test_branch_crossing_page_boundary_negative_offset(|ref mut cpu, offset| {
        cpu.registers.set_zero_flag(true);
        Beq::execute_cycles(cpu, offset)
    });
}

#[test]
fn no_branch() {
    test_no_branch(|ref mut cpu, offset| {
        cpu.registers.set_zero_flag(false);
        Beq::execute_cycles(cpu, offset)
    });
}
