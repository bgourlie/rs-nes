mod background_pixel;
mod sprite_pixel;

pub use self::background_pixel::BackgroundPixel;

pub trait Pixel {
    fn palette(&self, attributes: u8) -> u8;
    fn pattern_offset(&self, tile_index: u16) -> u16;
    fn coords(&self) -> (u16, u16);

    fn color(&self, pattern_lower: u8, pattern_upper: u8) -> u8 {
        let (x, _) = self.coords();
        let x = x % 8;
        // credit sprocket nes for the fancy bit fiddling
        let bit0 = (pattern_lower >> ((7 - ((x % 8) as u8)) as usize)) & 1;
        let bit1 = (pattern_upper >> ((7 - ((x % 8) as u8)) as usize)) & 1;
        (bit1 << 1) | bit0
    }
}
