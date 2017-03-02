use super::*;
#[cfg(feature = "debugger")]
use base64;
#[cfg(feature = "debugger")]
use png::HasParameters;
#[cfg(feature = "debugger")]
use serde::{Serialize, Serializer};
#[cfg(feature = "debugger")]
use serde::ser::SerializeStruct;

const SCREEN_WIDTH: usize = 256;
const SCREEN_HEIGHT: usize = 240;

pub struct NesScreen {
    screen_buffer: [u8; SCREEN_WIDTH * SCREEN_HEIGHT * 3],
}

impl Default for NesScreen {
    fn default() -> Self {
        NesScreen { screen_buffer: [0; SCREEN_WIDTH * SCREEN_HEIGHT * 3] }
    }
}

impl Clone for NesScreen {
    fn clone(&self) -> Self {
        println!("After clone: {:?}", self.screen_buffer[0]);
        let buffer = self.screen_buffer;
        println!("After clone: {:?}", self.screen_buffer[0]);
        NesScreen { screen_buffer: buffer }
    }
}

#[cfg(feature = "debugger")]
impl Serialize for NesScreen {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let (width, height) = Self::dimensions();
        let mut img_buf = Vec::<u8>::new();
        {
            let mut encoder = png::Encoder::new(&mut img_buf, width as _, height as _);
            encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);
            let mut writer = encoder.write_header().unwrap();
            writer.write_image_data(&self.screen_buffer).unwrap();
        }
        let img_buf = &*img_buf;
        let encoded_img = base64::encode(&img_buf);
        let mut state = serializer.serialize_struct("ScreenSnapshot", 3)?;
        state.serialize_field("height", &height)?;
        state.serialize_field("width", &width)?;
        state.serialize_field("imgData", &encoded_img)?;
        state.end()
    }
}

impl Screen for NesScreen {
    fn put_pixel(&mut self, x: usize, y: usize, pixel: Pixel) {
        let Pixel(r, g, b) = pixel;
        let i = (y * SCREEN_HEIGHT) + x;
        self.screen_buffer[i] = r;
        self.screen_buffer[i + 1] = g;
        self.screen_buffer[i + 2] = b;
        println!("After put pixel: {:?}", self.screen_buffer[0]);
    }

    fn dimensions() -> (usize, usize) {
        (SCREEN_WIDTH, SCREEN_HEIGHT)
    }
}
