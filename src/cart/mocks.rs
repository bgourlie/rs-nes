use cart::Cart;
use rom::NesRom;
use rom::{CHR_BANK_SIZE, PRG_BANK_SIZE};

pub struct CartMock {
    pub prg: [u8; PRG_BANK_SIZE],
    pub chr: [u8; CHR_BANK_SIZE],
}

impl Default for CartMock {
    fn default() -> Self {
        CartMock {
            prg: [0; PRG_BANK_SIZE],
            chr: [0; CHR_BANK_SIZE],
        }
    }
}

impl Cart for CartMock {
    fn new(_: NesRom) -> Result<Self, &'static str> {
        unimplemented!()
    }

    fn read_prg(&self, addr: u16) -> u8 {
        self.prg[addr as usize]
    }

    fn write_prg(&mut self, addr: u16, value: u8) {
        self.prg[addr as usize] = value
    }

    fn read_chr(&self, addr: u16) -> u8 {
        self.chr[addr as usize]
    }

    fn write_chr(&mut self, addr: u16, value: u8) {
        self.chr[addr as usize] = value
    }
}
