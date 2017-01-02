use cpu::opcodes::addressing_mode::test_addressing_modes::WriterAddressingMode;
use cpu::opcodes::inc_dec_tests_base::*;
use cpu::opcodes::OpCode;
use super::Dec;

#[test]
fn test1() {
    dec_base_1(|ref mut cpu, val| {
        let am = WriterAddressingMode::new(val);
        let value_ref = am.value_ref();
        Dec::execute_cycles(cpu, am);
        value_ref.get()
    });
}

#[test]
fn test2() {
    dec_base_2(|ref mut cpu, val| {
        let am = WriterAddressingMode::new(val);
        let value_ref = am.value_ref();
        Dec::execute_cycles(cpu, am);
        value_ref.get()
    });
}

#[test]
fn test3() {
    dec_base_3(|ref mut cpu, val| {
        let am = WriterAddressingMode::new(val);
        let value_ref = am.value_ref();
        Dec::execute_cycles(cpu, am);
        value_ref.get()
    });
}
