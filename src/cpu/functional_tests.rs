use cpu::*;
use memory::*;
use std::fs::File;
use std::io::Read;

const PC_START: u16 = 0x400;
const MAX_CYCLES: u64 = 100000000;

#[test]
fn opcodes() {
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

        if last_pc == cpu.registers.pc {
            if cpu.registers.pc == 0x3367 {
                // Success!
                break;
            } else {
                assert!(false, "Trap detected");
            }
        }

        last_pc = cpu.registers.pc;
    }
}

#[test]
fn interrupts() {
    let mut f = File::open("test_roms/6502_interrupt_test.bin").unwrap();
    let mut rom = Vec::<u8>::new();
    f.read_to_end(&mut rom).unwrap();
    let mut mem = SimpleMemory::new();
    mem.store_many(PC_START, &rom);
    let mut cpu = Cpu::new_init_pc(mem, PC_START);
    let mut last_pc = PC_START;
    let mut previous_interrupt_probe = 0;

    loop {
        let interrupt_probe = cpu.read_memory(0xbffc).unwrap();

        if interrupt_probe != previous_interrupt_probe {
            previous_interrupt_probe = interrupt_probe;

            if interrupt_probe & 0x2 > 0 {
                cpu.pending_interrupt = Interrupt::Nmi;
            }
        }

        if !cpu.registers.interrupt_disable_flag() && (interrupt_probe & 0x1) > 0 {
            cpu.pending_interrupt = Interrupt::Irq;
        }

        cpu.step().unwrap();
        // Prevent endless loop
        if cpu.cycles > MAX_CYCLES {
            assert!(false, "Took too many cycles to complete");
        }

        if last_pc == cpu.registers.pc {
            if cpu.registers.pc == 0x700 {
                // Success!
                break;
            } else {
                assert!(false, "Trap detected");
            }
        }

        last_pc = cpu.registers.pc;
    }
}
