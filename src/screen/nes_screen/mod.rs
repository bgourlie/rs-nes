use super::*;
#[cfg(feature = "debugger")]
use base64;
#[cfg(feature = "debugger")]
use serde::{Serialize, Serializer};
#[cfg(feature = "debugger")]
use serde::ser::SerializeStruct;

const SCREEN_WIDTH: usize = 256;
const SCREEN_HEIGHT: usize = 240;

pub struct NesScreen {
    buffer: [u8; SCREEN_WIDTH * SCREEN_HEIGHT * 3],
}

impl Default for NesScreen {
    fn default() -> Self {
        NesScreen { buffer: [0; SCREEN_WIDTH * SCREEN_HEIGHT * 3] }
    }
}

impl Clone for NesScreen {
    fn clone(&self) -> Self {
        let buffer = self.buffer;
        NesScreen { buffer: buffer }
    }
}

#[cfg(feature = "debugger")]
impl Serialize for NesScreen {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("ScreenSnapshot", 3)?;
        let (width, height) = Self::dimensions();
        let encoded_buffer = base64::encode(&self.buffer);
        state.serialize_field("height", &height)?;
        state.serialize_field("width", &width)?;
        state.serialize_field("buffer", &encoded_buffer)?;
        state.end()
    }
}

impl Screen for NesScreen {
    fn put_pixel(&mut self, pixel: Pixel, x: usize, y: usize) {
        let Pixel(r, g, b) = pixel;
        let i = (y * SCREEN_HEIGHT) + x;
        self.buffer[i] = r;
        self.buffer[i + 1] = g;
        self.buffer[i + 2] = b;
    }

    fn dimensions() -> (usize, usize) {
        (SCREEN_WIDTH, SCREEN_HEIGHT)
    }
}
