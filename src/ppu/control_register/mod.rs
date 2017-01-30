#[cfg(test)]
mod spec_tests;

use std::ops::Deref;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum SpriteSize {
    X8, // 8x8
    X16, // 8x16
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum PpuMode {
    Master,
    Slave,
}

/// $2000, Write Only
/// Various flags controlling PPU operation
#[derive(Copy, Clone)]
pub struct ControlRegister {
    reg: u8,
}

impl ControlRegister {
    pub fn new(reg: u8) -> Self {
        ControlRegister { reg: reg }
    }

    /// Base nametable address (0 = $2000; 1 = $2400; 2 = $2800; 3 = $2C00)
    fn base_name_table_addr(&self) -> u16 {
        let mask = 0b00000011;
        let reg = **self;
        let val = reg & mask;

        if val == 0 {
            0x2000
        } else if val == 1 {
            0x2400
        } else if val == 2 {
            0x2800
        } else {
            0x2C00
        }
    }

    /// VRAM address increment per CPU read/write of PPUDATA
    /// (0: add 1, going across; 1: add 32, going down)
    fn vram_addr_increment(&self) -> u16 {
        let mask = 0b0000100;
        let reg = **self;
        if reg & mask == 0 { 1 } else { 32 }
    }

    /// Sprite pattern table address for 8x8 sprites (0: $0000; 1: $1000; ignored in 8x16 mode)
    fn sprite_pattern_table_addr(&self) -> u16 {
        let mask = 0b00001000;
        let reg = **self;
        if reg & mask == 0 { 0x0 } else { 0x1000 }
    }

    /// Background pattern table address (0: $0000; 1: $1000)
    fn background_pattern_table_addr(&self) -> u16 {
        let mask = 0b00010000;
        let reg = **self;
        if reg & mask == 0 { 0x0 } else { 0x1000 }
    }

    /// Sprite size (0: 8x8; 1: 8x16)
    fn sprite_size(&self) -> SpriteSize {
        let mask = 0b00100000;
        let reg = **self;
        if reg & mask == 0 {
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
        let mask = 0b01000000;
        let reg = **self;

        if reg & mask == 0 {
            PpuMode::Master
        } else {
            PpuMode::Slave
        }
    }

    /// Generate an NMI at the start of the vertical blanking interval (0: off; 1: on)
    fn nmi_on_vblank_start(&self) -> bool {
        let mask = 0b10000000;
        let reg = **self;
        !(reg & mask == 0)
    }

    pub fn set(&mut self, val: u8) {
        self.reg = val;
    }
}

impl Deref for ControlRegister {
    type Target = u8;

    fn deref(&self) -> &u8 {
        &self.reg
    }
}
