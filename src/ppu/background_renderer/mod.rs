// TODO: Could we leverage SIMD here??

#[cfg(test)]
mod spec_tests;

use ppu::control_register::ControlRegister;
use ppu::palette::{Color, PALETTE};
use ppu::vram::Vram;

#[derive(Default)]
pub struct BackgroundRenderer {
    palettes: [Color; 16],
    shift_registers: [u16; 4], // [pattern_low, pattern_high, palette_low, palette_high]
    attr_latch: u8,
    nametable_latch: u8,
    pattern_low_latch: u8,
    pattern_high_latch: u8,
}

impl BackgroundRenderer {
    pub fn update_palettes<V: Vram>(&mut self, vram: &V) {
        let bg = vram.read(0x3f00) as usize;
        self.palettes = [
            PALETTE[bg],
            PALETTE[vram.read(0x3f01) as usize],
            PALETTE[vram.read(0x3f02) as usize],
            PALETTE[vram.read(0x3f03) as usize],
            PALETTE[bg],
            PALETTE[vram.read(0x3f05) as usize],
            PALETTE[vram.read(0x3f06) as usize],
            PALETTE[vram.read(0x3f07) as usize],
            PALETTE[bg],
            PALETTE[vram.read(0x3f09) as usize],
            PALETTE[vram.read(0x3f0a) as usize],
            PALETTE[vram.read(0x3f0b) as usize],
            PALETTE[bg],
            PALETTE[vram.read(0x3f0d) as usize],
            PALETTE[vram.read(0x3f0e) as usize],
            PALETTE[vram.read(0x3f0f) as usize],
        ];
    }

    pub fn current_pixel(&self, fine_x: u8) -> (u8, Color) {
        let pattern_low = self.shift_registers[0] << fine_x;
        let pattern_high = self.shift_registers[1] << fine_x;
        let palette_low = self.shift_registers[2] << fine_x;
        let palette_high = self.shift_registers[3] << fine_x;
        let pixel = (((pattern_high & 0x8000) >> 14) | ((pattern_low & 0x8000) >> 15)) as u8 & 0b11;

        let palette = (((palette_high & 0x8000) >> 12) | ((palette_low & 0x8000) >> 13)) as u8;
        let palette_index = (palette | pixel) as usize;
        let color = self.palettes[palette_index];
        (pixel, color)
    }

    pub fn fill_shift_registers(&mut self, v: u16) {
        let (palette_low, palette_high) = Self::palette_shift_bytes(v, self.attr_latch);
        self.shift_registers[0] |= self.pattern_low_latch as u16;
        self.shift_registers[1] |= self.pattern_high_latch as u16;
        self.shift_registers[2] |= palette_low as u16;
        self.shift_registers[3] |= palette_high as u16;
    }

    fn palette_shift_bytes(v: u16, attr_byte: u8) -> (u8, u8) {
        // Second bit of coarse x and coarse y determine which bits get loaded into the shift
        // registers.

        // Intentionally shift each bit one too few to effectively multiply the OR'd result by two.
        // This will give us the amount to shift right by, selecting the correct two attribute bits.
        let x_component = v & 0b10;
        let y_component = (v >> 4) & 0b100;
        let shift = (y_component | x_component) as usize;
        let palette_nibble = (attr_byte >> shift) & 0b11;

        // Return (low bits, high bits)
        ((palette_nibble & 1) * 255, (palette_nibble >> 1) * 255)
    }

    pub fn tick_shifters(&mut self) {
        self.shift_registers[0] <<= 1;
        self.shift_registers[1] <<= 1;
        self.shift_registers[2] <<= 1;
        self.shift_registers[3] <<= 1;
    }

    // TODO: Tests
    pub fn fetch_attribute_byte<V: Vram>(&mut self, vram: &V) {
        let v = vram.addr();
        let attribute_address = 0x23C0 | (v & 0x0C00) | ((v >> 4) & 0x38) | ((v >> 2) & 0x07);
        self.attr_latch = vram.read(attribute_address);
    }

    // TODO: Tests
    pub fn fetch_nametable_byte<V: Vram>(&mut self, vram: &V) {
        let nametable_address = 0x2000 | (vram.addr() & 0x0FFF);
        self.nametable_latch = vram.read(nametable_address);
    }

    // TODO: Tests
    pub fn fetch_pattern_low_byte<V: Vram>(&mut self, vram: &V, control: ControlRegister) {
        let v = vram.addr();
        let pattern_addr = Self::pattern_offset(v, self.nametable_latch, control, true);
        self.pattern_low_latch = vram.read(pattern_addr);
    }

    // TODO: Tests
    pub fn fetch_pattern_high_byte<V: Vram>(&mut self, vram: &V, control: ControlRegister) {
        let v = vram.addr();
        let pattern_addr = Self::pattern_offset(v, self.nametable_latch, control, false);
        self.pattern_high_latch = vram.read(pattern_addr);
    }

    // TODO: Tests
    fn pattern_offset(
        v: u16,
        nametable_byte: u8,
        control: ControlRegister,
        is_lower_plane: bool,
    ) -> u16 {
        let fine_y = (v >> 12) & 0b111;
        let plane = if is_lower_plane { 0 } else { 0b1000 };
        let column_and_row = (nametable_byte as u16) << 4;
        control.background_pattern_table_base() | column_and_row | plane | fine_y
    }
}
