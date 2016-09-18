use std::fs::File;
use std::io::Read;

use cpu::*;
use memory::*;

const PC_START: u16 = 0x400;

// TODO: Verify that this is the number of cycles that the test ROM is expected to take
const EXPECTED_CYCLES: u64 = 80869309;

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
        cpu.step();

        // Prevent endless loop
        if cpu.cycles > EXPECTED_CYCLES {
            assert!(false, "Took too many cycles to complete");
        }

        if cpu.registers.pc == 0x3399 {
            assert_eq!(EXPECTED_CYCLES, cpu.cycles);

            // Success!
            return;
        }

        if last_pc == cpu.registers.pc {
            assert!(false, "Trap detected");
        }

        last_pc = cpu.registers.pc;
    }
}
