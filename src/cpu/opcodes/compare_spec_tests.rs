use cpu::*;
use cpu::opcodes::*;

#[test]
fn cmp_equal_flag_check() {
    equal_flag_check_base(|ref mut cpu, lhs, rhs| {
                              cpu.registers.acc = lhs;
                              Cmp::execute(cpu, rhs);
                          });
}

#[test]
fn cmp_greater_than_flag_check() {
    greater_than_flag_check_base(|ref mut cpu, lhs, rhs| {
                                     cpu.registers.acc = lhs;
                                     Cmp::execute(cpu, rhs);
                                 });
}

#[test]
fn cmp_less_than_flag_check() {
    less_than_flag_check_base(|ref mut cpu, lhs, rhs| {
                                  cpu.registers.acc = lhs;
                                  Cmp::execute(cpu, rhs);
                              });
}

#[test]
fn cpx_equal_flag_check() {
    equal_flag_check_base(|ref mut cpu, lhs, rhs| {
                              cpu.registers.x = lhs;
                              Cpx::execute(cpu, rhs);
                          });
}

#[test]
fn cpx_greater_than_flag_check() {
    greater_than_flag_check_base(|ref mut cpu, lhs, rhs| {
                                     cpu.registers.x = lhs;
                                     Cpx::execute(cpu, rhs);
                                 });
}

#[test]
fn cpx_less_than_flag_check() {
    less_than_flag_check_base(|ref mut cpu, lhs, rhs| {
                                  cpu.registers.x = lhs;
                                  Cpx::execute(cpu, rhs);
                              });
}

#[test]
fn cpy_equal_flag_check() {
    equal_flag_check_base(|ref mut cpu, lhs, rhs| {
                              cpu.registers.y = lhs;
                              Cpy::execute(cpu, rhs);
                          });
}

#[test]
fn cpy_greater_than_flag_check() {
    greater_than_flag_check_base(|ref mut cpu, lhs, rhs| {
                                     cpu.registers.y = lhs;
                                     Cpy::execute(cpu, rhs);
                                 });
}

#[test]
fn cpy_less_than_flag_check() {
    less_than_flag_check_base(|ref mut cpu, lhs, rhs| {
                                  cpu.registers.y = lhs;
                                  Cpy::execute(cpu, rhs);
                              });
}

fn equal_flag_check_base<F>(setup_and_compare: F)
    where F: Fn(&mut TestCpu, u8, u8)
{
    let mut cpu = TestCpu::new_test();
    setup_and_compare(&mut cpu, 1, 1);
    assert_eq!(true, cpu.registers.zero_flag());
    assert_eq!(true, cpu.registers.carry_flag());
    assert_eq!(false, cpu.registers.sign_flag());
}

fn less_than_flag_check_base<F>(setup_and_compare: F)
    where F: Fn(&mut TestCpu, u8, u8)
{
    let mut cpu = TestCpu::new_test();
    setup_and_compare(&mut cpu, 1, 2);
    assert_eq!(false, cpu.registers.zero_flag());
    assert_eq!(false, cpu.registers.carry_flag());
    assert_eq!(true, cpu.registers.sign_flag());
}

fn greater_than_flag_check_base<F>(setup_and_compare: F)
    where F: Fn(&mut TestCpu, u8, u8)
{
    let mut cpu = TestCpu::new_test();
    setup_and_compare(&mut cpu, 3, 2);
    assert_eq!(false, cpu.registers.zero_flag());
    assert_eq!(true, cpu.registers.carry_flag());
    assert_eq!(false, cpu.registers.sign_flag());
}
