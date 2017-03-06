use super::*;
use ppu::object_attribute_memory::SpriteAttributes;

pub struct SpritePixel {
    sprite_attributes: SpriteAttributes,
    pattern_table_base_offset: u16,
}

impl SpritePixel {
    fn new(sprite_attributes: SpriteAttributes, pattern_table_base_offset: u16) -> Self {
        SpritePixel {
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
        unimplemented!()
    }

    fn coords(&self) -> (u16, u16) {
        (self.sprite_attributes.x as u16, self.sprite_attributes.y as u16)
    }
}
