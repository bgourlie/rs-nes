#[cfg(test)]
mod spec_tests;

use std::cell::Cell;

#[derive(Default)]
pub struct StatusRegister {
    val: Cell<u8>,
}

impl StatusRegister {
    pub fn read(&self) -> u8 {
        // TODO: If an interrupt flag was set at the same moment of the read, it will read back as 1
        // but it will not be cleared.
        let val = self.val.get();
        self.val.set(0b_1011_1111 & val);
        val
    }

    pub fn write(&self, val: u8) {
        self.val.set(0b_0111_1111 & val)
    }
}
