use cpu::opcodes::addressing_mode::Implied;
use cpu::opcodes::inc_dec_tests_base::*;
use cpu::opcodes::OpCode;
use super::Dey;

#[test]
fn test1() {
    dec_base_1(|ref mut cpu, val| {
        cpu.registers.y = val;
        Dey::execute_cycles(cpu, Implied);
        cpu.registers.y
    });
}

#[test]
fn test2() {
    dec_base_2(|ref mut cpu, val| {
        cpu.registers.y = val;
        Dey::execute_cycles(cpu, Implied);
        cpu.registers.y
    });
}

#[test]
fn test3() {
    dec_base_3(|ref mut cpu, val| {
        cpu.registers.y = val;
        Dey::execute_cycles(cpu, Implied);
        cpu.registers.y
    });
}
