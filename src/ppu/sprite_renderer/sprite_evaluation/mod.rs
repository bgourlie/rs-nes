#![allow(dead_code)]

#[cfg(test)]
mod spec_tests;

use ppu::SpriteSize;

#[derive(Default)]
pub struct SpriteEvaluation {
    scanline: u8,
    sprites_found: u8,
    secondary_oam: [u8; 32],
    sprite_zero_map: u8,
    n: u8,
    m: u8,
    sprite_size: SpriteSize,
    read_buffer: u8,
    sprite_overflow: bool,
    cycle: u8,
}

impl SpriteEvaluation {
    pub fn new(scanline: u8, sprite_size: SpriteSize) -> Self {
        SpriteEvaluation {
            scanline,
            sprites_found: 0,
            secondary_oam: [0xff; 32],
            sprite_zero_map: 0,
            read_buffer: 0,
            n: 0,
            m: 0,
            sprite_size,
            sprite_overflow: false,
            cycle: 0,
        }
    }

    pub fn sprite_zero_map(&self) -> u8 {
        self.sprite_zero_map
    }

    pub fn read_secondary_oam(&self, index: u8) -> u8 {
        self.secondary_oam[index as usize]
    }

    pub fn scanline(&self) -> u8 {
        self.scanline
    }

    pub fn tick(&mut self, primary_oam: &[u8]) {
        debug_assert!(self.cycle <= 191);
        debug_assert!(self.m < 4);
        if !self.sprite_overflow && self.n < 64 {
            if self.cycle % 2 == 0 {
                // Read Cycles
                let oam_addr = (self.n as usize) * 4 + self.m as usize;
                self.read_buffer = primary_oam[oam_addr];
            } else if self.sprites_found < 8 {
                // Write Cycles
                // Standard sprite evaluation
                if self.m == 0 {
                    // We're evaluating y, check if sprite is on scanline
                    let y = self.read_buffer;
                    self.secondary_oam[self.sprites_found as usize * 4] = y;
                    if !self.is_sprite_on_scanline(self.read_buffer) {
                        // Sprite not on scanline, increment n to move on to next sprite.
                        self.increment_n();
                    } else {
                        self.increment_m()
                    }
                } else {
                    // Copy remaining bytes of the sprite to secondary oam
                    self.secondary_oam[self.sprites_found as usize * 4 + self.m as usize] =
                        self.read_buffer;
                    self.increment_m();

                    if self.m == 0 {
                        // m overflowed, meaning we copied the last byte for the sprite
                        // We track if the sprite was sprite zero using a bit map.
                        // It's sprite 0 if n == 1 (n will have been incremented once it gets
                        // here)
                        if self.n == 1 {
                            self.sprite_zero_map = 1 << self.sprites_found;
                        }
                        self.sprites_found += 1;
                    }
                }
            } else {
                // Overflow sprite evaluation
                // The first sprite overflow evaluation correctly reads the y value of the next
                // sprite in OAM. After that, it reads the Y value of the first sprite in
                // secondary OAM. This contributes to the sprite overflow bug behavior.
                let y = self.read_buffer;
                if self.is_sprite_on_scanline(y) {
                    self.sprite_overflow = true;
                } else {
                    self.increment_n_hardware_bug();
                }
            }
        }
        self.cycle += 1;
    }

    fn increment_m(&mut self) {
        if self.m >= 3 {
            self.n += 1;
            self.m = 0;
        } else {
            self.m += 1
        };
    }

    fn increment_n(&mut self) {
        self.n += 1;
        self.m = 0;
    }

    /// Emulates the m increment bug during sprite overflow evaluation
    fn increment_n_hardware_bug(&mut self) {
        self.n += 1;
        self.m = if self.m >= 3 { 0 } else { self.m + 1 };
    }

    fn is_sprite_on_scanline(&self, y: u8) -> bool {
        match self.sprite_size {
            SpriteSize::X8 => y <= self.scanline && self.scanline - y < 8,
            SpriteSize::X16 => y <= self.scanline && self.scanline - y < 16,
        }
    }
}
