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
    fn write_address(&self, val: u8);
    fn read_data_increment_address(&self) -> Result<u8>;
    fn read_data(&self) -> Result<u8>;
    fn write_data_increment_address(&mut self, val: u8) -> Result<()>;
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
    fn write_address(&self, val: u8) {
        self.write_address(val)
    }

    fn read_data_increment_address(&self) -> Result<u8> {
        self.read_data_increment_address()
    }

    fn read_data(&self) -> Result<u8> {
        self.read_data()
    }

    fn write_data_increment_address(&mut self, val: u8) -> Result<()> {
        self.write_data_increment_address(val)
    }
    fn clear_latch(&self) {
        self.clear_latch()
    }
}

impl VramBase {
    pub fn write_address(&self, val: u8) {
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

    pub fn read_data_increment_address(&self) -> Result<u8> {
        let val = self.read_data()?;
        self.address.set(self.address.get() + 1);
        Ok(val)
    }

    pub fn read_data(&self) -> Result<u8> {
        let addr = self.address.get();
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

    pub fn write_data_increment_address(&mut self, val: u8) -> Result<()> {
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

    pub fn clear_latch(&self) {
        self.latch_state.set(LatchState::WriteHighByte)
    }

    #[cfg(test)]
    pub fn address(&self) -> u16 {
        self.address.get()
    }
}
