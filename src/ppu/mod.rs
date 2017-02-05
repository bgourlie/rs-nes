// TODO: Remove once PPU is integrated
#![allow(dead_code)]

#[cfg(test)]
mod spec_tests;

mod control_register;
mod mask_register;
mod status_register;
mod scroll_register;
mod object_attribute_memory;
mod vram;

use ppu::control_register::ControlRegister;
use ppu::mask_register::MaskRegister;
use ppu::object_attribute_memory::ObjectAttributeMemory;
use ppu::scroll_register::ScrollRegister;
use ppu::status_register::StatusRegister;
use ppu::vram::{VramBase, Vram};
use std::io::Write;

const SCANLINES: u64 = 262;
const CYCLES_PER_SCANLINE: u64 = 341;
const CYCLES_PER_FRAME: u64 = SCANLINES * CYCLES_PER_SCANLINE;
const VBLANK_SCANLINE: u64 = 241;
const LAST_SCANLINE: u64 = 261;
const VBLANK_SET_CYCLE: u64 = VBLANK_SCANLINE * CYCLES_PER_SCANLINE + 1;
const VBLANK_CLEAR_CYCLE: u64 = LAST_SCANLINE * CYCLES_PER_SCANLINE + 1;

pub type Ppu = PpuBase<VramBase>;

#[derive(Clone)]
pub struct PpuBase<V: Vram> {
    cycles: u64,
    control: ControlRegister,
    mask: MaskRegister,
    status: StatusRegister,
    scroll: ScrollRegister,
    vram: V,
    oam: ObjectAttributeMemory,
}

#[derive(Eq, PartialEq)]
pub enum StepAction {
    None,
    VBlankNmi,
}

impl<V: Vram> PpuBase<V> {
    pub fn new() -> Self {
        PpuBase {
            cycles: 0,
            control: ControlRegister::new(0),
            mask: MaskRegister::new(0),
            status: StatusRegister::new(0),
            scroll: ScrollRegister::new(),
            vram: V::default(),
            oam: ObjectAttributeMemory::new(),
        }
    }

    pub fn step(&mut self) -> StepAction {
        let result = match self.cycles % CYCLES_PER_FRAME {
            VBLANK_SET_CYCLE => {
                self.status.set_in_vblank();
                if self.control.nmi_on_vblank_start() {
                    StepAction::VBlankNmi
                } else {
                    StepAction::None
                }
            }
            VBLANK_CLEAR_CYCLE => {
                self.status.clear_in_vblank();
                StepAction::None
            }
            _ => StepAction::None,
        };
        self.cycles += 1;
        result
    }

    /// Accepts a PPU memory mapped address and writes it to the appropriate register
    pub fn memory_mapped_register_write(&mut self, addr: u16, val: u8) {
        debug_assert!(addr >= 0x2000 && addr < 0x4000,
                      "Invalid memory mapped ppu address");
        match addr & 7 {
            0x0 => self.control.set(val),
            0x1 => self.mask.set(val),
            0x2 => (), // readonly
            0x3 => self.oam.set_address(val),
            0x4 => self.oam.write_data(val),
            0x5 => self.scroll.write(val),
            0x6 => self.vram.write_address(val),
            0x7 => self.vram.write_data_increment_address(val),
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
                self.status.clear_in_vblank();
                self.scroll.clear_latch();
                self.vram.clear_latch();
                status
            }
            0x4 => {
                if self.status.in_vblank() || self.mask.rendering_disabled() {
                    self.oam.read_data() // No OAM addr increment during vblank or forced blank
                } else {
                    self.oam.read_data_increment_addr()
                }
            }
            0x7 => self.vram.read_data_increment_address(),
            0x3 | 0x5 | 0x6 => 0, // Write-only
            _ => panic!("impossible"),
        }
    }

    /// Dump register memory
    pub fn dump_registers<T: Write>(&self, writer: &mut T) -> usize {

        let regs = [*self.control,
                    *self.mask,
                    self.status.value(),
                    0, // Write-only
                    self.oam.read_data(),
                    0, // Write-only
                    0, // Write-only
                    self.vram.read_data()];

        writer.write(&regs).unwrap()
    }
}
