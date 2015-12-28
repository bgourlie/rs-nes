mod registers;
mod virtual_frame_buffer;
mod vram;

use ppu::registers::*;
use ppu::vram::*;

pub struct Ppu {
    registers: Registers,
    ram: Vram,
}

impl Ppu {
    pub fn new() -> Self {
        Ppu {
            registers: Registers::new(),
            ram: Vram::new(),
        }
    }

    pub fn write_reg(&mut self, addr: u16, val: u8) {
        match addr & 0x7 {
            0x0 => {
                *self.registers.ppu_ctrl = val;
            }
            0x1 => {
                *self.registers.ppu_mask = val;
            }
            0x2 => {
                *self.registers.ppu_status = val;
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
                self.registers.ppu_addr.write(val);
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

    pub fn read_reg(&mut self, addr: u16, val: u8) -> u8 {
        match addr & 0x7 {
            0x0 => {
                *self.registers.ppu_ctrl
            }
            0x1 => {
                *self.registers.ppu_mask
            }
            0x2 => {
                *self.registers.ppu_status
            }
            0x3 => {
                unimplemented!();
            }
            0x4 => {
                // OAMDATA
                // See notes regarding OAMDATA reads here:
                // http://wiki.nesdev.com/w/index.php/PPU_registers
                panic!("Does this need to be implemented?");
            }
            0x5 => {
                unimplemented!();
            }
            0x6 => {
                unimplemented!();
            }
            0x7 => {
                unimplemented!();
            }
            _ => {
                panic!("This should never happen");
            }
        }
    }
}
