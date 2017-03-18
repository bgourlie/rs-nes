#[cfg(test)]
mod spec_tests;

use super::*;

#[derive(Copy, Clone)]
pub struct BackgroundPixel {
    pub x: u16,
    pub y: u16,
    pub name_table_offset: u16,
    pub attribute_table_offset: u16,
    pattern_table_base_offset: u16,
}

impl BackgroundPixel {
    pub fn new(x: u16, y: u16, pattern_table_base_offset: u16) -> Self {
        let tile_x = (x / 8) % 64;
        let tile_y = (y / 8) % 60;

        let name_table_base = match (tile_x >= 32, tile_y >= 30) {
            (false, false) => 0x2000,
            (true, false) => 0x2400,
            (false, true) => 0x2800,
            (true, true) => 0x2c00,
        };

        let tile_x = tile_x % 32;
        let tile_y = tile_y % 30;

        let name_table_offset = name_table_base + 32 * tile_y as u16 + tile_x as u16;
        let attribute_table_offset = name_table_base + 0x3c0 + Self::attribute_table_offset(x, y);

        BackgroundPixel {
            x: x,
            y: y,
            name_table_offset: name_table_offset,
            attribute_table_offset: attribute_table_offset,
            pattern_table_base_offset: pattern_table_base_offset,
        }
    }

    fn attribute_table_offset(x: u16, y: u16) -> u16 {
        8 * (y / 32) + (x / 32)
    }

    fn attribute_quadrant(&self) -> AttributeQuadrant {
        let (x_tile16, y_tile16) = ((self.x / 16) as u8, (self.y / 16) as u8);
        match (y_tile16 % 2 == 0, x_tile16 % 2 == 0) {
            (true, true) => AttributeQuadrant::TopLeft,
            (true, false) => AttributeQuadrant::TopRight,
            (false, true) => AttributeQuadrant::BottomLeft,
            (false, false) => AttributeQuadrant::BottomRight,
        }
    }
}

impl Pixel for BackgroundPixel {
    fn palette(&self, attribute_table_entry: u8) -> u8 {
        self.attribute_quadrant().palette(attribute_table_entry)
    }

    fn pattern_offset(&self, tile_index: u16) -> u16 {
        self.pattern_table_base_offset + ((tile_index as u16) << 4) + (self.y % 8)
    }

    fn color_index(&self, pattern_lower: u8, pattern_upper: u8) -> u8 {
        let x = self.x % 8;
        // credit sprocket nes for the fancy bit fiddling
        let bit0 = (pattern_lower >> ((7 - ((x % 8) as u8)) as usize)) & 1;
        let bit1 = (pattern_upper >> ((7 - ((x % 8) as u8)) as usize)) & 1;
        (bit1 << 1) | bit0
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
