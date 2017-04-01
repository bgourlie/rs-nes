use cpu::opcodes::OpCode;
use cpu::opcodes::addressing::Implied;
use cpu::opcodes::inc_dec_tests_base::*;
use cpu::opcodes::inx::Inx;

#[test]
fn test1() {
    inc_base_1(|ref mut cpu, val| {
                   cpu.registers.x = val;
                   Inx::execute(cpu, Implied).unwrap();
                   cpu.registers.x
               });
}

#[test]
fn test2() {
    inc_base_2(|ref mut cpu, val| {
                   cpu.registers.x = val;
                   Inx::execute(cpu, Implied).unwrap();
                   cpu.registers.x
               });
}

#[test]
fn test3() {
    inc_base_3(|ref mut cpu, val| {
                   cpu.registers.x = val;
                   Inx::execute(cpu, Implied).unwrap();
                   cpu.registers.x
               });
}
