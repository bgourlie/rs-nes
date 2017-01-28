use cpu::*;
use memory::*;
use std::cell::Cell;
use std::fs::File;
use std::io::Read;

const PC_START: u16 = 0x400;
const MAX_CYCLES: u64 = 100000000;

#[test]
fn functional_test() {
    let cycles = Cell::new(0);
    let mut f = File::open("test_roms/6502_functional_test.bin").unwrap();
    let mut rom = Vec::<u8>::new();
    let bytes_read = f.read_to_end(&mut rom).unwrap();
    assert!(bytes_read == 65536);
    let mut mem = SimpleMemory::new();
    mem.store_many(0, &rom);
    let mut cpu = Cpu::new(mem, PC_START);
    let mut last_pc = PC_START;

    loop {
        cpu.step(|_: &Cpu<SimpleMemory>| cycles.set(cycles.get() + 1));

        // Prevent endless loop
        if cycles.get() > MAX_CYCLES {
            assert!(false, "Took too many cycles to complete");
        }

        if cpu.registers.pc == 0x3399 {
            // Success!
            return;
        }

        if last_pc == cpu.registers.pc {
            assert!(false, "Trap detected");
        }

        last_pc = cpu.registers.pc;
    }
}
