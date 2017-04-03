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
}

pub struct SpriteRendererBase {
    primary_oam: [u8; 0x100],
    secondary_oam: [u8; 0x20],
    address: Cell<u8>, // Maps to the PPU's oam_addr register
    palettes: [Color; 16],
}

impl Default for SpriteRendererBase {
    fn default() -> Self {
        SpriteRendererBase {
            primary_oam: [0; 0x100],
            secondary_oam: [0; 0x20],
            address: Cell::new(0),
            palettes: palette::EMPTY,
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
}
