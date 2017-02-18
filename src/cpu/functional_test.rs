use cpu::*;
use memory::*;
use std::fs::File;
use std::io::Read;

const PC_START: u16 = 0x400;
const MAX_CYCLES: u64 = 100000000;

#[test]
fn functional_test() {
    let mut f = File::open("test_roms/6502_functional_test.bin").unwrap();
    let mut rom = Vec::<u8>::new();
    f.read_to_end(&mut rom).unwrap();
    let mut mem = SimpleMemory::new();
    mem.store_many(PC_START, &rom);
    let mut cpu = Cpu::new_init_pc(mem, PC_START);
    let mut last_pc = PC_START;

    loop {
        cpu.step().unwrap();
        // Prevent endless loop
        if cpu.cycles > MAX_CYCLES {
            assert!(false, "Took too many cycles to complete");
        }

        if cpu.registers.pc == 0x3367 {
            // Success!
            return;
        }

        if last_pc == cpu.registers.pc {
            assert!(false, "Trap detected");
        }

        last_pc = cpu.registers.pc;
    }
}
