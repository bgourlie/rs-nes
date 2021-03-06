#![allow(dead_code)]

#[cfg(test)]
mod spec_tests;

use std::ops::Deref;

/// $2001, Write Only
/// This register controls the rendering of sprites and backgrounds, as well as color effects.
#[derive(Default)]
pub struct MaskRegister {
    reg: u8,
}

impl Deref for MaskRegister {
    type Target = u8;

    fn deref(&self) -> &u8 {
        &self.reg
    }
}

impl MaskRegister {
    pub fn background_render_leftmost_8_px(&self) -> bool {
        self.reg & 0b0000_0010 > 0
    }

    pub fn sprites_render_leftmost_8_px(&self) -> bool {
        self.reg & 0b0000_0100 > 0
    }

    pub fn show_background(&self) -> bool {
        self.reg & 0b0000_1000 > 0
    }

    pub fn show_sprites(&self) -> bool {
        self.reg & 0b0001_0000 > 0
    }

    fn emphasize_red(&self) -> bool {
        self.reg & 0b0010_0000 > 0
    }

    fn emphasize_green(&self) -> bool {
        self.reg & 0b0100_0000 > 0
    }

    fn emphasize_blue(&self) -> bool {
        self.reg & 0b1000_0000 > 0
    }

    pub fn write(&mut self, val: u8) {
        self.reg = val;
    }

    pub fn rendering_enabled(&self) -> bool {
        self.show_background() || self.show_sprites()
    }
}
