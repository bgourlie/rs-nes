use super::control_register::IncrementAmount;
use errors::*;
use rom::NesRom;
use std::cell::Cell;

#[cfg(test)]
mod spec_tests;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum LatchState {
    WriteHighByte,
    WriteLowByte,
}

impl Default for LatchState {
    fn default() -> Self {
        LatchState::WriteHighByte
    }
}

pub trait Vram {
    fn new(rom: NesRom) -> Self;
    fn write_ppu_addr(&self, val: u8);
    fn write_ppu_data(&mut self, val: u8, inc_amount: IncrementAmount) -> Result<()>;
    fn read_ppu_data(&self, inc_amount: IncrementAmount) -> Result<u8>;
    fn ppu_data(&self) -> Result<u8>;
    fn read(&self, addr: u16) -> Result<u8>;
    fn clear_latch(&self);
}

pub struct VramBase {
    address: Cell<u16>,
    latch_state: Cell<LatchState>,
    name_tables: [u8; 0x1000],
    palette: [u8; 0x20],
    rom: NesRom, // TODO: mapper
}

impl Vram for VramBase {
    fn new(rom: NesRom) -> Self {
        VramBase {
            address: Cell::new(0),
            latch_state: Cell::new(LatchState::default()),
            name_tables: [0; 0x1000],
            palette: [0; 0x20],
            rom: rom,
        }
    }

    fn write_ppu_addr(&self, val: u8) {
        match self.latch_state.get() {
            LatchState::WriteHighByte => {
                let addr = self.address.get();
                self.address.set((addr & 0x00ff) | ((val as u16) << 8));
                self.latch_state.set(LatchState::WriteLowByte);
            }
            LatchState::WriteLowByte => {
                let addr = self.address.get();
                self.address.set((addr & 0xff00) | val as u16);
                self.latch_state.set(LatchState::WriteHighByte);
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
        self.read(addr)
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

    fn clear_latch(&self) {
        self.latch_state.set(LatchState::WriteHighByte)
    }

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
}
