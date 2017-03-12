use super::*;
use ppu::object_attribute_memory::SpriteAttributes;

pub struct SpritePixel {
    x: u16,
    y: u16,
    sprite_attributes: SpriteAttributes,
    pattern_table_base_offset: u16,
}

impl SpritePixel {
    pub fn new(x: u16,
               y: u16,
               sprite_attributes: SpriteAttributes,
               pattern_table_base_offset: u16)
               -> Self {
        SpritePixel {
            x: x,
            y: y,
            sprite_attributes: sprite_attributes,
            pattern_table_base_offset: pattern_table_base_offset,
        }
    }
}

impl Pixel for SpritePixel {
    fn palette(&self, _: u8) -> u8 {
        self.sprite_attributes.palette
    }

    fn pattern_offset(&self, _: u16) -> u16 {
        let mut y = self.y - self.sprite_attributes.y as u16;

        if self.sprite_attributes.vertical_flip {
            y = 7 - y;
        }

        debug_assert!(y < 8, "invalid pattern bit");
        let y = y % 8;
        self.pattern_table_base_offset + ((self.sprite_attributes.tile_index as u16) << 4) + y
    }

    fn color_index(&self, pattern_lower: u8, pattern_upper: u8) -> u8 {
        let mut x = self.x - self.sprite_attributes.x as u16;

        if self.sprite_attributes.horizontal_flip {
            x = 7 - x;
        }

        debug_assert!(x < 8, "invalid pattern bit");
        let x = x % 8;

        // credit sprocket nes for the fancy bit fiddling
        let bit0 = (pattern_lower >> ((7 - x) as usize)) & 1;
        let bit1 = (pattern_upper >> ((7 - x) as usize)) & 1;
        (bit1 << 1) | bit0
    }
}
