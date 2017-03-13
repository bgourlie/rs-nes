use errors::*;
use ppu::object_attribute_memory::SpriteAttributes;
use ppu::vram::Vram;

pub struct Sprite {
    attributes: SpriteAttributes,
    pattern_lower: u8,
    pattern_upper: u8,
}

impl Sprite {
    pub fn read<V: Vram>(y: u8,
                         attributes: SpriteAttributes,
                         base_offset: u16,
                         vram: &V)
                         -> Result<Self> {
        let mut y = y - attributes.y;

        if attributes.vertical_flip {
            y = 7 - y;
        }

        debug_assert!(y < 8, "invalid pattern bit");
        let y = y % 8;
        let pattern_offset = base_offset + ((attributes.tile_index as u16) << 4) + y as u16;
        let pattern_lower = vram.read(pattern_offset)?;
        let pattern_upper = vram.read(pattern_offset + 8)?;

        Ok(Sprite {
            attributes: attributes,
            pattern_lower: pattern_lower,
            pattern_upper: pattern_upper,
        })
    }

    pub fn pixel_at(&self, x: u16) -> Option<u8> {
        if x >= self.attributes.x as u16 && x < self.attributes.x as u16 + 8 {
            let mut x = x - self.attributes.x as u16;

            if self.attributes.horizontal_flip {
                x = 7 - x;
            }

            debug_assert!(x < 8, "invalid pattern bit");
            let x = x % 8;

            // credit sprocket nes for the fancy bit fiddling
            let bit0 = (self.pattern_lower >> ((7 - x) as usize)) & 1;
            let bit1 = (self.pattern_upper >> ((7 - x) as usize)) & 1;
            Some((bit1 << 1) | bit0)
        } else {
            None
        }
    }

    pub fn palette(&self) -> u8 {
        self.attributes.palette
    }
}
