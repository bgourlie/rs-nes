// TODO: Remove once PPU is integrated
#![allow(dead_code)]

#[cfg(test)]
mod spec_tests;

mod control_register;
mod mask_register;
mod status_register;
mod object_attribute_memory;

use ppu::control_register::ControlRegister;
use ppu::mask_register::MaskRegister;
use ppu::object_attribute_memory::ObjectAttributeMemory;
use ppu::status_register::StatusRegister;
use std::cell::RefCell;
use std::io::Write;

#[derive(Clone)]
pub struct Ppu {
    ctrl_reg: ControlRegister,
    mask_reg: MaskRegister,
    status_reg: StatusRegister,
    scroll: u8,
    vram_addr: u8,
    vram_data: u8,
    cycles: usize,
    oam: RefCell<ObjectAttributeMemory>,
}

impl Ppu {
    pub fn new() -> Self {
        Ppu {
            ctrl_reg: ControlRegister::new(0),
            mask_reg: MaskRegister::new(0),
            status_reg: StatusRegister::new(0),
            scroll: 0,
            vram_addr: 0,
            vram_data: 0,
            cycles: 0,
            oam: RefCell::new(ObjectAttributeMemory::new()),
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
            0x3 => self.oam_set_address(val),
            0x4 => self.oam_write_data(val),
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
            0x4 => self.oam_read_data(),
            0x7 => self.vram_data,
            0x3 | 0x5 | 0x6 => 0, // Write-only
            _ => panic!("impossible"),
        }
    }

    fn oam_read_data(&self) -> u8 {
        let mut oam = self.oam.borrow_mut();
        oam.read_data_increment_addr()
    }

    fn oam_write_data(&self, data: u8) {
        let mut oam = self.oam.borrow_mut();
        oam.write_data(data);
    }

    fn oam_set_address(&self, address: u8) {
        let mut oam = self.oam.borrow_mut();
        oam.set_address(address);
    }

    #[cfg(test)]
    fn oam_address(&self) -> u8 {
        let oam = self.oam.borrow();
        oam.address()
    }

    /// Dump register memory
    pub fn dump_registers<T: Write>(&self, writer: &mut T) -> usize {
        let oam = self.oam.borrow();

        let regs = [*self.ctrl_reg,
                    *self.mask_reg,
                    *self.status_reg,
                    0, // Write-only
                    oam.read_data(),
                    0, // Write-only
                    0, // Write-only
                    self.vram_data];

        writer.write(&regs).unwrap()
    }
}
