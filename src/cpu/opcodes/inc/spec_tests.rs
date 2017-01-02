use cpu::opcodes::inc_dec_tests_base::*;
use cpu::opcodes::OpCode;
use super::Inc;

#[test]
fn test1() {
    inc_base_1(|ref mut cpu, am| {
        Inc::execute_cycles(cpu, am);
    });
}

#[test]
fn test2() {
    inc_base_2(|ref mut cpu, am| {
        Inc::execute_cycles(cpu, am);
    });
}

#[test]
fn test3() {
    inc_base_3(|ref mut cpu, am| {
        Inc::execute_cycles(cpu, am);
    });
}
