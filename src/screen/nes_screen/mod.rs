use super::*;
const SCREEN_WIDTH: usize = 256;
const SCREEN_HEIGHT: usize = 240;

pub struct NesScreen {
    pub screen_buffer: Box<[u8; SCREEN_WIDTH * SCREEN_HEIGHT * 3]>,
}

impl Clone for NesScreen {
    fn clone(&self) -> Self {
        let mut scr = [0_u8; SCREEN_WIDTH * SCREEN_HEIGHT * 3];
        for (i, pixel) in scr.iter_mut().enumerate().take(self.screen_buffer.len()) {
            *pixel = self.screen_buffer[i]
        }
        NesScreen {
            screen_buffer: Box::new(scr),
        }
    }
}

impl Default for NesScreen {
    fn default() -> Self {
        let mut screen = NesScreen {
            screen_buffer: Box::new([0xff; SCREEN_WIDTH * SCREEN_HEIGHT * 3]),
        };
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                screen.put_pixel(x, y, Color(0xf0, 0, 0))
            }
        }
        screen
    }
}

impl Screen for NesScreen {
    fn put_pixel(&mut self, x: usize, y: usize, pixel: Color) {
        let Color(r, g, b) = pixel;
        let i = ((y * SCREEN_WIDTH) + x) * 3;
        self.screen_buffer[i] = r;
        self.screen_buffer[i + 1] = g;
        self.screen_buffer[i + 2] = b;
    }

    fn dimensions() -> (usize, usize) {
        (SCREEN_WIDTH, SCREEN_HEIGHT)
    }
}
