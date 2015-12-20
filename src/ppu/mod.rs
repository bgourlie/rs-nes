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

    pub fn write_reg(&mut self, addr: u16, val: u8) {
        match addr & 0x7 {
            0x0 => {
                // PPUCTRL
                unimplemented!();
            }
            0x1 => {
                // PPUMASK
                unimplemented!();
            }
            0x2 => {
                // PPUSTATUS
                unimplemented!();
            }
            0x3 => {
                // OAMADDR
                unimplemented!();
            }
            0x4 => {
                // OAMDATA
                unimplemented!();
            }
            0x5 => {
                // PPUSCROLL
                unimplemented!();
            }
            0x6 => {
                // PPUADDR
                unimplemented!();
            }
            0x7 => {
                // PPUDATA
                unimplemented!();
            }
            _ => {
                panic!("This should never happen");
            }
        }
    }
}
