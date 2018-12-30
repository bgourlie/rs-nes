#[cfg(test)]
pub mod mocks;

#[cfg(test)]
mod spec_tests;

use crate::{
    cart::Cart,
    ppu::{control_register::IncrementAmount, write_latch::LatchState},
};
use std::cell::Cell;

pub trait IVram: Default {
    fn write_ppu_addr(&self, latch_state: LatchState);
    fn write_ppu_data<C: Cart>(&mut self, val: u8, inc_amount: IncrementAmount, cart: &mut C);
    fn read_ppu_data<C: Cart>(&self, inc_amount: IncrementAmount, cart: &C) -> u8;
    fn ppu_data<C: Cart>(&self, cart: &C) -> u8;
    fn read<C: Cart>(&self, addr: u16, cart: &C) -> u8;
    fn read_palette(&self, addr: u16) -> u8;
    fn addr(&self) -> u16;
    fn scroll_write(&self, latch_state: LatchState);
    fn control_write(&self, val: u8);
    fn coarse_x_increment(&self);
    fn fine_y_increment(&self);
    fn copy_horizontal_pos_to_addr(&self);
    fn copy_vertical_pos_to_addr(&self);
    fn fine_x(&self) -> u8;
}

pub struct Vram {
    address: Cell<u16>,
    name_tables: [u8; 0x1000],
    palette: [u8; 0x20],
    ppu_data_buffer: Cell<u8>,
    t: Cell<u16>,
    fine_x: Cell<u8>,
}

impl Default for Vram {
    fn default() -> Self {
        Vram {
            address: Cell::new(0),
            name_tables: [0; 0x1000],
            palette: [0; 0x20],
            ppu_data_buffer: Cell::new(0),
            t: Cell::new(0),
            fine_x: Cell::new(0),
        }
    }
}

impl IVram for Vram {
    fn write_ppu_addr(&self, latch_state: LatchState) {
        // Addresses greater than 0x3fff are mirrored down
        match latch_state {
            LatchState::FirstWrite(val) => {
                // t: ..FEDCBA ........ = d: ..FEDCBA
                // t: .X...... ........ = 0
                let t = self.t.get() & 0b1000_0000_1111_1111;
                self.t.set(((u16::from(val) & 0b0011_1111) << 8) | t)
            }
            LatchState::SecondWrite(val) => {
                // t: ....... HGFEDCBA = d: HGFEDCBA
                // v                   = t
                let t = u16::from(val) | (self.t.get() & 0b0111_1111_0000_0000);
                self.t.set(t);
                self.address.set(t);
            }
        }
    }

    fn write_ppu_data<C: Cart>(&mut self, val: u8, inc_amount: IncrementAmount, cart: &mut C) {
        let addr = self.address.get();

        if addr < 0x2000 {
            cart.write_chr(addr, val);
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
        }

        match inc_amount {
            IncrementAmount::One => self.address.set(self.address.get() + 1),
            IncrementAmount::ThirtyTwo => self.address.set(self.address.get() + 32),
        }
    }

    fn read_ppu_data<C: Cart>(&self, inc_amount: IncrementAmount, cart: &C) -> u8 {
        let val = self.ppu_data(cart);
        match inc_amount {
            IncrementAmount::One => self.address.set(self.address.get() + 1),
            IncrementAmount::ThirtyTwo => self.address.set(self.address.get() + 32),
        }
        val
    }

    fn ppu_data<C: Cart>(&self, cart: &C) -> u8 {
        let addr = self.address.get();
        let val = self.read(addr, cart);

        // When reading while the VRAM address is in the range 0-$3EFF (i.e., before the palettes),
        // the read will return the contents of an internal read buffer. This internal buffer is
        // updated only when reading PPUDATA, and so is preserved across frames. After the CPU reads
        // and gets the contents of the internal buffer, the PPU will immediately update the
        // internal buffer with the byte at the current VRAM address
        if addr < 0x3f00 {
            let buffered_val = self.ppu_data_buffer.get();
            self.ppu_data_buffer.set(val);
            buffered_val
        } else {
            val
        }
    }

    fn read<C: Cart>(&self, addr: u16, cart: &C) -> u8 {
        if addr < 0x2000 {
            cart.read_chr(addr)
        } else if addr < 0x3f00 {
            self.name_tables[addr as usize & 0x0fff]
        } else if addr < 0x4000 {
            let addr = addr & 0x1f;
            self.read_palette(addr)
        } else {
            panic!("Invalid vram read")
        }
    }

    fn read_palette(&self, addr: u16) -> u8 {
        // Certain sprite addresses are mirrored back into background addresses
        let addr = match addr & 0xf {
            0x0 => 0x0,
            0x4 => 0x4,
            0x8 => 0x8,
            0xc => 0xc,
            _ => addr,
        };
        self.palette[addr as usize]
    }

    fn addr(&self) -> u16 {
        self.address.get()
    }

    fn scroll_write(&self, latch_state: LatchState) {
        match latch_state {
            LatchState::FirstWrite(val) => {
                // t: ....... ...HGFED = d: HGFED...
                let t = self.t.get() & 0b_1111_1111_1110_0000;
                self.t.set(((u16::from(val) & 0b_1111_1000) >> 3) | t);

                //x:              CBA = d: .....CBA
                self.fine_x.set(val & 0b_0000_0111);
            }
            LatchState::SecondWrite(val) => {
                // t: CBA..HG FED..... = d: HGFEDCBA
                let t = self.t.get() & 0b_0000_1100_0001_1111;
                let cba_mask = (u16::from(val) & 0b_0000_0111) << 12;
                let hgfed_mask = (u16::from(val) & 0b_1111_1000) << 2;
                self.t.set((cba_mask | hgfed_mask) | t);
            }
        }
    }

    fn control_write(&self, val: u8) {
        // t: ...BA.. ........ = d: ......BA
        let t = self.t.get() & 0b0111_0011_1111_1111;
        let new_t = ((u16::from(val) & 0b0011) << 10) | t;
        self.t.set(new_t);
    }

    fn coarse_x_increment(&self) {
        let v = self.address.get();

        // The coarse X component of v needs to be incremented when the next tile is reached. Bits
        // 0-4 are incremented, with overflow toggling bit 10. This means that bits 0-4 count from 0
        // to 31 across a single nametable, and bit 10 selects the current nametable horizontally.
        let v = if v & 0x001F == 31 {
            // set coarse X = 0 and switch horizontal nametable
            (v & !0x001F) ^ 0x0400
        } else {
            // increment coarse X
            v + 1
        };

        self.address.set(v);
    }

    fn fine_y_increment(&self) {
        let v = self.address.get();

        let v = if v & 0x7000 != 0x7000 {
            // if fine Y < 7, increment fine Y
            v + 0x1000
        } else {
            // if fine Y = 0...
            let v = v & !0x7000;

            // let y = coarse Y
            let mut y = (v & 0x03E0) >> 5;
            let v = if y == 29 {
                // set coarse Y to 0
                y = 0;

                // switch vertical nametable
                v ^ 0x0800
            } else if y == 31 {
                // set coarse Y = 0, nametable not switched
                y = 0;
                v
            } else {
                // increment coarse Y
                y += 1;
                v
            };

            // put coarse Y back into v
            (v & !0x03E0) | (y << 5)
        };

        self.address.set(v);
    }

    fn copy_horizontal_pos_to_addr(&self) {
        // At dot 257 of each scanline, if rendering is enabled, the PPU copies all bits related to
        // horizontal position from t to v:
        // v: ....F.. ...EDCBA = t: ....F.. ...EDCBA
        let v = self.address.get() & 0b0111_1011_1110_0000;
        self.address.set((self.t.get() & 0b0000_0100_0001_1111) | v)
    }

    fn copy_vertical_pos_to_addr(&self) {
        // During dots 280 to 304 of the pre-render scanline (end of vblank), if rendering is
        // enabled, at the end of vblank, shortly after the horizontal bits are copied from t to v
        // at dot 257, the PPU will repeatedly copy the vertical bits from t to v from dots 280 to
        // 304, completing the full initialization of v from t:
        // v: IHGF.ED CBA..... = t: IHGF.ED CBA.....
        let v = self.address.get() & 0b0000_0100_0001_1111;
        self.address.set((self.t.get() & 0b0111_1011_1110_0000) | v)
    }

    fn fine_x(&self) -> u8 {
        self.fine_x.get()
    }
}
