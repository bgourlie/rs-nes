use super::Bcs;
use cpu::opcodes::OpCode;
use cpu::opcodes::branch_tests_base::*;

#[test]
fn branch_not_crossing_page_boundary_positive_rel_addr() {
    test_branch_not_crossing_page_boundary_positive_offset(|ref mut cpu, offset| {
        cpu.registers.set_carry_flag(true);
        Bcs::execute_cycles(cpu, offset)
    });
}

#[test]
fn branch_not_crossing_page_boundary_negative_rel_addr() {
    test_branch_not_crossing_page_boundary_negative_offset(|ref mut cpu, offset| {
        cpu.registers.set_carry_flag(true);
        Bcs::execute_cycles(cpu, offset)
    });
}

#[test]
fn branch_crossing_page_boundary_positive_rel_addr() {
    test_branch_crossing_page_boundary_positive_offset(|ref mut cpu, offset| {
        cpu.registers.set_carry_flag(true);
        Bcs::execute_cycles(cpu, offset)
    });
}

#[test]
fn branch_crossing_page_boundary_negative_rel_addr() {
    test_branch_crossing_page_boundary_negative_offset(|ref mut cpu, rel_addr| {
        cpu.registers.set_carry_flag(true);
        Bcs::execute_cycles(cpu, rel_addr)
    });
}

#[test]
fn no_branch() {
    test_no_branch(|ref mut cpu, rel_addr| {
        cpu.registers.set_carry_flag(false);
        Bcs::execute_cycles(cpu, rel_addr)
    });
}
