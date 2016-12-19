#[cfg(test)]
mod spec_tests;

use std::ops::Deref;

/// $2001, Write Only
/// This register controls the rendering of sprites and backgrounds, as well as colour effects.
pub struct MaskRegister {
    reg: u8,
}

impl Deref for MaskRegister {
    type Target = u8;

    fn deref(&self) -> &u8 {
        &self.reg
    }
}

#[derive(Debug, PartialEq)]
pub enum ColorMode {
    Normal,
    Greyscale,
}

impl MaskRegister {
    fn new(reg: u8) -> Self {
        MaskRegister { reg: reg }
    }

    /// Greyscale (0: normal color, 1: produce a greyscale display)
    fn color_mode(self) -> ColorMode {
        let mask = 0b00000001;
        let reg = *self;
        let val = reg & mask;

        if val == 0 {
            ColorMode::Normal
        } else {
            ColorMode::Greyscale
        }
    }

    fn background_render_leftmost_8_px(self) -> bool {
        let mask = 0b00000010;
        let reg = *self;
        reg & mask > 0
    }

    fn sprites_render_leftmost_8_px(self) -> bool {
        let mask = 0b00000100;
        let reg = *self;
        reg & mask > 0
    }

    fn show_background(self) -> bool {
        let mask = 0b00001000;
        let reg = *self;
        reg & mask > 0
    }

    fn show_sprites(self) -> bool {
        let mask = 0b00010000;
        let reg = *self;
        reg & mask > 0
    }

    fn emphasize_red(self) -> bool {
        let mask = 0b00100000;
        let reg = *self;
        reg & mask > 0
    }

    fn emphasize_green(self) -> bool {
        let mask = 0b01000000;
        let reg = *self;
        reg & mask > 0
    }

    fn emphasize_blue(self) -> bool {
        let mask = 0b10000000;
        let reg = *self;
        reg & mask > 0
    }
}
