#[cfg(test)]
pub mod mocks;

use rom::{NesRom, CHR_BANK_SIZE, PRG_BANK_SIZE};

pub trait Cart: Sized {
    fn new(rom: NesRom) -> Result<Self, &'static str>;
    fn read_prg(&self, addr: u16) -> u8;
    fn write_prg(&mut self, addr: u16, value: u8);
    fn read_chr(&self, addr: u16) -> u8;
    fn write_chr(&mut self, addr: u16, value: u8);
}

pub struct Nrom256Cart {
    prg_rom: [u8; PRG_BANK_SIZE * 2],
    chr_rom: [u8; CHR_BANK_SIZE],
}

impl Cart for Nrom256Cart {
    fn new(rom: NesRom) -> Result<Self, &'static str> {
        if rom.prg.len() != PRG_BANK_SIZE * 2 {
            println!("{}", rom.prg.len());
            Err("Unexpected PRG ROM size: {}")
        } else if rom.chr.len() != CHR_BANK_SIZE {
            Err("Unexpected CHR ROM size")
        } else {
            let mut cart = Nrom256Cart {
                prg_rom: [0; PRG_BANK_SIZE * 2],
                chr_rom: [0; CHR_BANK_SIZE],
            };
            cart.prg_rom.copy_from_slice(&rom.prg);
            cart.chr_rom.copy_from_slice(&rom.chr);
            Ok(cart)
        }
    }

    fn read_prg(&self, addr: u16) -> u8 {
        debug_assert!(addr >= 0x8000);
        self.prg_rom[addr as usize & 0x7fff]
    }

    fn write_prg(&mut self, _: u16, _: u8) {}

    fn read_chr(&self, addr: u16) -> u8 {
        self.chr_rom[addr as usize]
    }

    fn write_chr(&mut self, _: u16, _: u8) {}
}

pub struct Nrom128Cart {
    prg_rom: [u8; PRG_BANK_SIZE],
    chr_rom: [u8; CHR_BANK_SIZE],
}

impl Cart for Nrom128Cart {
    fn new(rom: NesRom) -> Result<Self, &'static str> {
        if rom.prg.len() != PRG_BANK_SIZE {
            println!("{}", rom.prg.len());
            Err("Unexpected PRG ROM size: {}")
        } else if rom.chr.len() != CHR_BANK_SIZE {
            Err("Unexpected CHR ROM size")
        } else {
            let mut cart = Nrom128Cart {
                prg_rom: [0; PRG_BANK_SIZE],
                chr_rom: [0; CHR_BANK_SIZE],
            };
            cart.prg_rom.copy_from_slice(&rom.prg);
            cart.chr_rom.copy_from_slice(&rom.chr);
            Ok(cart)
        }
    }

    fn read_prg(&self, addr: u16) -> u8 {
        debug_assert!(addr >= 0x8000);
        self.prg_rom[addr as usize & 0x3fff]
    }

    fn write_prg(&mut self, _: u16, _: u8) {}

    fn read_chr(&self, addr: u16) -> u8 {
        self.chr_rom[addr as usize]
    }

    fn write_chr(&mut self, _: u16, _: u8) {}
}
