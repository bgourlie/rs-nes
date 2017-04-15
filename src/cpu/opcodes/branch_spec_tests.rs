use cpu::*;
use cpu::opcodes::*;

#[test]
fn bpl_not_crossing_page_boundary_positive_offset() {
    test_branch_not_crossing_page_boundary_positive_offset(|ref mut cpu, offset| {
                                                               cpu.registers.set_sign_flag(false);
                                                               Bpl::execute(cpu, offset)
                                                           });
}

#[test]
fn bpl_not_crossing_page_boundary_negative_offset() {
    test_branch_not_crossing_page_boundary_negative_offset(|ref mut cpu, offset| {
                                                               cpu.registers.set_sign_flag(false);
                                                               Bpl::execute(cpu, offset)
                                                           });
}

#[test]
fn bpl_no_branch() {
    test_no_branch(|ref mut cpu, offset| {
                       cpu.registers.set_sign_flag(true);
                       Bpl::execute(cpu, offset)
                   });
}

#[test]
fn bcc_not_crossing_page_boundary_positive_offset() {
    test_branch_not_crossing_page_boundary_positive_offset(|ref mut cpu, offset| {
                                                               cpu.registers.set_carry_flag(false);
                                                               Bcc::execute(cpu, offset)
                                                           });
}

#[test]
fn bcc_not_crossing_page_boundary_negative_offset() {
    test_branch_not_crossing_page_boundary_negative_offset(|ref mut cpu, offset| {
                                                               cpu.registers.set_carry_flag(false);
                                                               Bcc::execute(cpu, offset)
                                                           });
}

#[test]
fn bcc_no_branch() {
    test_no_branch(|ref mut cpu, offset| {
                       cpu.registers.set_carry_flag(true);
                       Bcc::execute(cpu, offset)
                   });
}

#[test]
fn bcs_not_crossing_page_boundary_positive_rel_addr() {
    test_branch_not_crossing_page_boundary_positive_offset(|ref mut cpu, offset| {
                                                               cpu.registers.set_carry_flag(true);
                                                               Bcs::execute(cpu, offset)
                                                           });
}

#[test]
fn bcs_not_crossing_page_boundary_negative_rel_addr() {
    test_branch_not_crossing_page_boundary_negative_offset(|ref mut cpu, offset| {
                                                               cpu.registers.set_carry_flag(true);
                                                               Bcs::execute(cpu, offset)
                                                           });
}

#[test]
fn bcs_no_branch() {
    test_no_branch(|ref mut cpu, rel_addr| {
                       cpu.registers.set_carry_flag(false);
                       Bcs::execute(cpu, rel_addr)
                   });
}

#[test]
fn beq_not_crossing_page_boundary_positive_offset() {
    test_branch_not_crossing_page_boundary_positive_offset(|ref mut cpu, offset| {
                                                               cpu.registers.set_zero_flag(true);
                                                               Beq::execute(cpu, offset)
                                                           });
}

#[test]
fn beq_not_crossing_page_boundary_negative_offset() {
    test_branch_not_crossing_page_boundary_negative_offset(|ref mut cpu, offset| {
                                                               cpu.registers.set_zero_flag(true);
                                                               Beq::execute(cpu, offset)
                                                           });
}

#[test]
fn beq_no_branch() {
    test_no_branch(|ref mut cpu, offset| {
                       cpu.registers.set_zero_flag(false);
                       Beq::execute(cpu, offset)
                   });
}

#[test]
fn bmi_not_crossing_page_boundary_positive_offset() {
    test_branch_not_crossing_page_boundary_positive_offset(|ref mut cpu, offset| {
                                                               cpu.registers.set_sign_flag(true);
                                                               Bmi::execute(cpu, offset)
                                                           });
}

#[test]
fn bmi_not_crossing_page_boundary_negative_offset() {
    test_branch_not_crossing_page_boundary_negative_offset(|ref mut cpu, offset| {
                                                               cpu.registers.set_sign_flag(true);
                                                               Bmi::execute(cpu, offset)
                                                           });
}

#[test]
fn bmi_no_branch() {
    test_no_branch(|ref mut cpu, offset| {
                       cpu.registers.set_sign_flag(false);
                       Bmi::execute(cpu, offset)
                   });
}

#[test]
fn bne_not_crossing_page_boundary_positive_offset() {
    test_branch_not_crossing_page_boundary_positive_offset(|ref mut cpu, offset| {
                                                               cpu.registers.set_zero_flag(false);
                                                               Bne::execute(cpu, offset)
                                                           });
}

#[test]
fn bne_not_crossing_page_boundary_negative_offset() {
    test_branch_not_crossing_page_boundary_negative_offset(|ref mut cpu, offset| {
                                                               cpu.registers.set_zero_flag(false);
                                                               Bne::execute(cpu, offset)
                                                           });
}

#[test]
fn bne_no_branch() {
    test_no_branch(|ref mut cpu, offset| {
                       cpu.registers.set_zero_flag(true);
                       Bne::execute(cpu, offset)
                   });
}

#[test]
fn bvc_not_crossing_page_boundary_positive_offset() {
    test_branch_not_crossing_page_boundary_positive_offset(|ref mut cpu, offset| {
                                                               cpu.registers
                                                                   .set_overflow_flag(false);
                                                               Bvc::execute(cpu, offset)
                                                           });
}

#[test]
fn bvc_not_crossing_page_boundary_negative_offset() {
    test_branch_not_crossing_page_boundary_negative_offset(|ref mut cpu, offset| {
                                                               cpu.registers
                                                                   .set_overflow_flag(false);
                                                               Bvc::execute(cpu, offset)
                                                           });
}

#[test]
fn bvc_no_branch() {
    test_no_branch(|ref mut cpu, offset| {
                       cpu.registers.set_overflow_flag(true);
                       Bvc::execute(cpu, offset)
                   });
}

#[test]
fn bvs_not_crossing_page_boundary_positive_offset() {
    test_branch_not_crossing_page_boundary_positive_offset(|ref mut cpu, offset| {
                                                               cpu.registers
                                                                   .set_overflow_flag(true);
                                                               Bvs::execute(cpu, offset)
                                                           });
}

#[test]
fn bvs_not_crossing_page_boundary_negative_offset() {
    test_branch_not_crossing_page_boundary_negative_offset(|ref mut cpu, offset| {
                                                               cpu.registers
                                                                   .set_overflow_flag(true);
                                                               Bvs::execute(cpu, offset)
                                                           });
}

#[test]
fn bvs_no_branch() {
    test_no_branch(|ref mut cpu, offset| {
                       cpu.registers.set_overflow_flag(false);
                       Bvs::execute(cpu, offset)
                   });
}

fn test_branch_not_crossing_page_boundary_positive_offset<F>(setup_and_branch: F)
    where F: Fn(&mut TestCpu, i8)
{
    let mut cpu = TestCpu::new_test();
    cpu.registers.pc = 43656;
    setup_and_branch(&mut cpu, 5);
    assert_eq!(43661, cpu.registers.pc);
}

fn test_branch_not_crossing_page_boundary_negative_offset<F>(setup_and_branch: F)
    where F: Fn(&mut TestCpu, i8)
{
    let mut cpu = TestCpu::new_test();
    cpu.registers.pc = 43656;
    setup_and_branch(&mut cpu, -1);
    assert_eq!(43655, cpu.registers.pc);
}

fn test_no_branch<F>(setup_and_branch: F)
    where F: Fn(&mut TestCpu, i8)
{
    let mut cpu = TestCpu::new_test();
    cpu.registers.pc = 30;
    setup_and_branch(&mut cpu, -20);
    assert_eq!(30, cpu.registers.pc);
}
