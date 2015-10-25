use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

use cpu::*;

const PC_START: u16 = 0x400;

#[test]
fn functional_test() {
    let mut f = File::open("test_roms/6502_functional_test.bin").unwrap();

    let mut vec = Vec::<u8>::new();
    let bytes_read = f.read_to_end(&mut vec).unwrap();
    assert!(bytes_read == 65536);

    let mut cpu = Cpu6502::new();
    cpu.load(0, &vec, PC_START);
    let mut last_pc: u16 = PC_START;

    loop {
        match cpu.step() {
            Ok(instr) => {
                if cpu.registers.pc == 0x3399 {
                    // Success!
                    return;
                }

                if last_pc == cpu.registers.pc {
                    assert!(false);
                }

                last_pc = cpu.registers.pc;

            }
            Err(msg) => {
                assert!(false);
            }
        }
    }
}
