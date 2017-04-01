#[cfg(test)]
mod spec_tests;

use errors::*;
use ppu::control_register::PatternTableSelect;
use ppu::vram::Vram;
use std::cell::Cell;

#[derive(Default)]
pub struct BackgroundRenderer {
    pattern_low_shift_register: u16,
    pattern_high_shift_register: u16,
    palette_low_bit_shift_register: u16,
    palette_high_bit_shift_register: u16,
    attr_latch: u8,
    nametable_latch: u8,
    pattern_low_latch: u8,
    pattern_high_latch: u8,
    current_pixel: u8,
    pattern_table_select: Cell<PatternTableSelect>, // TODO: Remove this
}

// TODO tests
impl BackgroundRenderer {
    pub fn current_pixel(&self) -> u8 {
        self.current_pixel
    }

    pub fn fill_shift_registers(&mut self, vram_addr: u16) {
        self.pattern_low_shift_register |= self.pattern_low_latch as u16;
        self.pattern_high_shift_register |= self.pattern_high_latch as u16;
        let (palette_low, palette_high) = Self::palette_shift_bytes(vram_addr, self.attr_latch);
        self.palette_low_bit_shift_register |= palette_low as u16;
        self.palette_high_bit_shift_register |= palette_high as u16;
    }

    fn palette_shift_bytes(v: u16, attr_byte: u8) -> (u8, u8) {
        // Bit 1 of coarse x and coarse y determine which bits get loaded into the shift registers.

        // Intentionally shift each bit one too few to effectively multiply the OR'd result by two.
        // This will give us the amount to shift right by, selecting the correct two attribute bits.
        let x_component = (v << 1) & 0b10;
        let y_component = (v >> 3) & 0b100;
        let shift = (y_component | x_component) as usize;
        let palette_nibble = (attr_byte >> shift) & 0b11;

        // Return (low bits, high bits)
        ((palette_nibble & 1) * 255, ((palette_nibble >> 1) & 1) * 255)
    }

    //TODO: TEST THIS
    pub fn tick_shifters(&mut self, fine_x: u8) {
        let pattern_low_bit = (self.pattern_low_shift_register << fine_x) & 0x8000;
        let pattern_high_bit = (self.pattern_high_shift_register << fine_x) & 0x8000;
        let pixel_low_nibble = (pattern_high_bit >> 14) | (pattern_low_bit >> 15);

        let palette_low_bit = (self.palette_low_bit_shift_register << fine_x) & 0x8000;
        let palette_high_bit = (self.palette_high_bit_shift_register << fine_x) & 0x8000;
        let pixel_high_nibble = ((palette_high_bit >> 12) | (palette_low_bit >> 13)) & 0b1100;

        self.current_pixel = (pixel_high_nibble | pixel_low_nibble) as u8;
        self.pattern_low_shift_register <<= 1;
        self.pattern_high_shift_register <<= 1;
        self.palette_low_bit_shift_register <<= 1;
        self.palette_high_bit_shift_register <<= 1;
    }

    pub fn fetch_attribute_byte<V: Vram>(&mut self, vram: &V) -> Result<()> {
        let v = vram.addr();
        let attribute_address = 0x23C0 | (v & 0x0C00) | ((v >> 4) & 0x38) | ((v >> 2) & 0x07);
        self.attr_latch = vram.read(attribute_address)?;
        Ok(())
    }

    pub fn fetch_nametable_byte<V: Vram>(&mut self, vram: &V) -> Result<()> {
        let nametable_address = 0x2000 | (vram.addr() & 0x0FFF);
        self.nametable_latch = vram.read(nametable_address)?;
        Ok(())
    }

    pub fn fetch_pattern_low_byte<V: Vram>(&mut self,
                                           vram: &V,
                                           table_select: PatternTableSelect)
                                           -> Result<()> {
        self.pattern_low_latch = self.fetch_pattern_plane(vram, table_select, true)?;
        Ok(())
    }

    pub fn fetch_pattern_high_byte<V: Vram>(&mut self,
                                            vram: &V,
                                            table_select: PatternTableSelect)
                                            -> Result<()> {
        self.pattern_high_latch = self.fetch_pattern_plane(vram, table_select, false)?;
        Ok(())
    }

    fn fetch_pattern_plane<V: Vram>(&self,
                                    vram: &V,
                                    table_select: PatternTableSelect,
                                    is_lower_plane: bool)
                                    -> Result<u8> {
        let v = vram.addr();
        let fine_y = (v >> 12) & 0b111;

        let plane = if is_lower_plane { 0 } else { 0b1000 };

        let column_and_row = (self.nametable_latch as u16) << 4;
        let pattern_table = match table_select {
            PatternTableSelect::Left => 0,
            PatternTableSelect::Right => 0b0001_0000_0000_0000,
        };

        if self.pattern_table_select.get() != table_select {
            println!("pattern table select changed from {:?} to {:?}",
                     self.pattern_table_select,
                     table_select);
            self.pattern_table_select.set(table_select);
        }

        let plane_row_addr = pattern_table | column_and_row | plane | fine_y;
        vram.read(plane_row_addr)
    }
}
