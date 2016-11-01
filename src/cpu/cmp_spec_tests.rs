use cpu::*;
use constants::*;

fn cmp_base_equal_flag_check<F>(setup_and_compare: F)
    where F: Fn(&mut TestCpu, u8, u8)
{

    let mut cpu = TestCpu::new_test();

    let lop = 0x1;
    let rop = 0x1;

    setup_and_compare(&mut cpu, lop, rop);

    assert_eq!(true, cpu.registers.get_flag(FL_ZERO));
    assert_eq!(true, cpu.registers.get_flag(FL_CARRY));
    assert_eq!(false, cpu.registers.get_flag(FL_SIGN));
}

fn cmp_base_less_than_flag_check<F>(setup_and_compare: F)
    where F: Fn(&mut TestCpu, u8, u8)
{
    let mut cpu = TestCpu::new_test();

    let lop = 0x1;
    let rop = 0x2;

    setup_and_compare(&mut cpu, lop, rop);

    assert_eq!(false, cpu.registers.get_flag(FL_ZERO));
    assert_eq!(false, cpu.registers.get_flag(FL_CARRY));
    assert_eq!(true, cpu.registers.get_flag(FL_SIGN));
}

fn cmp_base_greater_than_flag_check<F>(setup_and_compare: F)
    where F: Fn(&mut TestCpu, u8, u8)
{
    let mut cpu = TestCpu::new_test();

    let lop = 0x3;
    let rop = 0x2;

    setup_and_compare(&mut cpu, lop, rop);

    assert_eq!(false, cpu.registers.get_flag(FL_ZERO));
    assert_eq!(true, cpu.registers.get_flag(FL_CARRY));
    assert_eq!(false, cpu.registers.get_flag(FL_SIGN));
}


#[test]
fn cmp_equal_base_flag_check() {
    cmp_base_equal_flag_check(|ref mut cpu, lop, rop| {
        cpu.registers.acc = lop;
        cpu.cmp(rop);
    });
}

#[test]
fn cmp_greater_than_flag_check() {
    cmp_base_greater_than_flag_check(|ref mut cpu, lop, rop| {
        cpu.registers.acc = lop;
        cpu.cmp(rop);
    });
}

#[test]
fn cmp_less_than_flag_check() {
    cmp_base_less_than_flag_check(|ref mut cpu, lop, rop| {
        cpu.registers.acc = lop;
        cpu.cmp(rop);
    });
}

#[test]
fn cpy_equal_base_flag_check() {
    cmp_base_equal_flag_check(|ref mut cpu, lop, rop| {
        cpu.registers.iry = lop;
        cpu.cpy(rop);
    });
}

#[test]
fn cpy_greater_than_flag_check() {
    cmp_base_greater_than_flag_check(|ref mut cpu, lop, rop| {
        cpu.registers.iry = lop;
        cpu.cpy(rop);
    });
}

#[test]
fn cpy_less_than_flag_check() {
    cmp_base_less_than_flag_check(|ref mut cpu, lop, rop| {
        cpu.registers.iry = lop;
        cpu.cpy(rop);
    });
}

#[test]
fn cpx_equal_base_flag_check() {
    cmp_base_equal_flag_check(|ref mut cpu, lop, rop| {
        cpu.registers.irx = lop;
        cpu.cpx(rop);
    });
}

#[test]
fn cpx_greater_than_flag_check() {
    cmp_base_greater_than_flag_check(|ref mut cpu, lop, rop| {
        cpu.registers.irx = lop;
        cpu.cpx(rop);
    });
}

#[test]
fn cpx_less_than_flag_check() {
    cmp_base_less_than_flag_check(|ref mut cpu, lop, rop| {
        cpu.registers.irx = lop;
        cpu.cpx(rop);
    });
}
