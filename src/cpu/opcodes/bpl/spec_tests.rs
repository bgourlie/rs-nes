use cpu::opcodes::OpCode;
use cpu::opcodes::branch_tests_base::*;
use super::Bpl;

#[test]
fn test_branch_not_crossing_page_boundary_positive_offset() {
    branch_not_crossing_page_boundary_positive_offset(|ref mut cpu, offset| {
        cpu.registers.set_sign_flag(false);
        Bpl::execute_cycles(cpu, offset)
    });
}

#[test]
fn test_branch_not_crossing_page_boundary_negative_offset() {
    branch_not_crossing_page_boundary_negative_offset(|ref mut cpu, offset| {
        cpu.registers.set_sign_flag(false);
        Bpl::execute_cycles(cpu, offset)
    });
}

#[test]
fn test_branch_crossing_page_boundary_positive_offset() {
    branch_crossing_page_boundary_positive_offset(|ref mut cpu, offset| {
        cpu.registers.set_sign_flag(false);
        Bpl::execute_cycles(cpu, offset)
    });
}

#[test]
fn test_branch_crossing_page_boundary_negative_rel_addr() {
    branch_crossing_page_boundary_negative_offset(|ref mut cpu, offset| {
        cpu.registers.set_sign_flag(false);
        Bpl::execute_cycles(cpu, offset)
    });
}

#[test]
fn test_no_branch() {
    no_branch(|ref mut cpu, offset| {
        cpu.registers.set_sign_flag(true);
        Bpl::execute_cycles(cpu, offset)
    });
}
