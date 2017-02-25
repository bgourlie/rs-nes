use super::*;

const SCREEN_WIDTH: usize = 256;
const SCREEN_HEIGHT: usize = 240;

pub struct NesScreen {
    buffer: [Pixel; SCREEN_WIDTH * SCREEN_HEIGHT],
}

impl Default for NesScreen {
    fn default() -> Self {
        NesScreen { buffer: [Pixel::default(); SCREEN_WIDTH * SCREEN_HEIGHT] }
    }
}

impl Screen for NesScreen {
    fn put_pixel(&mut self, pixel: Pixel, x: usize, y: usize) {
        let i = (y * SCREEN_HEIGHT) + x;
        self.buffer[i] = pixel;
    }
}
