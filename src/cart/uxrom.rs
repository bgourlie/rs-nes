use cart::Cart;
use rom::{NesRom, CHR_BANK_SIZE, PRG_BANK_SIZE};

pub struct Uxrom {
    prg_bank_switchable_1: [u8; PRG_BANK_SIZE],
    prg_bank_switchable_2: [u8; PRG_BANK_SIZE],
    prg_bank_fixed: [u8; PRG_BANK_SIZE],
    chr_rom: [u8; CHR_BANK_SIZE],
    bank_2_select: bool,
}

impl Uxrom {
    pub fn new(rom: NesRom) -> Result<Self, &'static str> {
        if rom.prg.len() != PRG_BANK_SIZE * 3 {
            println!("{}", rom.prg.len());
            Err("Unexpected PRG ROM size: {}")
        } else if rom.chr.len() != CHR_BANK_SIZE {
            Err("Unexpected CHR ROM size")
        } else {
            let mut cart = Uxrom {
                prg_bank_switchable_1: [0; PRG_BANK_SIZE],
                prg_bank_switchable_2: [0; PRG_BANK_SIZE],
                prg_bank_fixed: [0; PRG_BANK_SIZE],
                chr_rom: [0; CHR_BANK_SIZE],
                bank_2_select: false,
            };
            cart.prg_bank_switchable_1
                .copy_from_slice(&rom.prg[0..PRG_BANK_SIZE]);
            cart.prg_bank_switchable_2
                .copy_from_slice(&rom.prg[PRG_BANK_SIZE..(PRG_BANK_SIZE * 2)]);
            cart.prg_bank_fixed
                .copy_from_slice(&rom.prg[(PRG_BANK_SIZE * 2)..(PRG_BANK_SIZE * 3)]);
            cart.chr_rom.copy_from_slice(&rom.chr);
            Ok(cart)
        }
    }
}

impl Cart for Uxrom {
    fn read_prg(&self, addr: u16) -> u8 {
        debug_assert!(addr >= 0x8000);
        let addr = addr as usize;
        if addr & 0xc000 == 0x8000 {
            if self.bank_2_select {
                self.prg_bank_switchable_2[addr & 0x3fff]
            } else {
                self.prg_bank_switchable_1[addr & 0x3fff]
            }
        } else {
            self.prg_bank_fixed[addr & 0x3fff]
        }
    }

    fn write_prg(&mut self, addr: u16, value: u8) {
        if addr & 0x8000 > 0 {
            self.bank_2_select = value & 0x0f > 0
        }
    }

    fn read_chr(&self, addr: u16) -> u8 {
        self.chr_rom[addr as usize]
    }

    fn write_chr(&mut self, _: u16, _: u8) {}
}
