#[cfg(test)]
mod spec_tests;

pub const FLG_VBLANK: u8 = 0b10000000;
pub const FLG_SPRITE_ZERO_HIT: u8 = 0b01000000;
pub const FLG_SPRITE_OVERFLOW: u8 = 0b00100000;

pub struct PpuStatus {
    reg: u8,
}

impl PpuStatus {
    pub fn new() -> Self {
        PpuStatus { reg: 0 }
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
