mod registers;
mod virtual_frame_buffer;

use ppu::registers::*;

pub struct Ppu {
    registers: Registers,
    ram: [u8; 2048],
}

impl Ppu {
    pub fn new() -> Self {
        Ppu {
            registers: Registers::new(),
            ram: [0; 2048],
        }
    }
}
