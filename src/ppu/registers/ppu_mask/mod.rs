use std::ops::{Deref, DerefMut};

#[cfg(test)]
mod spec_tests;

// Grayscale (0: normal color, 1: produce a greyscale display)
const FLG_GRAYSCALE: u8 = 0b00000001;

// 1: Show background in leftmost 8 pixels of screen, 0: Hide
const FLG_SHOW_BACKGROUND_LEFTMOST: u8 = 0b00000010;

// 1: Show sprites in leftmost 8 pixels of screen, 0: Hide
const FLG_SHOW_SPRITES_LEFTMOST: u8 = 0b00000100;

// 1: Show background
const FLG_SHOW_BACKGROUND: u8 = 0b00001000;

// 1: Show sprites
const FLG_SHOW_SPRITES: u8 = 0b00010000;

// Emphasize Red
const FLG_EMPHASIZE_RED: u8 = 0b00100000;

// Emphasize Green
const FLG_EMPHASIZE_GREEN: u8 = 0b01000000;

// Emphasize Blue
const FLG_EMPHASIZE_BLUE: u8 = 0b10000000;

pub enum ColorMode {
    Normal,
    Grayscale,
}

#[derive(Copy, Clone)]
pub struct PpuMask {
    reg: u8,
}

impl Deref for PpuMask {
    type Target = u8;

    fn deref(&self) -> &u8 {
        &self.reg
    }
}

impl DerefMut for PpuMask {
    fn deref_mut(&mut self) -> &mut u8 {
        &mut self.reg
    }
}

impl PpuMask {
    pub fn new() -> Self {
        PpuMask { reg: 0 }
    }

    pub fn color_mode(self) -> ColorMode {
        if *self & FLG_GRAYSCALE == 0 {
            ColorMode::Normal
        } else {
            ColorMode::Grayscale
        }
    }

    pub fn show_background_leftmost(self) -> bool {
        *self & FLG_SHOW_BACKGROUND_LEFTMOST != 0
    }

    pub fn show_sprites_leftmost(self) -> bool {
        *self & FLG_SHOW_SPRITES_LEFTMOST != 0
    }

    pub fn show_background(self) -> bool {
        *self & FLG_SHOW_BACKGROUND != 0
    }

    pub fn show_sprites(self) -> bool {
        *self & FLG_SHOW_SPRITES != 0
    }

    pub fn emphasize_blue(self) -> bool {
        *self & FLG_EMPHASIZE_BLUE != 0
    }

    pub fn emphasize_green(self) -> bool {
        *self & FLG_EMPHASIZE_GREEN != 0
    }

    pub fn emphasize_red(self) -> bool {
        *self & FLG_EMPHASIZE_RED != 0
    }
}
