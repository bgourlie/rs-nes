#[cfg(test)]
mod spec_tests;

use std::cell::Cell;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum LatchState {
    FirstWrite(u8),
    SecondWrite(u8),
}

pub struct WriteLatch {
    is_first_write: Cell<bool>,
}

impl Default for WriteLatch {
    fn default() -> Self {
        WriteLatch { is_first_write: Cell::new(true) }
    }
}

impl WriteLatch {
    pub fn write(&self, val: u8) -> LatchState {
        let is_first_write = self.is_first_write.get();
        self.is_first_write.set(!is_first_write);
        match is_first_write {
            true => LatchState::FirstWrite(val),
            false => LatchState::SecondWrite(val),
        }
    }

    pub fn clear(&self) {
        self.is_first_write.set(true)
    }
}
