#[cfg(test)]
mod spec_tests;

use errors::*;
use ppu::vram::Vram;

#[derive(Copy, Clone)]
pub struct BackgroundPattern {
    pub nametable_pixel_x: u16,
    pub nametable_pixel_y: u16,
    screen_tile_x: u8,
    screen_tile_y: u8,
    pattern_offset: u16,
    name_table_base_offset: u16,
}

impl BackgroundPattern {
    pub fn new<V: Vram>(nametable_x_pixel: u16,
                        nametable_y_pixel: u16,
                        pattern_table_base_offset: u16,
                        vram: &V)
                        -> Result<Self> {
        let name_table_base = Self::nametable_base_offset(nametable_x_pixel, nametable_y_pixel);

        // Determine the color of this pixel.
        let screen_tile_x = (nametable_x_pixel / 8) % 32;
        let screen_tile_y = (nametable_y_pixel / 8) % 30;
        let tile_offset = name_table_base + 32 * screen_tile_y + screen_tile_x;
        let tile = vram.read(tile_offset)?;
        let tile_pixel_y = (nametable_y_pixel % 8) as u8;
        let pattern_offset = pattern_table_base_offset + ((tile as u16) << 4) + tile_pixel_y as u16;

        Ok(BackgroundPattern {
            nametable_pixel_x: nametable_x_pixel,
            nametable_pixel_y: nametable_y_pixel,
            pattern_offset: pattern_offset,
            screen_tile_x: screen_tile_x as u8,
            screen_tile_y: screen_tile_y as u8,
            name_table_base_offset: name_table_base,
        })
    }

    pub fn color_index<V: Vram>(&self, vram: &V) -> Result<u8> {
        let plane0 = vram.read(self.pattern_offset)?;
        let plane1 = vram.read(self.pattern_offset + 8)?;
        let tile_pixel_x = (self.nametable_pixel_x % 8) as u8;
        let bit0 = (plane0 >> ((7 - ((tile_pixel_x % 8) as u8)) as usize)) & 1;
        let bit1 = (plane1 >> ((7 - ((tile_pixel_x % 8) as u8)) as usize)) & 1;
        Ok((bit1 << 1) | bit0)
    }

    pub fn palette_index<V: Vram>(&self, vram: &V) -> Result<u8> {
        let block_index = self.screen_tile_y / 4 * 8 + self.screen_tile_x / 4;

        let attr_byte = vram.read(self.name_table_base_offset + 0x3c0 + (block_index as u16))?;
        let (left, top) = (self.screen_tile_x % 4 < 2, self.screen_tile_y % 4 < 2);

        let color_index = match (left, top) {
            (true, true) => attr_byte & 0x3,
            (false, true) => (attr_byte >> 2) & 0x3,
            (true, false) => (attr_byte >> 4) & 0x3,
            (false, false) => (attr_byte >> 6) & 0x3,
        };

        Ok(color_index)
    }

    fn attribute_table_offset(x: u16, y: u16) -> u16 {
        8 * (y / 32) + (x / 32)
    }

    fn nametable_base_offset(x_pixel: u16, y_pixel: u16) -> u16 {
        // Nametables are twice the width and height of the screen, so 64 x 60 tiles
        let nametable_tile_x = (x_pixel / 8) % 64;
        let nametable_tile_y = (y_pixel / 8) % 60;
        match (nametable_tile_x >= 32, nametable_tile_y >= 30) {
            (false, false) => 0x2000,
            (true, false) => 0x2400,
            (false, true) => 0x2800,
            (true, true) => 0x2c00,
        }
    }
}
