#![allow(dead_code)]

#[cfg(test)]
mod spec_tests;

use ppu::SpriteSize;

#[derive(Default)]
pub struct SpriteEvaluation {
    scanline: u8,
    sprites_found: u8,
    secondary_oam: [u8; 0x20],
    n: u8,
    m: u8,
    sprite_size: SpriteSize,
    read_buffer: u8,
    first_sprite_overflow_check_occurred: bool,
    sprite_overflow_read_cycle: u8,
    sprite_overflow_read: bool,
    overflow: bool
}

impl SpriteEvaluation {
    pub fn new(scanline: u8, sprite_size: SpriteSize) -> Self {
        SpriteEvaluation {
            scanline: scanline,
            sprites_found: 0,
            secondary_oam: [0xff; 0x20],
            read_buffer: 0,
            first_sprite_overflow_check_occurred: false,
            n: 0,
            m: 0,
            sprite_size: sprite_size,
            sprite_overflow_read: false,
            sprite_overflow_read_cycle: 0,
            overflow: false,
        }
    }

    pub fn tick(&mut self, primary_oam: &[u8], cycle: u64) {
        debug_assert!(cycle >= 65 && cycle <= 256);
        debug_assert!(self.m < 4);
        if self.n >= 64 {
            // If we've evaluated all sprints, do nothing
            println!("Evaluated all sprites, doing nothing");
        } else {
            let oam_addr = (self.n as usize) * 4 + self.m as usize;

            if cycle % 2 == 1 { // Read Cycles
                self.read_buffer = primary_oam[oam_addr];
                println!();
                println!("CYCLE {} (READ): Read value = {}", cycle, self.read_buffer);
            } else { // Write Cycles
                println!("CYCLE {} (WRITE): ", cycle);
                if self.sprites_found < 8 { // Standard sprite evaluation
                    println!("    Fewer than 8 sprites on scanline so far.");

                    if self.m == 0 { // We're evaluating y, check if sprite is on scanline
                        let y = self.read_buffer;
                        println!("        Evaluating Sprite (y = {}). ", y);
                        self.secondary_oam[self.sprites_found as usize * 4] = y;
                        if !self.is_sprite_on_scanline(self.read_buffer) {
                            // Sprite not on scanline, increment n to move on to next sprite.
                            println!("            Sprite not on scanline, incrementing n to {}.", self.n + 1);
                            self.increment_n();
                        } else {
                            println!("            Sprite on scanline, increment m to {}.", self.m + 1);
                            self.increment_m()
                        }
                    } else { // Copy remaining bytes of the sprite to secondary oam
                        println!("    Copying remaining bytes for sprite {} of 8 (n = {}, m = {}). ", self.sprites_found + 1, self.n, self.m);
                        self.secondary_oam[self.sprites_found as usize * 4 + self.m as usize] = self.read_buffer;
                        self.increment_m();

                        if self.m == 0 { // m overflowed, we copied the last byte for the sprite
                            println!("        Finished copying bytes for sprite {} of 8. Incrementing sprites_found and n, resetting m. ", self.sprites_found + 1);
                            self.sprites_found += 1;
                        }
                    }
                } else { // Overflow sprite evaluation
                    println!("    In sprite overflow evaluation.");

                    if self.sprite_overflow_read {
                        println!("        Sprite overflow dummy read.");
                        self.sprite_overflow_read_cycle = if self.sprite_overflow_read_cycle >= 3 {
                            self.sprite_overflow_read = false;
                            0
                        } else {
                            self.sprite_overflow_read_cycle + 1
                        };
                    } else {
                        // The first sprite overflow evaluation correctly reads the y value of the next
                        // sprite in OAM. After that, it reads the Y value of the first sprite in
                        // secondary OAM. This contributes to the sprite overflow bug behavior.
                        let y = if !self.first_sprite_overflow_check_occurred {
                            println!("        First sprite overflow check");
                            self.first_sprite_overflow_check_occurred = true;
                            self.read_buffer
                        } else {
                            println!("        Not first sprite overflow check, incorrectly evaluating y");
                            self.secondary_oam[0]
                        };

                        if self.is_sprite_on_scanline(y) {
                            println!("        Sprite is on scanline. Mock byte writes. ");
                            self.increment_m();
                            self.sprite_overflow_read = true;
                            self.overflow = true;
                        } else {
                            println!("        No sprite overflow, Incrementing n and m per hardware bug. ");
                            self.increment_n_hardware_bug();
                        }
                    }
                }
            }
        }
    }

    fn increment_m(&mut self) {
        if self.m >= 3 {
            //print!("m overflowed; Incrementing n to {}, resetting m ", self.n + 1);
            self.n += 1;
            self.m = 0;
        } else {
            //print!("Incrementing m to {} ", self.m + 1);
            self.m += 1
        };
    }

    fn increment_n(&mut self) {
        //print!("incrementing n to {}, resetting m ", self.n + 1);
        self.n += 1;
        self.m = 0;
    }

    /// Emulates the m increment bug during sprite overflow evaluation
    fn increment_n_hardware_bug(&mut self) {
        //print!("incrementing n to {} and m (hardware bug) to {} (will overflow to 0 if 4) ", self.n + 1, self.m + 1);
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
