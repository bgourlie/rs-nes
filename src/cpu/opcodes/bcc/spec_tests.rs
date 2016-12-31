use cpu::opcodes::OpCode;
use cpu::opcodes::branch_utils::spec_tests::*;
use super::Bcc;

fn test_branch_not_crossing_page_boundary_positive_rel_addr() {
    branch_not_crossing_page_boundary_positive_rel_addr(|ref mut cpu, rel_addr| {
        cpu.registers.set_carry_flag(false);
        Bcc::execute(cpu, rel_addr)
    });
}

#[test]
fn test_branch_not_crossing_page_boundary_negative_rel_addr() {
    branch_not_crossing_page_boundary_negative_rel_addr(|ref mut cpu, rel_addr| {
        cpu.registers.set_carry_flag(false);
        Bcc::execute(cpu, rel_addr)
    });
}

#[test]
fn test_branch_crossing_page_boundary_positive_rel_addr() {
    branch_crossing_page_boundary_positive_rel_addr(|ref mut cpu, rel_addr| {
        cpu.registers.set_carry_flag(false);
        Bcc::execute(cpu, rel_addr)
    });
}

#[test]
fn test_branch_crossing_page_boundary_negative_rel_addr() {
    branch_crossing_page_boundary_negative_rel_addr(|ref mut cpu, rel_addr| {
        cpu.registers.set_carry_flag(false);
        Bcc::execute(cpu, rel_addr)
    });
}

#[test]
fn test_no_branch() {
    no_branch(|ref mut cpu, rel_addr| {
        cpu.registers.set_carry_flag(true);
        Bcc::execute(cpu, rel_addr)
    });
}
