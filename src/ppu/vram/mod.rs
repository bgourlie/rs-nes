use std::cell::Cell;

#[derive(Copy, Clone)]
enum LatchState {
    WriteHighByte,
    WriteLowByte,
}

pub struct Vram {
    address: Cell<u16>,
    latch_state: Cell<LatchState>,
    pattern_tables: [u8; 0x2000], // TODO: mapper
    name_tables: [u8; 0x1000],
    palette: [u8; 0x20],
}

impl Clone for Vram {
    fn clone(&self) -> Self {
        let pattern_tables = self.pattern_tables;
        let name_tables = self.name_tables;
        let palette = self.palette;

        Vram {
            address: self.address.clone(),
            latch_state: self.latch_state.clone(),
            pattern_tables: pattern_tables,
            name_tables: name_tables,
            palette: palette,
        }
    }
}

impl Vram {
    pub fn new() -> Self {
        Vram {
            address: Cell::new(0),
            latch_state: Cell::new(LatchState::WriteHighByte),
            pattern_tables: [0; 0x2000],
            name_tables: [0; 0x1000],
            palette: [0; 0x20],
        }
    }

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

    pub fn read_data_increment_addr(&self) -> u8 {
        let val = self.read_data();
        self.address.set(self.address.get() + 1);
        val
    }

    pub fn read_data(&self) -> u8 {
        let addr = self.address.get();
        let val = if addr < 0x2000 {
            self.pattern_tables[addr as usize]
        } else if addr < 0x3f00 {
            self.name_tables[addr as usize & 0x0fff]
        } else if addr < 0x4000 {
            self.palette[addr as usize & 0x1f]
        } else {
            panic!("Attempt to read invalid vram address: {}", addr)
        };
        val
    }

    pub fn write_data(&mut self, val: u8) {
        let addr = self.address.get();

        if addr < 0x2000 {
            self.pattern_tables[addr as usize] = val;
        } else if addr < 0x3f00 {
            self.name_tables[addr as usize & 0x0fff] = val;
        } else if addr < 0x4000 {
            self.palette[addr as usize & 0x1f] = val;
        } else {
            panic!("Attempt to write invalid vram address: {:0>4X}", addr)
        }

        self.address.set(addr + 1);
    }

    pub fn clear_latch(&self) {
        self.latch_state.set(LatchState::WriteHighByte)
    }

    #[cfg(test)]
    pub fn address(&self) -> u16 {
        self.address.get()
    }
}
