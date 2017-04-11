use super::Cmp;
use cpu::opcodes::OpCode;
use cpu::opcodes::compare_tests_base::*;

#[test]
fn equal_flag_check() {
    equal_flag_check_base(|ref mut cpu, lhs, rhs| {
                              cpu.registers.acc = lhs;
                              Cmp::execute(cpu, rhs);
                          });
}

#[test]
fn greater_than_flag_check() {
    greater_than_flag_check_base(|ref mut cpu, lhs, rhs| {
                                     cpu.registers.acc = lhs;
                                     Cmp::execute(cpu, rhs);
                                 });
}

#[test]
fn less_than_flag_check() {
    less_than_flag_check_base(|ref mut cpu, lhs, rhs| {
                                  cpu.registers.acc = lhs;
                                  Cmp::execute(cpu, rhs);
                              });
}
