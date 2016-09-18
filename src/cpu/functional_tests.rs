use std::fs::File;
use std::io::Read;

use cpu::*;
use memory::*;

const PC_START: u16 = 0x400;

#[test]
fn functional_test() {
    let mut f = File::open("test_roms/6502_functional_test.bin").unwrap();
    let mut rom = Vec::<u8>::new();
    let bytes_read = f.read_to_end(&mut rom).unwrap();
    assert!(bytes_read == 65536);
    let mut mem = SimpleMemory::new();
    mem.store_many(0, &rom);
    let mut cpu = Cpu6502::new(mem);
    let mut last_pc: u16 = PC_START;
    cpu.registers.pc = PC_START;

    loop {
        match cpu.step() {
            Ok(_) => {
                if cpu.registers.pc == 0x3399 {
                    // Success!
                    return;
                }

                if last_pc == cpu.registers.pc {
                    assert!(false);
                }

                last_pc = cpu.registers.pc;

            }
            Err(_) => {
                assert!(false);
            }
        }
    }
}
