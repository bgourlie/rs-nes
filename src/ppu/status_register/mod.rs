#[cfg(test)]
mod spec_tests;

use std::ops::Deref;

/// $2002, Read Only
/// This register reflects the state of various functions inside the PPU.
///
/// ** Notes: **
///
/// - Reading the status register will clear D7 mentioned above and also the address latch used by
///   PPUSCROLL and PPUADDR. It does not clear the sprite 0 hit or overflow bit.
///
/// - Once the sprite 0 hit flag is set, it will not be cleared until the end of the next vertical
///   blank. If attempting to use this flag for raster timing, it is important to ensure that the
///   sprite 0 hit check happens outside of vertical blank, otherwise the CPU will "leak" through
///   and the check will fail. The easiest way to do this is to place an earlier check for D6 = 0,
///   which will wait for the pre-render scanline to begin.
///
/// - If using sprite 0 hit to make a bottom scroll bar below a vertically scrolling or
///   freely scrolling playfield, be careful to ensure that the tile in the playfield behind sprite
///   0 is opaque.
///
/// - Sprite 0 hit is not detected at x=255, nor is it detected at x=0 through 7 if the background
///   or sprites are hidden in this area.
///
/// - See: PPU rendering for more information on the timing of setting and clearing the flags.
///
/// - Some Vs. System PPUs return a constant value in D4-D0 that the game checks.
///
/// - Caution: Reading PPUSTATUS at the exact start of vertical blank will return 0 in bit 7 but
///   clear the latch anyway, causing the program to miss frames. See NMI for details.
#[derive(Copy, Clone)]
pub struct StatusRegister {
    reg: u8,
}

impl Deref for StatusRegister {
    type Target = u8;

    fn deref(&self) -> &u8 {
        &self.reg
    }
}

impl StatusRegister {
    pub fn new(reg: u8) -> Self {
        StatusRegister { reg: reg }
    }

    /// Vertical blank has started (0: not in vblank; 1: in vblank).
    /// Set at dot 1 of line 241 (the line *after* the post-render
    /// line); cleared after reading $2002 and at dot 1 of the
    /// pre-render line.
    fn in_vblank(self) -> bool {
        let mask = 0b10000000;
        let reg = *self;
        reg & mask > 0
    }

    /// Sprite 0 Hit.  Set when a nonzero pixel of sprite 0 overlaps
    /// a nonzero background pixel; cleared at dot 1 of the pre-render
    /// line.  Used for raster timing.
    fn sprite_zero_hit(self) -> bool {
        let mask = 0b01000000;
        let reg = *self;
        reg & mask > 0
    }
    /// Sprite overflow. The intent was for this flag to be set
    /// whenever more than eight sprites appear on a scanline, but a
    /// hardware bug causes the actual behavior to be more complicated
    /// and generate false positives as well as false negatives; see
    /// PPU sprite evaluation. This flag is set during sprite
    /// evaluation and cleared at dot 1 (the second dot) of the
    /// pre-render line.
    /// See: https://github.com/christopherpow/nes-test-roms
    fn sprite_overflow(self) -> bool {
        let mask = 0b00100000;
        let reg = *self;
        reg & mask > 0
    }
}
