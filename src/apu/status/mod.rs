#[cfg(test)]
mod spec_tests;

use std::cell::Cell;

pub trait Status: Default {
    fn read(&self) -> u8;
    fn write(&self, val: u8);
}

#[derive(Default)]
pub struct StatusImpl {
    val: Cell<u8>,
}

impl Status for StatusImpl {
    fn read(&self) -> u8 {
        // TODO: If an interrupt flag was set at the same moment of the read, it will read back as 1
        // but it will not be cleared.
        let val = self.val.get();
        self.val.set(0b_1011_1111 & val);
        val
    }

    fn write(&self, val: u8) {
        self.val.set(0b_0111_1111 & val)
    }
}
