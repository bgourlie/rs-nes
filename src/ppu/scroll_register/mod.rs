use std::cell::Cell;

// TODO: Emulate Scroll vertical position quirk
// Horizontal offsets range from 0 to 255. "Normal" vertical offsets range from 0 to 239, while
// values of 240 to 255 are treated as -16 through -1 in a way, but tile data is incorrectly fetched
// from the attribute table.

#[cfg(test)]
mod spec_tests;

pub trait ScrollRegister: Default {
    fn write(&mut self, pos: u8);
    fn clear_latch(&self);
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum LatchState {
    WriteX,
    WriteY,
}

impl Default for LatchState {
    fn default() -> Self {
        LatchState::WriteX
    }
}

#[derive(Default)]
pub struct ScrollRegisterBase {
    latch_state: Cell<LatchState>,
    pub x_pos: u8,
    pub y_pos: u8,
}

impl ScrollRegisterBase {
    pub fn write(&mut self, pos: u8) {
        match self.latch_state.get() {
            LatchState::WriteX => {
                self.x_pos = pos;
                self.latch_state.set(LatchState::WriteY)
            }
            LatchState::WriteY => {
                self.y_pos = pos;
                self.latch_state.set(LatchState::WriteX)
            }
        }
    }

    pub fn clear_latch(&self) {
        self.latch_state.set(LatchState::WriteX)
    }
}

impl ScrollRegister for ScrollRegisterBase {
    fn write(&mut self, pos: u8) {
        self.write(pos)
    }

    fn clear_latch(&self) {
        self.clear_latch()
    }
}
