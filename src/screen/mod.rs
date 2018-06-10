mod nes_screen;

pub use self::nes_screen::NesScreen;

pub trait Screen: Default + Clone + Send + 'static {
    fn put_pixel(&mut self, _: usize, _: usize, _: Color) {}
    fn dimensions() -> (usize, usize) {
        (0, 0)
    }
}

#[derive(Copy, Clone, Default, Debug)]
pub struct Color(pub u8, pub u8, pub u8);

#[derive(Copy, Clone, Default)]
pub struct NoScreen;

impl Screen for NoScreen {}
