// TODO: Explore SIMD
// TODO: Tests

use errors::*;
use ppu::palette::{self, Color, PALETTE};
use ppu::vram::Vram;
use std::cell::Cell;
use std::num::Wrapping;

#[cfg(test)]
mod spec_tests;

pub trait SpriteRenderer: Default {
    fn read_data(&self) -> u8;
    fn read_data_increment_addr(&self) -> u8;
    fn write_address(&mut self, addr: u8);
    fn write_data(&mut self, val: u8);
    fn update_palettes<V: Vram>(&mut self, vram: &V) -> Result<()>;
    fn pixel_color(&self, pixel: u8) -> Color;
    fn dec_x_counters(&mut self);
    fn start_secondary_oam_init(&mut self);
    fn start_sprite_evaluation(&mut self);
    fn tick_secondary_oam_init(&mut self);
    fn tick_sprite_evaluation<V: Vram>(&mut self, vram: &V) -> Result<()>;
    fn fetch_pattern_low_byte<V: Vram>(&mut self, vram: &V) -> Result<()>;
    fn fetch_pattern_high_byte<V: Vram>(&mut self, vram: &V) -> Result<()>;
}

pub struct SpriteRendererBase {
    primary_oam: [u8; 0x100],
    secondary_oam: [u8; 0x20],
    address: Cell<u8>, // Maps to the PPU's oam_addr register
    palettes: [Color; 16],
    pattern_shift_registers: [u8; 16], // odd index pattern low, even index pattern high
    attribute_latches: [u8; 8],
    x_counters: [u8; 8],
    secondary_oam_init_cycle: u8,
    sprite_evaluation_cycle: u8,
}

impl Default for SpriteRendererBase {
    fn default() -> Self {
        SpriteRendererBase {
            primary_oam: [0; 0x100],
            secondary_oam: [0; 0x20],
            address: Cell::new(0),
            palettes: palette::EMPTY,
            pattern_shift_registers: [0; 16],
            attribute_latches: [0; 8],
            x_counters: [0; 8],
            secondary_oam_init_cycle: 0,
            sprite_evaluation_cycle: 0,
        }
    }
}

impl SpriteRendererBase {
    fn inc_address(&self) {
        let new_addr = (Wrapping(self.address.get()) + Wrapping(1_u8)).0;
        self.address.set(new_addr)
    }
}

impl SpriteRenderer for SpriteRendererBase {
    // Maps to the PPU's oam_data register
    fn read_data(&self) -> u8 {
        self.primary_oam[self.address.get() as usize]
    }

    fn read_data_increment_addr(&self) -> u8 {
        let ret = self.read_data();
        self.inc_address();
        ret
    }

    fn write_address(&mut self, val: u8) {
        self.address.set(val);
    }

    fn write_data(&mut self, val: u8) {
        self.primary_oam[self.address.get() as usize] = val;
        self.inc_address();
    }

    //    fn sprite_attributes(&self, tile_index: u8) -> SpriteAttributes {
    //        debug_assert!(tile_index <= 64, "Tile index out of bounds: {}", tile_index);
    //        let mem = self.primary_oam;
    //        let index = tile_index as usize * 4;
    //        let y = mem[index];
    //        let tile_index = mem[index + 1];
    //        let attributes = mem[index + 2];
    //        let x = mem[index + 3];
    //
    //        let palette = attributes & 0b00000011;
    //
    //        let priority = if attributes & 0b00100000 == 0 {
    //            Priority::InFrontOfBackground
    //        } else {
    //            Priority::BehindBackground
    //        };
    //
    //        let horizontal_flip = attributes & 0b01000000 > 0;
    //        let vertical_flip = attributes & 0b10000000 > 0;
    //
    //        SpriteAttributes {
    //            x: x,
    //            y: y,
    //            palette: palette,
    //            priority: priority,
    //            horizontal_flip: horizontal_flip,
    //            vertical_flip: vertical_flip,
    //            tile_index: tile_index,
    //        }
    //    }

    fn update_palettes<V: Vram>(&mut self, vram: &V) -> Result<()> {
        let bg = vram.read(0x3f00)? as usize;
        self.palettes = [PALETTE[bg],
                         PALETTE[vram.read(0x3f11)? as usize],
                         PALETTE[vram.read(0x3f12)? as usize],
                         PALETTE[vram.read(0x3f13)? as usize],
                         PALETTE[bg],
                         PALETTE[vram.read(0x3f15)? as usize],
                         PALETTE[vram.read(0x3f16)? as usize],
                         PALETTE[vram.read(0x3f17)? as usize],
                         PALETTE[bg],
                         PALETTE[vram.read(0x3f19)? as usize],
                         PALETTE[vram.read(0x3f1a)? as usize],
                         PALETTE[vram.read(0x3f1b)? as usize],
                         PALETTE[bg],
                         PALETTE[vram.read(0x3f1d)? as usize],
                         PALETTE[vram.read(0x3f1e)? as usize],
                         PALETTE[vram.read(0x3f1f)? as usize]];
        Ok(())
    }

    fn pixel_color(&self, pixel: u8) -> Color {
        self.palettes[pixel as usize]
    }

    fn dec_x_counters(&mut self) {
        for i in 0..8 {
            if self.x_counters[i] > 0 {
                self.x_counters[i] -= 1;
            } else {
                self.pattern_shift_registers[i * 2] <<= 1;
                self.pattern_shift_registers[i * 2 + 1] <<= 1;
            }
        }
    }

    fn tick_secondary_oam_init(&mut self) {
        debug_assert!(self.secondary_oam_init_cycle < 64);
        if self.secondary_oam_init_cycle % 2 == 1 {
            self.secondary_oam[(self.secondary_oam_init_cycle / 2) as usize] = 0xff
        }
        self.secondary_oam_init_cycle += 1;
    }

    fn tick_sprite_evaluation<V: Vram>(&mut self, vram: &V) -> Result<()> {
        //  Cycles 65-256: Sprite evaluation
        //  On odd cycles, data is read from (primary) OAM
        //  On even cycles, data is written to secondary OAM (unless secondary OAM is full, in which
        //  case it will read the value in secondary OAM instead)
        //
        //  1.  Starting at n = 0, read a sprite's Y-coordinate (OAM[n][0], copying it to the next
        //      open slot in secondary OAM (unless 8 sprites have been found, in which case the write is ignored).
        //
        //    1a. If Y-coordinate is in range, copy remaining bytes of sprite data (OAM[n][1] thru
        //      OAM[n][3]) into secondary OAM.
        //
        //  2.  Increment n
        //    2a. If n has overflowed back to zero (all 64 sprites evaluated), go to 4
        //
        //    2b. If less than 8 sprites have been found, go to 1
        //
        //    2c. If exactly 8 sprites have been found, disable writes to secondary OAM because it
        //        is full. This causes sprites in back to drop out.
        //
        //  3.  Starting at m = 0, evaluate OAM[n][m] as a Y-coordinate.
        //
        //    3a. If the value is in range, set the sprite overflow flag in $2002 and read the next
        //        3 entries of OAM (incrementing 'm' after each byte and incrementing 'n' when 'm'
        //        overflows); if m = 3, increment n
        //
        //    3b. If the value is not in range, increment n and m (without carry). If n overflows
        //        to 0, go to 4; otherwise go to 3
        //
        //        - The m increment is a hardware bug - if only n was incremented, the overflow
        //          flag would be set whenever more than 8 sprites were present on the same
        //          scanline, as expected.
        //
        //  4.  Attempt (and fail) to copy OAM[n][0] into the next free slot in secondary OAM, and
        //      increment n (repeat until HBLANK is reached)
        unimplemented!()
    }
    fn fetch_pattern_low_byte<V: Vram>(&mut self, _: &V) -> Result<()> {
        unimplemented!()
    }

    fn fetch_pattern_high_byte<V: Vram>(&mut self, _: &V) -> Result<()> {
        unimplemented!()
    }

    fn start_secondary_oam_init(&mut self) {
        self.secondary_oam_init_cycle = 0
    }

    fn start_sprite_evaluation(&mut self) {
        self.sprite_evaluation_cycle = 0
    }
}
