#[cfg(test)]
mod spec_tests;

#[derive(Debug, Eq, PartialEq)]
pub enum Quadrant {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl Quadrant {
    pub fn palette_index(&self, attr: u8) -> u8 {
        match *self {
            Quadrant::TopLeft => {
                // top left
                attr & 0x3
            }
            Quadrant::TopRight => {
                // top right
                (attr >> 2) & 0x3
            }
            Quadrant::BottomLeft => {
                // bottom left
                (attr >> 4) & 0x3
            }
            Quadrant::BottomRight => {
                // bottom right
                (attr >> 6) & 0x3
            }
        }
    }
}

#[derive(Copy, Clone)]
pub struct Pixel(pub u8, pub u8);

impl Pixel {
    pub fn tile16_coords(&self) -> (u8, u8) {
        let Pixel(x, y) = *self;
        let tile_x = x / 16;
        let tile_y = y / 16;
        (tile_x, tile_y)
    }

    pub fn tile32_coords(&self) -> (u8, u8) {
        let Pixel(x, y) = *self;
        let tile_x = x / 32;
        let tile_y = y / 32;
        (tile_x, tile_y)
    }

    pub fn tile32_quadrant(&self) -> Quadrant {
        let (x, y) = self.tile16_coords();

        match (y % 2 == 0, x % 2 == 0) {
            (true, true) => Quadrant::TopLeft,
            (true, false) => Quadrant::TopRight,
            (false, true) => Quadrant::BottomLeft,
            (false, false) => Quadrant::BottomRight,
        }
    }
}
