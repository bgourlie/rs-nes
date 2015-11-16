mod registers;
mod virtual_frame_buffer;

use memory::*;
use ppu::registers::*;

struct Ppu<'a> {
    registers: Registers,
    memory: &'a mut Memory,
}

impl<'a> Ppu<'a> {
    pub fn new(memory: &'a mut Memory) -> Self {
        Ppu {
            registers: Registers::new(),
            memory: memory,
        }
    }
}
