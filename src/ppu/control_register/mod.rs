#[cfg(test)]
mod spec_tests;

use std::ops::Deref;

pub const BG_PATTERN_SELECT: u8 = 0b00010000;
pub const SPRITE_PATTERN_SELECT: u8 = 0b00001000;

#[derive(Debug, PartialEq)]
pub enum SpriteSize {
    X8, // 8x8
    X16, // 8x16
}

#[derive(Debug, PartialEq)]
pub enum PpuMode {
    Master,
    Slave,
}

#[derive(Debug, PartialEq)]
pub enum IncrementAmount {
    One,
    ThirtyTwo,
}

/// $2000, Write Only
/// Various flags controlling PPU operation
#[derive(Default)]
pub struct ControlRegister {
    reg: u8,
}

impl ControlRegister {
    /// VRAM address increment per CPU read/write of PPUDATA
    /// (0: add 1, going across; 1: add 32, going down)
    pub fn vram_addr_increment(&self) -> IncrementAmount {
        if self.reg & 0b0000100 == 0 {
            IncrementAmount::One
        } else {
            IncrementAmount::ThirtyTwo
        }
    }

    /// Sprite size (0: 8x8; 1: 8x16)
    pub fn sprite_size(&self) -> SpriteSize {
        if self.reg & 0b00100000 == 0 {
            SpriteSize::X8
        } else {
            SpriteSize::X16
        }
    }

    /// PPU master/slave select (0: read backdrop from EXT pins; 1: output color on EXT pins)
    ///
    /// *Note:* I don't think this is necessary for emulation since the NES never set the PPU
    /// slave bit. Apparently, it could actually harm the NES hardware if it were set.
    fn ppu_mode(&self) -> PpuMode {
        if self.reg & 0b01000000 == 0 {
            PpuMode::Master
        } else {
            PpuMode::Slave
        }
    }

    /// Generate an NMI at the start of the vertical blanking interval (0: off; 1: on)
    pub fn nmi_on_vblank_start(&self) -> bool {
        !(self.reg & 0b10000000 == 0)
    }

    pub fn write(&mut self, val: u8) {
        self.reg = val;
    }
}

impl Deref for ControlRegister {
    type Target = u8;

    fn deref(&self) -> &u8 {
        &self.reg
    }
}
