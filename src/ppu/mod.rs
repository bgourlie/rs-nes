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

const SCANLINES: u64 = 262;
const CYCLES_PER_SCANLINE: u64 = 341;
const CYCLES_PER_FRAME: u64 = SCANLINES * CYCLES_PER_SCANLINE;
const VBLANK_SCANLINE: u64 = 241;
const LAST_SCANLINE: u64 = 261;
const VBLANK_SET_CYCLE: u64 = VBLANK_SCANLINE * CYCLES_PER_SCANLINE + 1;
const VBLANK_CLEAR_CYCLE: u64 = LAST_SCANLINE * CYCLES_PER_SCANLINE + 1;

#[derive(Clone)]
pub struct Ppu {
    cycles: u64,
    control: ControlRegister,
    mask: MaskRegister,
    status: StatusRegister,
    scroll: u8,
    vram_addr: u8,
    vram_data: u8,
    oam: RefCell<ObjectAttributeMemory>,
}

impl Ppu {
    pub fn new() -> Self {
        Ppu {
            cycles: 0,
            control: ControlRegister::new(0),
            mask: MaskRegister::new(0),
            status: StatusRegister::new(0),
            scroll: 0,
            vram_addr: 0,
            vram_data: 0,
            oam: RefCell::new(ObjectAttributeMemory::new()),
        }
    }

    pub fn step(&mut self) {
        match self.cycles % CYCLES_PER_FRAME {
            VBLANK_SET_CYCLE => self.status.set_in_vblank(),
            VBLANK_CLEAR_CYCLE => self.status.clear_in_vblank(),
            _ => (), // Do other stuff
        }
        self.cycles += 1;
    }

    /// Accepts a PPU memory mapped address and writes it to the appropriate register
    pub fn memory_mapped_register_write(&mut self, addr: u16, val: u8) {
        debug_assert!(addr >= 0x2000 && addr < 0x4000,
                      "Invalid memory mapped ppu address");
        match addr & 7 {
            0x0 => self.control.set(val),
            0x1 => self.mask.set(val),
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
            0x0 => *self.control,
            0x1 => *self.mask,
            0x2 => {
                let status = self.status.value();
                self.status.clear_in_vblank(); // 0x2002 read clears vblank

                // TODO: Clear PPUSCROLL/PPUADDR address latch
                status
            }
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

        let regs = [*self.control,
                    *self.mask,
                    self.status.value(),
                    0, // Write-only
                    oam.read_data(),
                    0, // Write-only
                    0, // Write-only
                    self.vram_data];

        writer.write(&regs).unwrap()
    }
}
