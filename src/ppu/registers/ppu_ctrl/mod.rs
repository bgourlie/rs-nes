use std::ops::{Deref, DerefMut};

#[cfg(test)]
mod spec_tests;

const FLG_NMI_ENABLE: u8 = 0b10000000;
const FLG_PPU_MASTER_SLAVE: u8 = 0b01000000;
const FLG_SPRITE_SIZE: u8 = 0b00100000;
const FLG_BACKGROUND_TILE_SELECT: u8 = 0b00010000;
const FLG_SPRITE_TILE_SELECT: u8 = 0b00001000;
const FLG_INCREMENT_MODE: u8 = 0b00000100;
const NAMETABLE_SELECT: u8 = 0b00000011;

pub enum PpuDataAddrIncr {
    Across,
    Down,
}

pub enum SpriteSize {
    EightByEight,
    EightBySixteen,
}

#[derive(Copy, Clone)]
pub struct PpuCtrl {
    reg: u8,
}

impl Deref for PpuCtrl {
    type Target = u8;

    fn deref(&self) -> &u8 {
        &self.reg
    }
}

impl DerefMut for PpuCtrl {
    fn deref_mut(&mut self) -> &mut u8 {
        &mut self.reg
    }
}

impl PpuCtrl {
    pub fn new() -> Self {
        PpuCtrl { reg: 0 }
    }

    pub fn nmi_enable(self) -> bool {
        *self & FLG_NMI_ENABLE > 0
    }

    pub fn sprite_size(self) -> SpriteSize {
        if *self & FLG_SPRITE_SIZE == 0 {
            SpriteSize::EightByEight
        } else {
            SpriteSize::EightBySixteen
        }
    }

    pub fn sprite_pattern_table_addr(self) -> u16 {
        if *self & FLG_SPRITE_TILE_SELECT == 0 {
            0x0
        } else {
            0x1000
        }
    }

    pub fn background_pattern_table_addr(self) -> u16 {
        if *self & FLG_BACKGROUND_TILE_SELECT == 0 {
            0x0
        } else {
            0x1000
        }
    }

    pub fn ppu_data_addr_incr(self) -> PpuDataAddrIncr {
        if *self & FLG_INCREMENT_MODE == 0 {
            PpuDataAddrIncr::Across
        } else {
            PpuDataAddrIncr::Down
        }
    }

    pub fn nametable_base_addr(self) -> u16 {
        match *self & 0x3 {
            0 => 0x2000,
            1 => 0x2400,
            2 => 0x2800,
            3 => 0x2c00,
            _ => {
                panic!("This should never happen");
            }
        }
    }
}
