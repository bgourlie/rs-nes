use crate::{
    cart::Cart,
    rom::{NesRom, CHR_BANK_SIZE, PRG_BANK_SIZE},
};

pub struct Uxrom {
    prg_bank: Vec<u8>,
    chr_rom: [u8; CHR_BANK_SIZE],
    bank_select: u8,
    last_bank: u8,
}

impl Uxrom {
    pub fn new(rom: &NesRom) -> Result<Self, &'static str> {
        if rom.prg.len() != PRG_BANK_SIZE * 8 {
            println!("{}", rom.prg.len());
            Err("Unexpected PRG ROM size")
        } else if rom.chr_rom_banks > 0 && rom.chr.len() != CHR_BANK_SIZE {
            Err("Unexpected CHR ROM size")
        } else {
            let mut cart = Uxrom {
                prg_bank: vec![0; rom.prg.len()],
                chr_rom: [0; CHR_BANK_SIZE],
                bank_select: 0,
                last_bank: rom.prg_rom_banks - 1,
            };

            cart.prg_bank.copy_from_slice(&rom.prg);

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
        let resolved_addr = resolve_prg_addr(addr, self.bank_select, self.last_bank);
        self.prg_bank[resolved_addr]
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

fn resolve_prg_addr(addr: u16, bank_select: u8, last_bank: u8) -> usize {
    let addr = addr as usize;
    let normalized_addr = addr & 0x3fff;
    let bank_select = bank_select as usize;
    let last_bank = last_bank as usize;

    if addr & 0xc000 == 0x8000 {
        (bank_select << 14) | normalized_addr
    } else {
        (last_bank << 14) | normalized_addr
    }
}

#[test]
fn test_resolve() {
    let addr = resolve_prg_addr(0x8000, 0, 7);
    assert_eq!(0, addr);

    let addr = resolve_prg_addr(0xbfff, 0, 7);
    assert_eq!(0x3fff, addr);

    let addr = resolve_prg_addr(0x8000, 1, 7);
    assert_eq!(0x4000, addr);

    let addr = resolve_prg_addr(0xbfff, 1, 7);
    assert_eq!(0x7fff, addr);

    let addr = resolve_prg_addr(0x8000, 2, 7);
    assert_eq!(0x8000, addr);

    let addr = resolve_prg_addr(0xbfff, 2, 7);
    assert_eq!(0xbfff, addr);

    let addr = resolve_prg_addr(0x8000, 3, 7);
    assert_eq!(0xc000, addr);

    let addr = resolve_prg_addr(0xbfff, 3, 7);
    assert_eq!(0xffff, addr);

    let addr = resolve_prg_addr(0x8000, 4, 7);
    assert_eq!(0x10000, addr);

    let addr = resolve_prg_addr(0xbfff, 4, 7);
    assert_eq!(0x13fff, addr);

    let addr = resolve_prg_addr(0x8000, 5, 7);
    assert_eq!(0x14000, addr);

    let addr = resolve_prg_addr(0xbfff, 5, 7);
    assert_eq!(0x17fff, addr);

    let addr = resolve_prg_addr(0x8000, 6, 7);
    assert_eq!(0x18000, addr);

    let addr = resolve_prg_addr(0xbfff, 6, 7);
    assert_eq!(0x1bfff, addr);

    let addr = resolve_prg_addr(0x8000, 7, 7);
    assert_eq!(0x1c000, addr);

    let addr = resolve_prg_addr(0xbfff, 7, 7);
    assert_eq!(0x1ffff, addr);

    // Any address 0xc000 and up should always resolve to the last bank
    let addr = resolve_prg_addr(0xc000, 0, 7);
    assert_eq!(0x1c000, addr);

    let addr = resolve_prg_addr(0xc000, 0, 6);
    assert_eq!(0x18000, addr);

    let addr = resolve_prg_addr(0xffff, 0, 5);
    assert_eq!(0x17fff, addr);
}
