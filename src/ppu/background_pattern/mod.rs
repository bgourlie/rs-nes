#[cfg(test)]
mod spec_tests;

use errors::*;
use ppu::vram::Vram;

#[derive(Copy, Clone)]
pub struct BackgroundPattern {
    pattern_offset: u16,
    attriute_offset: u16,
    vram_addr: u16,
    fine_x: u8,
    coarse_x: u16,
    coarse_y: u16,
}

impl BackgroundPattern {
    pub fn new<V: Vram>(pattern_table_base: u16, fine_x: u8, vram: &V) -> Result<Self> {
        let v = vram.addr();
        let coarse_x = v & 0b11111;
        let coarse_y = (v & 0b1111100000) >> 5;
        let tile_address = 0x2000 | (v & 0x0fff);
        let tile = vram.read(tile_address)?;
        let fine_y = (v & 0b0111_0000_0000_0000) >> 12;
        let pattern_offset = pattern_table_base + ((tile as u16) << 4) + fine_y;
        let attribute_offset = 0x23c0 | (v & 0x0c00) | ((v >> 4) & 0x38) | ((v >> 2) & 0x07);

        Ok(BackgroundPattern {
               pattern_offset: pattern_offset,
               vram_addr: v,
               fine_x: fine_x,
               attriute_offset: attribute_offset,
               coarse_x: coarse_x,
               coarse_y: coarse_y,
           })
    }

    pub fn color_index<V: Vram>(&self, vram: &V) -> Result<u8> {
        let plane0 = vram.read(self.pattern_offset)?;
        let plane1 = vram.read(self.pattern_offset + 8)?;
        let bit0 = (plane0 >> ((7 - ((self.fine_x % 8) as u8)) as usize)) & 1;
        let bit1 = (plane1 >> ((7 - ((self.fine_x % 8) as u8)) as usize)) & 1;
        Ok((bit1 << 1) | bit0)
    }

    pub fn palette_index<V: Vram>(&self, vram: &V) -> Result<u8> {
        let attr_byte = vram.read(self.attriute_offset)?;
        let (left, top) = (self.coarse_x % 2 == 0, self.coarse_y % 2 == 0);

        let color_index = match (left, top) {
            (true, true) => attr_byte & 0x3,
            (false, true) => (attr_byte >> 2) & 0x3,
            (true, false) => (attr_byte >> 4) & 0x3,
            (false, false) => (attr_byte >> 6) & 0x3,
        };

        Ok(color_index)
    }
}
