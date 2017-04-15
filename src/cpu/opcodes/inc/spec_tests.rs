use cpu::opcodes::*;

use cpu::opcodes::am_test_utils::*;
use cpu::opcodes::inc::Inc;
use cpu::opcodes::inc_dec_tests_base::*;

#[test]
fn test1() {
    inc_base_1(|ref mut cpu, val| {
                   let am = WriterAddressingMode::with_read_value(val);
                   let write_ref = am.write_ref();
                   Inc::execute(cpu, am);
                   write_ref.get()
               });
}

#[test]
fn test2() {
    inc_base_2(|ref mut cpu, val| {
                   let am = WriterAddressingMode::with_read_value(val);
                   let write_ref = am.write_ref();
                   Inc::execute(cpu, am);
                   write_ref.get()
               });
}

#[test]
fn test3() {
    inc_base_3(|ref mut cpu, val| {
                   let am = WriterAddressingMode::with_read_value(val);
                   let write_ref = am.write_ref();
                   Inc::execute(cpu, am);
                   write_ref.get()
               });
}
