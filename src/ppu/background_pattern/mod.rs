#[cfg(test)]
mod spec_tests;

use errors::*;
use ppu::vram::Vram;

#[derive(Copy, Clone)]
pub struct BackgroundPattern {
    pub pixel_x: u16,
    pub pixel_y: u16,
    pub palette_index: u8,
    pub color_index: u8,
}

impl BackgroundPattern {
    pub fn new<V: Vram>(x_pixel: u16,
                        y_pixel: u16,
                        pattern_table_base_offset: u16,
                        vram: &V)
                        -> Result<Self> {
        // Nametables are twice the width and height of the screen, so 64 x 60 tiles
        let nametable_tile_x = (x_pixel / 8) % 64;
        let nametable_tile_y = (y_pixel / 8) % 60;
        let (tile_pixel_x, tile_pixel_y) = ((x_pixel % 8) as u8, (y_pixel % 8) as u8);
        let name_table_base = match (nametable_tile_x >= 32, nametable_tile_y >= 30) {
            (false, false) => 0x2000,
            (true, false) => 0x2400,
            (false, true) => 0x2800,
            (true, true) => 0x2c00,
        };

        let screen_tile_x = nametable_tile_x % 32;
        let screen_tile_y = nametable_tile_y % 30;

        let tile =
            vram.read(name_table_base + 32 * (screen_tile_y as u16) + (screen_tile_x as u16))?;
        let pattern_offset = pattern_table_base_offset + ((tile as u16) << 4) + tile_pixel_y as u16;

        // Determine the color of this pixel.
        let plane0 = vram.read(pattern_offset)?;
        let plane1 = vram.read(pattern_offset + 8)?;
        let bit0 = (plane0 >> ((7 - ((tile_pixel_x % 8) as u8)) as usize)) & 1;
        let bit1 = (plane1 >> ((7 - ((tile_pixel_x % 8) as u8)) as usize)) & 1;
        let color_index = (bit1 << 1) | bit0;


        let group = screen_tile_y / 4 * 8 + screen_tile_x / 4;

        let attr_byte = vram.read(name_table_base + 0x3c0 + (group as u16))?;
        let (left, top) = (screen_tile_x % 4 < 2, screen_tile_y % 4 < 2);

        let palette_index = match (left, top) {
            (true, true) => attr_byte & 0x3,
            (false, true) => (attr_byte >> 2) & 0x3,
            (true, false) => (attr_byte >> 4) & 0x3,
            (false, false) => (attr_byte >> 6) & 0x3,
        };

        //let tile_color = (palette_index << 2) | color_index;
        //println!("tile_color = {}", tile_color);
        //let palette_index = vram.read(0x3f00 + (tile_color as u16))? & 0x3f;

        Ok(BackgroundPattern {
            pixel_x: x_pixel,
            pixel_y: y_pixel,
            palette_index: palette_index,
            color_index: color_index,
        })
    }

    fn attribute_table_offset(x: u16, y: u16) -> u16 {
        8 * (y / 32) + (x / 32)
    }

    fn attribute_quadrant(&self) -> AttributeQuadrant {
        let (x_tile16, y_tile16) = ((self.pixel_x / 16) as u8, (self.pixel_y / 16) as u8);
        match (y_tile16 % 2 == 0, x_tile16 % 2 == 0) {
            (true, true) => AttributeQuadrant::TopLeft,
            (true, false) => AttributeQuadrant::TopRight,
            (false, true) => AttributeQuadrant::BottomLeft,
            (false, false) => AttributeQuadrant::BottomRight,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum AttributeQuadrant {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl AttributeQuadrant {
    fn palette(&self, attribute_table_entry: u8) -> u8 {
        match *self {
            AttributeQuadrant::TopLeft => {
                // top left
                attribute_table_entry & 0x3
            }
            AttributeQuadrant::TopRight => {
                // top right
                (attribute_table_entry >> 2) & 0x3
            }
            AttributeQuadrant::BottomLeft => {
                // bottom left
                (attribute_table_entry >> 4) & 0x3
            }
            AttributeQuadrant::BottomRight => {
                // bottom right
                (attribute_table_entry >> 6) & 0x3
            }
        }
    }
}
