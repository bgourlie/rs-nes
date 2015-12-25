use std::ops::{Deref};

#[cfg(test)]
mod spec_tests;

#[derive(Copy, Clone)]
pub struct PpuAddress {
    reg: u16,
    write_hi_next: bool
}

impl Deref for PpuAddress {
    type Target = u16;

    fn deref(&self) -> &u16 {
        &self.reg
    }
}

impl PpuAddress {
    pub fn new() -> Self {
        PpuAddress {
            reg: 0,
            write_hi_next: true
        }
    }

    pub fn write(&mut self, byte: u8) {
        if self.write_hi_next {
            self.reg = (self.reg & 0x00ff) | ((byte as u16) << 8)
        } else {
            self.reg = (self.reg & 0xff00) | (byte as u16)
        }
        self.write_hi_next = !self.write_hi_next;
    }
}
