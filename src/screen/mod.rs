mod nes_screen;

pub use self::nes_screen::NesScreen;

pub trait Screen: Default {
    fn put_pixel(&mut self, _: Pixel, _: usize, _: usize) {}
}

#[derive(Copy, Clone, Default)]
pub struct Pixel(u8, u8, u8);

#[derive(Default)]
pub struct NoScreen;

impl Screen for NoScreen {}
