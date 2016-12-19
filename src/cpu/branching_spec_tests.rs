use cpu::*;

/// Generic branching tests

fn branch_not_crossing_page_boundary_positive_rel_addr<F>(setup_and_branch: F)
    where F: Fn(&mut TestCpu, i8) -> u8
{
    let mut cpu = TestCpu::new_test();

    const PC_START: u16 = 0xaa88;
    const REL_ADDR: i8 = 0x5;

    // decimal values sanity check
    assert_eq!(43656, PC_START);
    assert_eq!(5, REL_ADDR);

    cpu.registers.pc = PC_START;
    let cycles = setup_and_branch(&mut cpu, REL_ADDR);

    assert_eq!(1, cycles);
    assert_eq!(0xaa8d, cpu.registers.pc);

    // decimal values sanity check
    assert_eq!(43661, cpu.registers.pc);
}

fn branch_not_crossing_page_boundary_negative_rel_addr<F>(setup_and_branch: F)
    where F: Fn(&mut TestCpu, i8) -> u8
{
    let mut cpu = TestCpu::new_test();

    const PC_START: u16 = 43656;
    const REL_ADDR: i8 = -1;

    cpu.registers.pc = PC_START;

    let cycles = setup_and_branch(&mut cpu, REL_ADDR);

    assert_eq!(1, cycles);
    assert_eq!(0xaa87, cpu.registers.pc);

    // decimal values sanity check
    assert_eq!(43655, cpu.registers.pc);
}

fn branch_crossing_page_boundary_positive_rel_addr<F>(setup_and_branch: F)
    where F: Fn(&mut TestCpu, i8) -> u8
{
    let mut cpu = TestCpu::new_test();

    const PC_START: u16 = 0xaa88;
    const REL_ADDR: i8 = 0x7f;

    // decimal values sanity check
    assert_eq!(43656, PC_START);
    assert_eq!(127, REL_ADDR);

    cpu.registers.pc = PC_START;

    let cycles = setup_and_branch(&mut cpu, REL_ADDR);

    assert_eq!(2, cycles);
    assert_eq!(0xab07, cpu.registers.pc);

    // decimal values sanity check
    assert_eq!(43783, cpu.registers.pc);
}

fn branch_crossing_page_boundary_negative_rel_addr<F>(setup_and_branch: F)
    where F: Fn(&mut TestCpu, i8) -> u8
{
    let mut cpu = TestCpu::new_test();

    const PC_START: u16 = 43520;
    const REL_ADDR: i8 = -128;

    cpu.registers.pc = PC_START;

    let cycles = setup_and_branch(&mut cpu, REL_ADDR);

    assert_eq!(0xa980, cpu.registers.pc);
    assert_eq!(2, cycles);

    // decimal values sanity check
    assert_eq!(43392, cpu.registers.pc);
}

fn no_branch<F>(setup_and_branch: F)
    where F: Fn(&mut TestCpu, i8) -> u8
{
    let mut cpu = TestCpu::new_test();
    cpu.registers.pc = 30;

    let cycles = setup_and_branch(&mut cpu, -20);

    // don't adjust pc when carry is set
    assert_eq!(30, cpu.registers.pc);

    // no additional cycle when not branching
    assert_eq!(0, cycles);
}

/// # BCC
///
/// Branch on carry clear
#[test]
fn bcc_test_branch_not_crossing_page_boundary_positive_rel_addr() {
    branch_not_crossing_page_boundary_positive_rel_addr(|ref mut cpu, rel_addr| {
        cpu.registers.set_carry_flag(false);
        cpu.bcc(rel_addr)
    });
}

#[test]
fn bcc_test_branch_not_crossing_page_boundary_negative_rel_addr() {
    branch_not_crossing_page_boundary_negative_rel_addr(|ref mut cpu, rel_addr| {
        cpu.registers.set_carry_flag(false);
        cpu.bcc(rel_addr)
    });
}

#[test]
fn bcc_test_branch_crossing_page_boundary_positive_rel_addr() {
    branch_crossing_page_boundary_positive_rel_addr(|ref mut cpu, rel_addr| {
        cpu.registers.set_carry_flag(false);
        cpu.bcc(rel_addr)
    });
}

#[test]
fn bcc_test_branch_crossing_page_boundary_negative_rel_addr() {
    branch_crossing_page_boundary_negative_rel_addr(|ref mut cpu, rel_addr| {
        cpu.registers.set_carry_flag(false);
        cpu.bcc(rel_addr)
    });
}

#[test]
fn bcc_test_no_branch() {
    no_branch(|ref mut cpu, rel_addr| {
        cpu.registers.set_carry_flag(true);
        cpu.bcc(rel_addr)
    });
}

/// # BCS
///
/// Branch on carry set
#[test]
fn bcs_test_branch_not_crossing_page_boundary_positive_rel_addr() {
    branch_not_crossing_page_boundary_positive_rel_addr(|ref mut cpu, rel_addr| {
        cpu.registers.set_carry_flag(true);
        cpu.bcs(rel_addr)
    });
}

#[test]
fn bcs_test_branch_not_crossing_page_boundary_negative_rel_addr() {
    branch_not_crossing_page_boundary_negative_rel_addr(|ref mut cpu, rel_addr| {
        cpu.registers.set_carry_flag(true);
        cpu.bcs(rel_addr)
    });
}

#[test]
fn bcs_test_branch_crossing_page_boundary_positive_rel_addr() {
    branch_crossing_page_boundary_positive_rel_addr(|ref mut cpu, rel_addr| {
        cpu.registers.set_carry_flag(true);
        cpu.bcs(rel_addr)
    });
}

#[test]
fn bcs_test_branch_crossing_page_boundary_negative_rel_addr() {
    branch_crossing_page_boundary_negative_rel_addr(|ref mut cpu, rel_addr| {
        cpu.registers.set_carry_flag(true);
        cpu.bcs(rel_addr)
    });
}

#[test]
fn bcs_test_no_branch() {
    no_branch(|ref mut cpu, rel_addr| {
        cpu.registers.set_carry_flag(false);
        cpu.bcs(rel_addr)
    });
}

/// # BEQ
///
/// Branch if equal
#[test]
fn beq_test_branch_not_crossing_page_boundary_positive_rel_addr() {
    branch_not_crossing_page_boundary_positive_rel_addr(|ref mut cpu, rel_addr| {
        cpu.registers.set_zero_flag(true);
        cpu.beq(rel_addr)
    });
}

#[test]
fn beq_test_branch_not_crossing_page_boundary_negative_rel_addr() {
    branch_not_crossing_page_boundary_negative_rel_addr(|ref mut cpu, rel_addr| {
        cpu.registers.set_zero_flag(true);
        cpu.beq(rel_addr)
    });
}

#[test]
fn beq_test_branch_crossing_page_boundary_positive_rel_addr() {
    branch_crossing_page_boundary_positive_rel_addr(|ref mut cpu, rel_addr| {
        cpu.registers.set_zero_flag(true);
        cpu.beq(rel_addr)
    });
}

#[test]
fn beq_test_branch_crossing_page_boundary_negative_rel_addr() {
    branch_crossing_page_boundary_negative_rel_addr(|ref mut cpu, rel_addr| {
        cpu.registers.set_zero_flag(true);
        cpu.beq(rel_addr)
    });
}

#[test]
fn beq_test_no_branch() {
    no_branch(|ref mut cpu, rel_addr| {
        cpu.registers.set_zero_flag(false);
        cpu.beq(rel_addr)
    });
}

/// # BMI
///
/// Branch if negative set
#[test]
fn bmi_test_branch_not_crossing_page_boundary_positive_rel_addr() {
    branch_not_crossing_page_boundary_positive_rel_addr(|ref mut cpu, rel_addr| {
        cpu.registers.set_sign_flag(true);
        cpu.bmi(rel_addr)
    });
}

#[test]
fn bmi_test_branch_not_crossing_page_boundary_negative_rel_addr() {
    branch_not_crossing_page_boundary_negative_rel_addr(|ref mut cpu, rel_addr| {
        cpu.registers.set_sign_flag(true);
        cpu.bmi(rel_addr)
    });
}

#[test]
fn bmi_test_branch_crossing_page_boundary_positive_rel_addr() {
    branch_crossing_page_boundary_positive_rel_addr(|ref mut cpu, rel_addr| {
        cpu.registers.set_sign_flag(true);
        cpu.bmi(rel_addr)
    });
}

#[test]
fn bmi_test_branch_crossing_page_boundary_negative_rel_addr() {
    branch_crossing_page_boundary_negative_rel_addr(|ref mut cpu, rel_addr| {
        cpu.registers.set_sign_flag(true);
        cpu.bmi(rel_addr)
    });
}

#[test]
fn bmi_test_no_branch() {
    no_branch(|ref mut cpu, rel_addr| {
        cpu.registers.set_sign_flag(false);
        cpu.bmi(rel_addr)
    });
}

/// # BNE
///
/// Branch if not equal
#[test]
fn bne_test_branch_not_crossing_page_boundary_positive_rel_addr() {
    branch_not_crossing_page_boundary_positive_rel_addr(|ref mut cpu, rel_addr| {
        cpu.registers.set_zero_flag(false);
        cpu.bne(rel_addr)
    });
}

#[test]
fn bne_test_branch_not_crossing_page_boundary_negative_rel_addr() {
    branch_not_crossing_page_boundary_negative_rel_addr(|ref mut cpu, rel_addr| {
        cpu.registers.set_zero_flag(false);
        cpu.bne(rel_addr)
    });
}

#[test]
fn bne_test_branch_crossing_page_boundary_positive_rel_addr() {
    branch_crossing_page_boundary_positive_rel_addr(|ref mut cpu, rel_addr| {
        cpu.registers.set_zero_flag(false);
        cpu.bne(rel_addr)
    });
}

#[test]
fn bne_test_branch_crossing_page_boundary_negative_rel_addr() {
    branch_crossing_page_boundary_negative_rel_addr(|ref mut cpu, rel_addr| {
        cpu.registers.set_zero_flag(false);
        cpu.bne(rel_addr)
    });
}

#[test]
fn bne_test_no_branch() {
    no_branch(|ref mut cpu, rel_addr| {
        cpu.registers.set_zero_flag(true);
        cpu.bne(rel_addr)
    });
}

/// # BPL
///
/// Branch if negative clear
#[test]
fn bpl_test_branch_not_crossing_page_boundary_positive_rel_addr() {
    branch_not_crossing_page_boundary_positive_rel_addr(|ref mut cpu, rel_addr| {
        cpu.registers.set_sign_flag(false);
        cpu.bpl(rel_addr)
    });
}

#[test]
fn bpl_test_branch_not_crossing_page_boundary_negative_rel_addr() {
    branch_not_crossing_page_boundary_negative_rel_addr(|ref mut cpu, rel_addr| {
        cpu.registers.set_sign_flag(false);
        cpu.bpl(rel_addr)
    });
}

#[test]
fn bpl_test_branch_crossing_page_boundary_positive_rel_addr() {
    branch_crossing_page_boundary_positive_rel_addr(|ref mut cpu, rel_addr| {
        cpu.registers.set_sign_flag(false);
        cpu.bpl(rel_addr)
    });
}

#[test]
fn bpl_test_branch_crossing_page_boundary_negative_rel_addr() {
    branch_crossing_page_boundary_negative_rel_addr(|ref mut cpu, rel_addr| {
        cpu.registers.set_sign_flag(false);
        cpu.bpl(rel_addr)
    });
}

#[test]
fn bpl_test_no_branch() {
    no_branch(|ref mut cpu, rel_addr| {
        cpu.registers.set_sign_flag(true);
        cpu.bpl(rel_addr)
    });
}

/// # BVC
///
/// Branch if overflow clear
#[test]
fn bvc_test_branch_not_crossing_page_boundary_positive_rel_addr() {
    branch_not_crossing_page_boundary_positive_rel_addr(|ref mut cpu, rel_addr| {
        cpu.registers.set_overflow_flag(false);
        cpu.bvc(rel_addr)
    });
}

#[test]
fn bvc_test_branch_not_crossing_page_boundary_negative_rel_addr() {
    branch_not_crossing_page_boundary_negative_rel_addr(|ref mut cpu, rel_addr| {
        cpu.registers.set_overflow_flag(false);
        cpu.bvc(rel_addr)
    });
}

#[test]
fn bvc_test_branch_crossing_page_boundary_positive_rel_addr() {
    branch_crossing_page_boundary_positive_rel_addr(|ref mut cpu, rel_addr| {
        cpu.registers.set_overflow_flag(false);
        cpu.bvc(rel_addr)
    });
}

#[test]
fn bvc_test_branch_crossing_page_boundary_negative_rel_addr() {
    branch_crossing_page_boundary_negative_rel_addr(|ref mut cpu, rel_addr| {
        cpu.registers.set_overflow_flag(false);
        cpu.bvc(rel_addr)
    });
}

#[test]
fn bvc_test_no_branch() {
    no_branch(|ref mut cpu, rel_addr| {
        cpu.registers.set_overflow_flag(true);
        cpu.bvc(rel_addr)
    });
}

/// # BVS
///
/// Branch if overflow set
#[test]
fn bvs_test_branch_not_crossing_page_boundary_positive_rel_addr() {
    branch_not_crossing_page_boundary_positive_rel_addr(|ref mut cpu, rel_addr| {
        cpu.registers.set_overflow_flag(true);
        cpu.bvs(rel_addr)
    });
}

#[test]
fn bvs_test_branch_not_crossing_page_boundary_negative_rel_addr() {
    branch_not_crossing_page_boundary_negative_rel_addr(|ref mut cpu, rel_addr| {
        cpu.registers.set_overflow_flag(true);
        cpu.bvs(rel_addr)
    });
}

#[test]
fn bvs_test_branch_crossing_page_boundary_positive_rel_addr() {
    branch_crossing_page_boundary_positive_rel_addr(|ref mut cpu, rel_addr| {
        cpu.registers.set_overflow_flag(true);
        cpu.bvs(rel_addr)
    });
}

#[test]
fn bvs_test_branch_crossing_page_boundary_negative_rel_addr() {
    branch_crossing_page_boundary_negative_rel_addr(|ref mut cpu, rel_addr| {
        cpu.registers.set_overflow_flag(true);
        cpu.bvs(rel_addr)
    });
}

#[test]
fn bvs_test_no_branch() {
    no_branch(|ref mut cpu, rel_addr| {
        cpu.registers.set_overflow_flag(false);
        cpu.bvs(rel_addr)
    });
}
