#[cfg(test)]
mod spec_tests;

pub const FLG_NMI_ENABLE: u8 = 0b10000000;
pub const FLG_PPU_MASTER_SLAVE: u8 = 0b01000000;
pub const FLG_SPRITE_HEIGHT: u8 = 0b00100000;
pub const FLG_BACKGROUND_TILE_SELECT: u8 = 0b00010000;
pub const FLG_SPRITE_TILE_SELECT: u8 = 0b00001000;
pub const FLG_INCREMENT_MODE: u8 = 0b00000100;
const NAMETABLE_SELECT: u8 = 0b00000011;

pub struct PpuCtrl {
    reg: u8,
}

impl PpuCtrl {
    pub fn new() -> Self {
        PpuCtrl { reg: 0 }
    }

    #[cfg(test)]
    pub fn get_reg(&self) -> u8 {
        self.reg
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

    pub fn set_nametable_select(&mut self, val: u8) {
        self.reg = (self.reg & 0b11111100) | (val & 0x3);
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
