use cart::Cart;
use rom::{NesRom, CHR_BANK_SIZE, PRG_BANK_SIZE};

pub struct Uxrom {
    prg_bank: [[u8; PRG_BANK_SIZE]; 8],
    chr_rom: [u8; CHR_BANK_SIZE],
    bank_select: u8,
}

impl Uxrom {
    pub fn new(rom: NesRom) -> Result<Self, &'static str> {
        if rom.prg.len() != PRG_BANK_SIZE * 8 {
            println!("{}", rom.prg.len());
            Err("Unexpected PRG ROM size")
        } else if rom.chr_rom_banks > 0 && rom.chr.len() != CHR_BANK_SIZE {
            Err("Unexpected CHR ROM size")
        } else {
            let mut cart = Uxrom {
                prg_bank: [[0; PRG_BANK_SIZE]; 8],
                chr_rom: [0; CHR_BANK_SIZE],
                bank_select: 0,
            };

            for i in 0..8 {
                let start_offset = i * PRG_BANK_SIZE;
                let end_offset = start_offset + PRG_BANK_SIZE;
                cart.prg_bank[i].copy_from_slice(&rom.prg[start_offset..end_offset]);
            }

            if rom.chr_rom_banks > 0 {
                cart.chr_rom.copy_from_slice(&rom.chr);
            }

            Ok(cart)
        }
    }
}

impl Cart for Uxrom {
    fn read_prg(&self, addr: u16) -> u8 {
        debug_assert!(addr >= 0x8000);
        let addr = addr as usize;
        if addr & 0xc000 == 0x8000 {
            let bank_select = self.bank_select as usize;
            self.prg_bank[bank_select][addr & 0x3fff]
        } else {
            self.prg_bank[7][addr & 0x3fff]
        }
    }

    fn write_prg(&mut self, addr: u16, value: u8) {
        debug_assert!(addr >= 0x8000);
        self.bank_select = value & 0x0f
    }

    fn read_chr(&self, addr: u16) -> u8 {
        self.chr_rom[addr as usize]
    }

    fn write_chr(&mut self, addr: u16, value: u8) {
        self.chr_rom[addr as usize] = value
    }
}
