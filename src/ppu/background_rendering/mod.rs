use errors::*;
use ppu::control_register::PatternTableSelect;
use ppu::vram::Vram;

#[derive(Default)]
pub struct BackgroundRendering {
    pattern_low_shift_register: u16,
    pattern_high_shift_register: u16,
    attr_shift_register: u8,
    attribute_latch: u8,
    nametable_latch: u8,
    pattern_low_latch: u8,
    pattern_high_latch: u8,
    current_pixel: u8,
}

// TODO tests
impl BackgroundRendering {
    pub fn current_pixel(&self) -> u8 {
        self.current_pixel
    }

    pub fn fill_shift_registers(&mut self) {
        self.pattern_low_shift_register = ((self.pattern_low_latch as u16) << 8) |
                                          self.pattern_low_shift_register;

        self.pattern_high_shift_register = ((self.pattern_high_latch as u16) << 8) |
                                           self.pattern_high_shift_register;

        self.attr_shift_register = self.attribute_latch;
    }

    pub fn tick_shifters(&mut self, fine_x: u8) {
        let pattern_low_bit = ((self.pattern_low_shift_register >> fine_x) & 1) as u8;
        let pattern_high_bit = ((self.pattern_high_shift_register >> fine_x) & 1) as u8;

        let pixel_low_nibble = (pattern_high_bit << 1) | pattern_low_bit;
        let pixel_high_nibble = ((self.attr_shift_register >> fine_x) & 0b11) << 2 as u8;

        self.current_pixel = pixel_high_nibble | pixel_low_nibble;
        self.pattern_low_shift_register >>= 1;
        self.pattern_high_shift_register >>= 1;
        self.attr_shift_register >>= 1;
    }

    pub fn fetch_attribute_byte<V: Vram>(&mut self, vram: &V) -> Result<()> {
        let v = vram.addr();
        let attribute_address = 0x23C0 | (v & 0x0C00) | ((v >> 4) & 0x38) | ((v >> 2) & 0x07);
        self.attribute_latch = vram.read(attribute_address)?;
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

        let plane_row_addr = pattern_table | column_and_row | plane | fine_y;
        vram.read(plane_row_addr)
    }
}
