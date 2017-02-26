mod nes_screen;

pub use self::nes_screen::NesScreen;
#[cfg(feature = "debugger")]
use serde::{Serialize, Serializer};
#[cfg(feature = "debugger")]
use serde::ser::SerializeStruct;

pub trait Screen: Default + Send + Clone + 'static {
    fn put_pixel(&mut self, _: Pixel, _: usize, _: usize) {}
    fn dimensions() -> (usize, usize) {
        (0, 0)
    }
}

#[derive(Copy, Clone, Default)]
pub struct Pixel(u8, u8, u8);

#[derive(Copy, Clone, Default)]
pub struct NoScreen;

#[cfg(feature = "debugger")]
impl Serialize for NoScreen {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("ScreenSnapshot", 3)?;
        let empty_slice = Vec::<u8>::new();
        state.serialize_field("height", &0)?;
        state.serialize_field("width", &0)?;
        state.serialize_field("buffer", &empty_slice)?;
        state.end()
    }
}

impl Screen for NoScreen {}
