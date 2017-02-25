#[cfg(test)]
mod spec_tests;

use std::cell::Cell;

const VBLANK: u8 = 0b10000000;

#[derive(Default)]
pub struct StatusRegister {
    reg: Cell<u8>,
}

impl StatusRegister {
    #[cfg(test)]
    pub fn new(reg: u8) -> Self {
        StatusRegister { reg: Cell::new(reg) }
    }

    pub fn read(&self) -> u8 {
        self.reg.get()
    }

    /// Vertical blank has started (0: not in vblank; 1: in vblank).
    /// Set at dot 1 of line 241 (the line *after* the post-render
    /// line); cleared after reading $2002 and at dot 1 of the
    /// pre-render line.
    pub fn in_vblank(&self) -> bool {
        self.reg.get() & VBLANK > 0
    }

    pub fn set_in_vblank(&self) {
        let reg = self.reg.get() | VBLANK;
        self.reg.set(reg)
    }

    pub fn clear_in_vblank(&self) {
        let reg = self.reg.get() & !VBLANK;
        self.reg.set(reg)
    }

    /// Sprite 0 Hit.  Set when a nonzero pixel of sprite 0 overlaps
    /// a nonzero background pixel; cleared at dot 1 of the pre-render
    /// line.  Used for raster timing.
    fn sprite_zero_hit(&self) -> bool {
        self.reg.get() & 0b01000000 > 0
    }
    /// Sprite overflow. The intent was for this flag to be set
    /// whenever more than eight sprites appear on a scanline, but a
    /// hardware bug causes the actual behavior to be more complicated
    /// and generate false positives as well as false negatives; see
    /// PPU sprite evaluation. This flag is set during sprite
    /// evaluation and cleared at dot 1 (the second dot) of the
    /// pre-render line.
    /// See: https://github.com/christopherpow/nes-test-roms
    fn sprite_overflow(&self) -> bool {
        self.reg.get() & 0b00100000 > 0
    }
}
