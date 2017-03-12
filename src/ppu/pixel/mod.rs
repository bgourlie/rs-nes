mod background_pixel;
mod sprite_pixel;

pub use self::background_pixel::BackgroundPixel;
pub use self::sprite_pixel::SpritePixel;

pub trait Pixel {
    fn palette(&self, attributes: u8) -> u8;
    fn pattern_offset(&self, tile_index: u16) -> u16;
    fn color_index(&self, pattern_lower: u8, pattern_upper: u8) -> u8;
}
