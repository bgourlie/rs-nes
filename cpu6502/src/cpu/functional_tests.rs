use crate::cpu::{test_fixture::TestInterconnect, *};
use std::{fs::File, io::Read};

const PC_START: u16 = 0x400;
const MAX_CYCLES: usize = 100000000;

#[test]
fn opcodes() {
    let mut f = File::open("../test_roms/6502_functional_test.bin").unwrap();
    let mut rom = Vec::<u8>::new();
    f.read_to_end(&mut rom).unwrap();
    let mem = TestInterconnect::default();
    let mut cpu = Cpu::new(mem, PC_START);
    cpu.interconnect.store_many(PC_START, &rom);
    let mut last_pc = PC_START;

    loop {
        cpu.step();
        // Prevent endless loop
        if cpu.interconnect.elapsed_cycles() > MAX_CYCLES {
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
    let mut f = File::open("../test_roms/6502_interrupt_test.bin").unwrap();
    let mut rom = Vec::<u8>::new();
    f.read_to_end(&mut rom).unwrap();
    let mem = TestInterconnect::default();
    let mut cpu = Cpu::new(mem, PC_START);
    cpu.interconnect.store_many(PC_START, &rom);
    let mut last_pc = PC_START;
    let mut previous_interrupt_probe = 0;

    loop {
        let interrupt_probe = cpu.read_memory(0xbffc);

        if interrupt_probe != previous_interrupt_probe {
            previous_interrupt_probe = interrupt_probe;

            if interrupt_probe & 0x2 > 0 {
                cpu.pending_interrupt = Interrupt::Nmi;
            }
        }

        if !cpu.registers.interrupt_disable_flag() && (interrupt_probe & 0x1) > 0 {
            cpu.pending_interrupt = Interrupt::Irq;
        }

        cpu.step();
        // Prevent endless loop
        if cpu.interconnect.elapsed_cycles() > MAX_CYCLES {
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
