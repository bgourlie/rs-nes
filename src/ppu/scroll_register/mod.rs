use std::cell::Cell;

#[cfg(test)]
mod spec_tests;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum LatchState {
    WriteX,
    WriteY,
}

#[derive(Clone)]
pub struct ScrollRegister {
    latch_state: Cell<LatchState>,
    pub x_pos: u8,
    pub y_pos: u8,
}

impl ScrollRegister {
    pub fn new() -> Self {
        ScrollRegister {
            latch_state: Cell::new(LatchState::WriteX),
            x_pos: 0,
            y_pos: 0,
        }
    }

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
