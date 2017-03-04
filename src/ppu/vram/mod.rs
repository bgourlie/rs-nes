use errors::*;
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

pub trait Vram: Default {
    fn write_ppu_addr(&self, val: u8);
    fn write_ppu_data(&mut self, val: u8) -> Result<()>;
    fn read_ppu_data(&self) -> Result<u8>;
    fn ppu_data(&self) -> Result<u8>;
    fn read(&self, addr: u16) -> Result<u8>;
    fn clear_latch(&self);
}

pub struct VramBase {
    address: Cell<u16>,
    latch_state: Cell<LatchState>,
    pattern_tables: [u8; 0x2000], // TODO: mapper
    name_tables: [u8; 0x1000],
    palette: [u8; 0x20],
}

impl Default for VramBase {
    fn default() -> Self {
        VramBase {
            address: Cell::new(0),
            latch_state: Cell::new(LatchState::default()),
            pattern_tables: [0; 0x2000],
            name_tables: [0; 0x1000],
            palette: [0; 0x20],
        }
    }
}

impl Vram for VramBase {
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

    fn read_ppu_data(&self) -> Result<u8> {
        let val = self.ppu_data()?;
        self.address.set(self.address.get() + 1);
        Ok(val)
    }

    fn ppu_data(&self) -> Result<u8> {
        let addr = self.address.get();
        self.read(addr)
    }

    fn write_ppu_data(&mut self, val: u8) -> Result<()> {
        let addr = self.address.get();

        if addr < 0x2000 {
            self.pattern_tables[addr as usize] = val;
        } else if addr < 0x3f00 {
            self.name_tables[addr as usize & 0x0fff] = val;
        } else if addr < 0x4000 {
            self.palette[addr as usize & 0x1f] = val;
        } else {
            bail!(ErrorKind::Crash(CrashReason::InvalidVramAccess(addr)));
        }

        self.address.set(addr + 1);
        Ok(())
    }

    fn clear_latch(&self) {
        self.latch_state.set(LatchState::WriteHighByte)
    }

    fn read(&self, addr: u16) -> Result<u8> {
        let val = if addr < 0x2000 {
            self.pattern_tables[addr as usize]
        } else if addr < 0x3f00 {
            self.name_tables[addr as usize & 0x0fff]
        } else if addr < 0x4000 {
            self.palette[addr as usize & 0x1f]
        } else {
            bail!(ErrorKind::Crash(CrashReason::InvalidVramAccess(addr)));
        };
        Ok(val)
    }
}
