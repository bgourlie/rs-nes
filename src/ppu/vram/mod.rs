use super::control_register::IncrementAmount;
use errors::*;
use ppu::write_latch::LatchState;
use rom::NesRom;
use std::cell::Cell;

#[cfg(test)]
mod spec_tests;

pub trait Vram {
    fn new(rom: NesRom) -> Self;
    fn write_ppu_addr(&self, latch_state: LatchState);
    fn write_ppu_data(&mut self, val: u8, inc_amount: IncrementAmount) -> Result<()>;
    fn read_ppu_data(&self, inc_amount: IncrementAmount) -> Result<u8>;
    fn ppu_data(&self) -> Result<u8>;
    fn read(&self, addr: u16) -> Result<u8>;
    fn addr(&self) -> u16;
    fn scroll_write(&self, latch_state: LatchState);
    fn control_write(&self, val: u8);
}

pub struct VramBase {
    address: Cell<u16>,
    name_tables: [u8; 0x1000],
    palette: [u8; 0x20],
    rom: NesRom, // TODO: mapper
    ppu_data_buffer: Cell<u8>,
    t: Cell<u16>,
}

impl Vram for VramBase {
    fn new(rom: NesRom) -> Self {
        VramBase {
            address: Cell::new(0),
            name_tables: [0; 0x1000],
            palette: [0; 0x20],
            rom: rom,
            ppu_data_buffer: Cell::new(0),
            t: Cell::new(0),
        }
    }

    fn write_ppu_addr(&self, latch_state: LatchState) {
        // Addresses greater than 0x3fff are mirrored down
        match latch_state {
            LatchState::FirstWrite(val) => {
                //self.address.set((addr & 0x80ff) | ((val as u16) & 0x3f) << 8);

                // t: ..FEDCBA ........ = d: ..FEDCBA
                // t: .X...... ........ = 0
                let t = self.t.get();
                let t_preserved = t & 0b1100_0000_00000000;
                let new_t = (t_preserved | ((val as u16 & 0b0011_1111) << 8)) &
                            0b1011_1111_1111_1111;
                self.t.set(new_t);
            }
            LatchState::SecondWrite(val) => {
                // t: ....... HGFEDCBA = d: HGFEDCBA
                // v                   = t
                let new_t = self.t.get() & val as u16;
                self.t.set(new_t);
                self.address.set(new_t);
            }
        }
    }

    fn read_ppu_data(&self, inc_amount: IncrementAmount) -> Result<u8> {
        let val = self.ppu_data()?;
        match inc_amount {
            IncrementAmount::One => self.address.set(self.address.get() + 1),
            IncrementAmount::ThirtyTwo => self.address.set(self.address.get() + 32),
        }
        Ok(val)
    }

    fn ppu_data(&self) -> Result<u8> {
        let addr = self.address.get();
        let val = self.read(addr)?;

        // TODO: Tests for this buffering behavior
        if addr < 0x3f00 {
            let buffered_val = self.ppu_data_buffer.get();
            self.ppu_data_buffer.set(val);
            Ok(buffered_val)
        } else {
            Ok(val)
        }
    }

    fn write_ppu_data(&mut self, val: u8, inc_amount: IncrementAmount) -> Result<()> {
        let addr = self.address.get();

        if addr < 0x2000 {
            self.rom.chr[addr as usize] = val;
        } else if addr < 0x3f00 {
            self.name_tables[addr as usize & 0x0fff] = val;
        } else if addr < 0x4000 {
            let addr = addr as usize & 0x1f;
            // Certain sprite addresses are mirrored back into background addresses
            let addr = match addr & 0xf {
                0x0 => 0x0,
                0x4 => 0x4,
                0x8 => 0x8,
                0xc => 0xc,
                _ => addr,
            };
            self.palette[addr] = val;
        } else {
            let message = "Invalid write".to_owned();
            bail!(ErrorKind::Crash(CrashReason::InvalidVramAccess(message, addr)));
        }

        match inc_amount {
            IncrementAmount::One => self.address.set(self.address.get() + 1),
            IncrementAmount::ThirtyTwo => self.address.set(self.address.get() + 32),
        }
        Ok(())
    }

    #[inline(always)]
    fn read(&self, addr: u16) -> Result<u8> {
        let val = if addr < 0x2000 {
            self.rom.chr[addr as usize]
        } else if addr < 0x3f00 {
            self.name_tables[addr as usize & 0x0fff]
        } else if addr < 0x4000 {
            let addr = addr as usize & 0x1f;
            // Certain sprite addresses are mirrored back into background addresses
            let addr = match addr & 0xf {
                0x0 => 0x0,
                0x4 => 0x4,
                0x8 => 0x8,
                0xc => 0xc,
                _ => addr,
            };
            self.palette[addr]
        } else {
            let message = "Invalid read".to_owned();
            bail!(ErrorKind::Crash(CrashReason::InvalidVramAccess(message, addr)));
        };
        Ok(val)
    }

    fn addr(&self) -> u16 {
        self.address.get()
    }
    fn scroll_write(&self, latch_state: LatchState) {
        match latch_state {
            LatchState::FirstWrite(val) => {
                // t: ....... ...HGFED = d: HGFED...
                let t = self.t.get() & 0b1111_1111_1110_0000;
                self.t.set(((val as u16 & 0b1111_1000) >> 3) | t);
            }
            LatchState::SecondWrite(val) => {
                // t: CBA..HG FED..... = d: HGFEDCBA
                let t = self.t.get() & 0b0000_1100_0001_1111;
                let cba_mask = (val as u16 & 0b0000_0111) << 12;
                let hgfed_mask = (val as u16 & 0b1111_1000) << 2;
                self.t.set((cba_mask | hgfed_mask) | t);
            }
        }
    }
    fn control_write(&self, val: u8) {
        // t: ...BA.. ........ = d: ......BA
        let t = self.t.get() & 0b0111_0011_1111_1111;
        let new_t = ((val as u16 & 0b0011) << 10) | t;
        self.t.set(new_t);
    }
}
