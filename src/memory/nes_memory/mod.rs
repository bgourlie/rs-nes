use super::{Memory, TickAction};
use ppu::{Ppu, StepAction};
use rom::NesRom;

#[cfg(feature = "debugger")]
use seahash;
use std::io::Write;

pub struct NesMemory {
    ram: [u8; 0x800],
    rom: NesRom,
    ppu: Ppu,
}

impl NesMemory {
    pub fn new(rom: NesRom) -> Self {
        NesMemory {
            ram: [0_u8; 0x800],
            rom: rom,
            ppu: Ppu::new(),
        }
    }
}

impl Clone for NesMemory {
    fn clone(&self) -> Self {
        // This copies the array since the element type (u8) implements Copy
        let new_ram = self.ram;
        NesMemory {
            ram: new_ram,
            rom: self.rom.clone(),
            ppu: self.ppu.clone(),
        }
    }
}

// Currently NROM only
impl Memory for NesMemory {
    fn tick(&mut self) -> TickAction {
        let mut tick_action = TickAction::None;
        // For every CPU cycle, the PPU steps 3 times
        for _ in 0..3 {
            if self.ppu.step() == StepAction::VBlankNmi {
                tick_action = TickAction::Nmi;
            };
        }
        tick_action
    }

    fn store(&mut self, address: u16, value: u8) {
        match address {
            0x0...0x1fff => self.ram[address as usize & 0x7ff] = value,
            0x2000...0x3fff => self.ppu.memory_mapped_register_write(address, value),
            _ => panic!("Unimplemented: write to 0x{:0>4X}", address),
        }
    }

    fn load(&self, address: u16) -> u8 {
        match address {
            0x0...0x1fff => self.ram[address as usize & 0x7ff],
            0x2000...0x3fff => self.ppu.memory_mapped_register_read(address),
            0x4000...0x7fff => 0,
            _ => {
                if self.rom.prg.len() > 16384 {
                    self.rom.prg[address as usize & 0x7fff]
                } else {
                    self.rom.prg[address as usize & 0x3fff]
                }
            }
        }
    }

    fn dump<T: Write>(&self, writer: &mut T) {
        // 0x0 to 0x1fff
        for _ in 0..4 {
            writer.write(&self.ram).unwrap();
        }

        // 0x2000 to 0x3fff
        for _ in 0..1024 {
            self.ppu.dump_registers(writer);
        }

        // 0x4000 to 0x401f (APU and IO regs placeholder)
        writer.write(&[0_u8; 0x20]).unwrap();

        // Not sure what goes here, but gotta pad it for now to have correct ROM size
        writer.write(&[0_u8; 16352]).unwrap();

        // 0x6000 to 0xFFFF
        if self.rom.prg.len() > 0x4000 {
            writer.write(&self.rom.prg).unwrap();
        } else {
            // PRG is mirrored if only one bank
            writer.write(&self.rom.prg).unwrap();
            writer.write(&self.rom.prg).unwrap();
        }

    }

    #[cfg(feature = "debugger")]
    fn hash(&self) -> u64 {
        // Hashing just the ram will suffice for now...
        seahash::hash(&self.ram)
    }
}
