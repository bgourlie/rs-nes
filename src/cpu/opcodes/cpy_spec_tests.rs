use super::Cpy;
use cpu::opcodes::*;
use cpu::opcodes::compare_tests_base::*;

#[test]
fn equal_flag_check() {
    equal_flag_check_base(|ref mut cpu, lhs, rhs| {
                              cpu.registers.y = lhs;
                              Cpy::execute(cpu, rhs);
                          });
}

#[test]
fn greater_than_flag_check() {
    greater_than_flag_check_base(|ref mut cpu, lhs, rhs| {
                                     cpu.registers.y = lhs;
                                     Cpy::execute(cpu, rhs);
                                 });
}

#[test]
fn less_than_flag_check() {
    less_than_flag_check_base(|ref mut cpu, lhs, rhs| {
                                  cpu.registers.y = lhs;
                                  Cpy::execute(cpu, rhs);
                              });
}
