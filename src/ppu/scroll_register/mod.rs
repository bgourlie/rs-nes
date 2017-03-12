use ppu::write_latch::LatchState;

// TODO: Emulate Scroll vertical position quirk
// Horizontal offsets range from 0 to 255. "Normal" vertical offsets range from 0 to 239, while
// values of 240 to 255 are treated as -16 through -1 in a way, but tile data is incorrectly fetched
// from the attribute table.

#[cfg(test)]
mod spec_tests;

pub trait ScrollRegister: Default {
    fn write(&mut self, latch_state: LatchState);
}


#[derive(Default)]
pub struct ScrollRegisterBase {
    pub x_pos: u8,
    pub y_pos: u8,
}

impl ScrollRegister for ScrollRegisterBase {
    fn write(&mut self, latch_state: LatchState) {
        match latch_state {
            LatchState::FirstWrite(val) => {
                self.x_pos = val;
            }
            LatchState::SecondWrite(val) => {
                self.y_pos = val;
            }
        }
    }
}
