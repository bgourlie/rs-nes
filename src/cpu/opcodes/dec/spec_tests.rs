use cpu::opcodes::inc_dec_tests_base::*;
use cpu::opcodes::OpCode;
use super::Dec;

#[test]
fn test1() {
    dec_base_1(|ref mut cpu, am| {
        Dec::execute_cycles(cpu, am);
    });
}

#[test]
fn test2() {
    dec_base_2(|ref mut cpu, am| {
        Dec::execute_cycles(cpu, am);
    });
}

#[test]
fn test3() {
    dec_base_3(|ref mut cpu, am| {
        Dec::execute_cycles(cpu, am);
    });
}
