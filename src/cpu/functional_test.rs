use std::cell::Cell;
use std::fs::File;
use std::io::Read;

use cpu::*;
use memory::*;

const PC_START: u16 = 0x400;

// TODO: Verify that this is the number of cycles that the test ROM is expected to take
const EXPECTED_CYCLES: u64 = 80869309;


#[test]
fn functional_test() {
    let cycles = Cell::new(0);
    let mut f = File::open("test_roms/6502_functional_test.bin").unwrap();
    let mut rom = Vec::<u8>::new();
    let bytes_read = f.read_to_end(&mut rom).unwrap();
    assert!(bytes_read == 65536);
    let mut mem = SimpleMemory::new();
    mem.store_many(0, &rom);
    let mut cpu = Cpu::new(mem);
    let mut last_pc = PC_START;
    cpu.registers.pc = PC_START;

    loop {
        cpu.step(|_: &Cpu<SimpleMemory>| cycles.set(cycles.get() + 1));

        // Prevent endless loop
        if cycles.get() > EXPECTED_CYCLES {
            assert!(false, "Took too many cycles to complete");
        }

        if cpu.registers.pc == 0x3399 {
            assert_eq!(EXPECTED_CYCLES, cycles.get());

            // Success!
            return;
        }

        if last_pc == cpu.registers.pc {
            assert!(false, "Trap detected");
        }

        last_pc = cpu.registers.pc;
    }
}
