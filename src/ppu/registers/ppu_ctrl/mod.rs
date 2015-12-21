use std::ops::{Deref, DerefMut};

#[cfg(test)]
mod spec_tests;

const FLG_NMI_ENABLE: u8 = 0b10000000;
const FLG_PPU_MASTER_SLAVE: u8 = 0b01000000;
const FLG_SPRITE_HEIGHT: u8 = 0b00100000;
const FLG_BACKGROUND_TILE_SELECT: u8 = 0b00010000;
const FLG_SPRITE_TILE_SELECT: u8 = 0b00001000;
const FLG_INCREMENT_MODE: u8 = 0b00000100;
const NAMETABLE_SELECT: u8 = 0b00000011;

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

    pub fn get_nametable_base_addr(&self) -> u16 {
        match self.reg & 0x3 {
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
