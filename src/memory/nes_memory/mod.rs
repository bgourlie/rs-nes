use std::io::Write;
use seahash;
use rom::NesRom;
use ppu::Ppu;
use super::Memory;

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
    fn store(&mut self, address: u16, value: u8) {
        match address {
            0x0...0x1fff => self.ram[address as usize & 0x7ff] = value,
            0x2000...0x3fff => self.ppu.memory_mapped_register_write(address, value),
            _ => unimplemented!(),
        }
    }

    fn load(&self, address: u16) -> u8 {
        if address < 0x2000 {
            self.ram[address as usize & 0x7ff]
        } else if address < 0x8000 {
            0x0
        } else if self.rom.prg.len() > 16384 {
            self.rom.prg[address as usize & 0x7fff]
        } else {
            self.rom.prg[address as usize & 0x3fff]
        }
    }

    fn dump<T: Write>(&self, writer: &mut T) {
        writer.write(&self.ram).unwrap();

        // A bunch of stuff does actually live here that's not implemented yet
        writer.write(&[0_u8; 0x7800]).unwrap();

        if self.rom.prg.len() > 0x4000 {
            writer.write(&self.rom.prg).unwrap();
        } else {
            // PRG is mirrored if only one bank
            writer.write(&self.rom.prg).unwrap();
            writer.write(&self.rom.prg).unwrap();
        }
    }

    fn hash(&self) -> u64 {
        // Hashing just the ram will suffice for now...
        seahash::hash(&self.ram)
    }
}
