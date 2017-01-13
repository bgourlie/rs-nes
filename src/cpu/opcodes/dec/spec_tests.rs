use cpu::opcodes::OpCode;
use cpu::opcodes::addressing::testing::WriterAddressingMode;
use cpu::opcodes::dec::Dec;
use cpu::opcodes::inc_dec_tests_base::*;

#[test]
fn test1() {
    dec_base_1(|ref mut cpu, val| {
        let am = WriterAddressingMode::with_read_value(val);
        let write_ref = am.write_ref();
        Dec::execute_cycles(cpu, am);
        write_ref.get()
    });
}

#[test]
fn test2() {
    dec_base_2(|ref mut cpu, val| {
        let am = WriterAddressingMode::with_read_value(val);
        let write_ref = am.write_ref();
        Dec::execute_cycles(cpu, am);
        write_ref.get()
    });
}

#[test]
fn test3() {
    dec_base_3(|ref mut cpu, val| {
        let am = WriterAddressingMode::with_read_value(val);
        let write_ref = am.write_ref();
        Dec::execute_cycles(cpu, am);
        write_ref.get()
    });
}
