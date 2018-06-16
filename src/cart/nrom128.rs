use cart::Cart;
use rom::{NesRom, CHR_BANK_SIZE, PRG_BANK_SIZE};

pub struct Nrom128 {
    prg_rom: [u8; PRG_BANK_SIZE],
    chr_rom: [u8; CHR_BANK_SIZE],
}

impl Nrom128 {
    pub fn new(rom: NesRom) -> Result<Self, &'static str> {
        if rom.prg.len() != PRG_BANK_SIZE {
            println!("{}", rom.prg.len());
            Err("Unexpected PRG ROM size: {}")
        } else if rom.chr.len() != CHR_BANK_SIZE {
            Err("Unexpected CHR ROM size")
        } else {
            let mut cart = Nrom128 {
                prg_rom: [0; PRG_BANK_SIZE],
                chr_rom: [0; CHR_BANK_SIZE],
            };
            cart.prg_rom.copy_from_slice(&rom.prg);
            cart.chr_rom.copy_from_slice(&rom.chr);
            Ok(cart)
        }
    }
}

impl Cart for Nrom128 {
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
