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
        self.pattern_table_base_offset + ((self.sprite_attributes.tile_index as u16) << 4) +
        (self.y % 8) as u16
    }

    fn coords(&self) -> (u16, u16) {
        (self.x as u16, self.y as u16)
    }
}
