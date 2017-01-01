use cpu::opcodes::OpCode;
use cpu::opcodes::branch_utils::spec_tests::*;
use super::Bcs;

#[test]
fn test_branch_not_crossing_page_boundary_positive_rel_addr() {
    branch_not_crossing_page_boundary_positive_offset(|ref mut cpu, offset| {
        cpu.registers.set_carry_flag(true);
        Bcs::execute_cycles(cpu, offset)
    });
}

#[test]
fn test_branch_not_crossing_page_boundary_negative_rel_addr() {
    branch_not_crossing_page_boundary_negative_offset(|ref mut cpu, offset| {
        cpu.registers.set_carry_flag(true);
        Bcs::execute_cycles(cpu, offset)
    });
}

#[test]
fn test_branch_crossing_page_boundary_positive_rel_addr() {
    branch_crossing_page_boundary_positive_offset(|ref mut cpu, offset| {
        cpu.registers.set_carry_flag(true);
        Bcs::execute_cycles(cpu, offset)
    });
}

#[test]
fn test_branch_crossing_page_boundary_negative_rel_addr() {
    branch_crossing_page_boundary_negative_offset(|ref mut cpu, rel_addr| {
        cpu.registers.set_carry_flag(true);
        Bcs::execute_cycles(cpu, rel_addr)
    });
}

#[test]
fn test_no_branch() {
    no_branch(|ref mut cpu, rel_addr| {
        cpu.registers.set_carry_flag(false);
        Bcs::execute_cycles(cpu, rel_addr)
    });
}
