#[cfg(test)]
mod spec_tests;

mod control_register;
mod mask_register;
mod status_register;

use std::io::Write;
use self::control_register::ControlRegister;
use self::mask_register::MaskRegister;
use self::status_register::StatusRegister;

#[derive(Clone)]
pub struct Ppu {
    ctrl_reg: ControlRegister, // 0x2000
    mask_reg: MaskRegister, // 0x2001
    status_reg: StatusRegister, // 0x2002
    oam_addr: u8, // 0x2003
    oam_data: u8, // 0x2004
    scroll: u8, // 0x2005
    vram_addr: u8, // 0x2006
    vram_data: u8, // 0x2007
    oam_dma: u8, // 0x4014
}

impl Ppu {
    pub fn new() -> Self {
        Ppu {
            ctrl_reg: ControlRegister::new(0),
            mask_reg: MaskRegister::new(0),
            status_reg: StatusRegister::new(0),
            oam_addr: 0,
            oam_data: 0,
            scroll: 0,
            vram_addr: 0,
            vram_data: 0,
            oam_dma: 0,
        }
    }

    /// Accepts a PPU memory mapped address and writes it to the appropriate register
    pub fn memory_mapped_register_write(&mut self, addr: u16, val: u8) {
        debug_assert!(addr >= 0x2000 && addr < 0x4000,
                      "Invalid memory mapped ppu address");
        match addr & 7 {
            0x0 => self.ctrl_reg.set(val),
            0x1 => self.mask_reg.set(val),
            0x2 => (), // readonly
            0x3 => self.oam_addr = val,
            0x4 => self.oam_data = val,
            0x5 => self.scroll = val,
            0x6 => self.vram_addr = val,
            0x7 => self.vram_data = val,
            _ => panic!("impossible"),
        }
    }

    /// Accepts a PPU memory mapped address and returns the value
    pub fn memory_mapped_register_read(&self, addr: u16) -> u8 {
        debug_assert!(addr >= 0x2000 && addr < 0x4000,
                      "Invalid memory mapped ppu address");
        match addr & 7 {
            0x0 => *self.ctrl_reg,
            0x1 => *self.mask_reg,
            0x2 => *self.status_reg,
            0x4 => self.oam_data,
            0x7 => self.vram_data,
            0x3 | 0x5 | 0x6 => 0, // Write-only
            _ => panic!("impossible"),
        }
    }

    /// Dump register memory
    pub fn dump_registers<T: Write>(&self, writer: &mut T) -> usize {
        let regs = [*self.ctrl_reg,
                    *self.mask_reg,
                    *self.status_reg,
                    self.oam_addr,
                    self.oam_data,
                    self.scroll,
                    self.vram_addr,
                    self.vram_data];

        writer.write(&regs).unwrap()
    }
}
