// TODO: Could we leverage SIMD here??

#[cfg(test)]
mod spec_tests;

use errors::*;
use ppu::palette::{Color, PALETTE};
use ppu::vram::Vram;

#[derive(Default)]
pub struct BackgroundRenderer {
    palettes: [Color; 16],
    pattern_low_shift_register: u16,
    pattern_high_shift_register: u16,
    palette_low_bit_shift_register: u16,
    palette_high_bit_shift_register: u16,
    attr_latch: u8,
    nametable_latch: u8,
    pattern_low_latch: u8,
    pattern_high_latch: u8,
    current_pixel: u8,
}

impl BackgroundRenderer {
    pub fn update_palettes<V: Vram>(&mut self, vram: &V) -> Result<()> {
        let bg = vram.read(0x3f00)? as usize;
        self.palettes = [PALETTE[bg],
                         PALETTE[vram.read(0x3f01)? as usize],
                         PALETTE[vram.read(0x3f02)? as usize],
                         PALETTE[vram.read(0x3f03)? as usize],
                         PALETTE[bg],
                         PALETTE[vram.read(0x3f05)? as usize],
                         PALETTE[vram.read(0x3f06)? as usize],
                         PALETTE[vram.read(0x3f07)? as usize],
                         PALETTE[bg],
                         PALETTE[vram.read(0x3f09)? as usize],
                         PALETTE[vram.read(0x3f0a)? as usize],
                         PALETTE[vram.read(0x3f0b)? as usize],
                         PALETTE[bg],
                         PALETTE[vram.read(0x3f0d)? as usize],
                         PALETTE[vram.read(0x3f0e)? as usize],
                         PALETTE[vram.read(0x3f0f)? as usize]];
        Ok(())
    }

    pub fn current_pixel(&self) -> u8 {
        self.current_pixel
    }

    pub fn pixel_color(&self) -> Color {
        self.palettes[self.current_pixel as usize]
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

    pub fn tick_shifters(&mut self, fine_x: u8) {
        let palette_low = self.palette_low_bit_shift_register << fine_x;
        let palette_high = self.palette_high_bit_shift_register << fine_x;
        let pattern_low = self.pattern_low_shift_register << fine_x;
        let pattern_high = self.pattern_high_shift_register << fine_x;
        self.current_pixel = Self::pixel_mux(palette_high, palette_low, pattern_high, pattern_low);
        self.pattern_low_shift_register <<= 1;
        self.pattern_high_shift_register <<= 1;
        self.palette_low_bit_shift_register <<= 1;
        self.palette_high_bit_shift_register <<= 1;
    }

    fn pixel_mux(palette_high: u16, palette_low: u16, pattern_high: u16, pattern_low: u16) -> u8 {
        let mask = 0x8000;
        let pixel_low = (((pattern_high & mask) >> 14) | ((pattern_low & mask) >> 15)) & 0b11;
        let pixel_high = (((palette_high & mask) >> 12) | ((palette_low & mask) >> 13)) & 0b1100;
        (pixel_high | pixel_low) as u8
    }

    // TODO: Tests
    pub fn fetch_attribute_byte<V: Vram>(&mut self, vram: &V) -> Result<()> {
        let v = vram.addr();
        let attribute_address = 0x23C0 | (v & 0x0C00) | ((v >> 4) & 0x38) | ((v >> 2) & 0x07);
        self.attr_latch = vram.read(attribute_address)?;
        Ok(())
    }

    // TODO: Tests
    pub fn fetch_nametable_byte<V: Vram>(&mut self, vram: &V) -> Result<()> {
        let nametable_address = 0x2000 | (vram.addr() & 0x0FFF);
        self.nametable_latch = vram.read(nametable_address)?;
        Ok(())
    }

    // TODO: Tests
    pub fn fetch_pattern_low_byte<V: Vram>(&mut self, vram: &V, control_reg: u8) -> Result<()> {
        let v = vram.addr();
        let pattern_addr = Self::pattern_address(v, self.nametable_latch, control_reg, true);
        self.pattern_low_latch = vram.read(pattern_addr)?;
        Ok(())
    }

    // TODO: Tests
    pub fn fetch_pattern_high_byte<V: Vram>(&mut self, vram: &V, control_reg: u8) -> Result<()> {
        let v = vram.addr();
        let pattern_addr = Self::pattern_address(v, self.nametable_latch, control_reg, false);
        self.pattern_high_latch = vram.read(pattern_addr)?;
        Ok(())
    }

    // TODO: Tests
    fn pattern_address(v: u16, nametable_byte: u8, control_reg: u8, is_lower_plane: bool) -> u16 {
        let fine_y = (v >> 12) & 0b111;
        let plane = if is_lower_plane { 0 } else { 0b1000 };
        let column_and_row = (nametable_byte as u16) << 4;
        let pattern_table_select = ((control_reg as u16) << 8) & 0x1000;
        pattern_table_select | column_and_row | plane | fine_y
    }
}
