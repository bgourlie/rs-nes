#![allow(dead_code)]

use std::fs::File;
use std::io::Read;

mod constants;
mod memory;
mod cpu;
mod ppu;
mod rom_loader;

use cpu::Cpu6502;
use memory::*;
use rom_loader::NesRom;

const PC_START: u16 = 0x400;
const DUMP_FILE: &'static str = "/Users/brian/Desktop/6502dump.bin";

fn main() {
    let rom = NesRom::read("/Users/brian/Desktop/roms/Super Mario Bros. 3 (U) (PRG1) [!].nes")
                  .unwrap();

    println!("ROM Format: {:?}\nVideo Standard: {:?}\nMapper: {}\nMirroring: {:?}\nPRG-ROM \
              banks: {}\nPRG-RAM banks: {}\nCHR-ROM banks: {}\nHas SRAM: {}\nHas trainer: {}\n",
             rom.format,
             rom.video_standard,
             rom.mapper,
             rom.mirroring,
             rom.prg_rom_banks,
             rom.prg_ram_banks,
             rom.chr_rom_banks,
             rom.has_sram,
             rom.has_trainer);

    return;

    let mut f = File::open("test_roms/6502_functional_test.bin").unwrap();

    let mut rom = Vec::<u8>::new();
    let bytes_read = f.read_to_end(&mut rom).unwrap();
    if bytes_read != 65536 {
        panic!("expected 16kb of data");
    }

    let mut mem = SimpleMemory::new();
    mem.store_many(0, &rom);
    let mut memory: Box<Memory> = Box::new(mem);
    {
        let mut cpu = Cpu6502::new(&mut *memory);
        cpu.registers.pc = PC_START;
        let mut last_pc: u16 = PC_START;

        loop {
            match cpu.step() {
                Ok(instr) => {
                    if cpu.registers.pc == 0x3399 {
                        println!("******* SUCCESS ********");
                        return;
                    }

                    if last_pc == cpu.registers.pc {
                        println!("{} {} cyc: {}",
                                 instr.to_string(),
                                 cpu.registers.to_string(),
                                 cpu.cycles);
                        println!("Detected Trap");
                        break;
                    }

                    last_pc = cpu.registers.pc;

                    if cpu.cycles % 1000000 == 0 {
                        println!("{} cycles", cpu.cycles);
                    }
                }
                Err(msg) => {
                    println!("{}", msg);
                    break;
                }
            }
        }
    }

    // If we get here, we detected a trap.
    memory.dump(DUMP_FILE);
}
