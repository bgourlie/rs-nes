#[cfg(test)]
mod spec_tests;

// Grayscale (0: normal color, 1: produce a greyscale display)
pub const FLG_GRAYSCALE: u8 = 0b00000001;

// 1: Show background in leftmost 8 pixels of screen, 0: Hide
pub const FLG_SHOW_BACKGROUND_LEFTMOST: u8 = 0b00000010;

// 1: Show sprites in leftmost 8 pixels of screen, 0: Hide
pub const FLG_SHOW_SPRITE_LEFTMOST: u8 = 0b00000100;

// 1: Show background
pub const FLG_SHOW_BACKGROUND: u8 = 0b00001000;

// 1: Show sprites
pub const FLG_SHOW_SPRITES: u8 = 0b00010000;

// Emphasize Red
pub const FLG_EMPHASIZE_RED: u8 = 0b00100000;

// Emphasize Green
pub const FLG_EMPHASIZE_GREEN: u8 = 0b01000000;

// Emphasize Blue
pub const FLG_EMPHASIZE_BLUE: u8 = 0b10000000;

pub struct PpuMask {
    reg: u8,
}

impl PpuMask {
    pub fn new() -> Self {
        PpuMask { reg: 0 }
    }

    pub fn set_flag(&mut self, mask: u8, val: bool) {
        if val {
            self.reg |= mask;
        } else {
            self.reg &= !mask;
        }
    }

    pub fn get_flag(&self, mask: u8) -> bool {
        self.reg & mask != 0
    }
}
