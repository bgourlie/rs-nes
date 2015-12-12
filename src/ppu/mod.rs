mod registers;
mod virtual_frame_buffer;

use ppu::registers::*;

pub struct Ppu {
    registers: Registers,
}

impl Ppu {
    pub fn new() -> Self {
        Ppu { registers: Registers::new() }
    }
}
