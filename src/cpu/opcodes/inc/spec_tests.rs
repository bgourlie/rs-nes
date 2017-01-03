use cpu::opcodes::addressing::testing::WriterAddressingMode;
use cpu::opcodes::inc_dec_tests_base::*;
use cpu::opcodes::OpCode;
use super::Inc;

#[test]
fn test1() {
    inc_base_1(|ref mut cpu, val| {
        let am = WriterAddressingMode::new(val);
        let value_ref = am.value_ref();
        Inc::execute_cycles(cpu, am);
        value_ref.get()
    });
}

#[test]
fn test2() {
    inc_base_2(|ref mut cpu, val| {
        let am = WriterAddressingMode::new(val);
        let value_ref = am.value_ref();
        Inc::execute_cycles(cpu, am);
        value_ref.get()
    });
}

#[test]
fn test3() {
    inc_base_3(|ref mut cpu, val| {
        let am = WriterAddressingMode::new(val);
        let value_ref = am.value_ref();
        Inc::execute_cycles(cpu, am);
        value_ref.get()
    });
}
