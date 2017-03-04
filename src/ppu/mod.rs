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

use errors::*;
use ppu::control_register::ControlRegister;
use ppu::mask_register::MaskRegister;
use ppu::object_attribute_memory::{ObjectAttributeMemory, ObjectAttributeMemoryBase};
use ppu::scroll_register::{ScrollRegister, ScrollRegisterBase};
use ppu::status_register::StatusRegister;
use ppu::vram::{Vram, VramBase};
use screen::{NesScreen, Screen};
use std::cell::RefCell;
use std::io::Write;
use std::rc::Rc;

const SCANLINES: u64 = 262;
const CYCLES_PER_SCANLINE: u64 = 341;
const CYCLES_PER_FRAME: u64 = SCANLINES * CYCLES_PER_SCANLINE;
const VBLANK_SCANLINE: u64 = 241;
const LAST_SCANLINE: u64 = 261;
const VBLANK_SET_CYCLE: u64 = VBLANK_SCANLINE * CYCLES_PER_SCANLINE + 1;
const VBLANK_CLEAR_CYCLE: u64 = LAST_SCANLINE * CYCLES_PER_SCANLINE + 1;

pub type PpuImpl = PpuBase<VramBase, ScrollRegisterBase, ObjectAttributeMemoryBase>;

pub trait Ppu: Default {
    type Scr: Screen;

    fn new(screen: Rc<RefCell<Self::Scr>>) -> Self;
    fn write(&mut self, addr: u16, val: u8) -> Result<()>;
    fn read(&self, addr: u16) -> Result<u8>;
    fn step(&mut self) -> StepAction;
    fn dump_registers<T: Write>(&self, writer: &mut T);
}

#[derive(Default)]
pub struct PpuBase<V: Vram, S: ScrollRegister, O: ObjectAttributeMemory> {
    cycles: u64,
    control: ControlRegister,
    mask: MaskRegister,
    status: StatusRegister,
    scroll: S,
    vram: V,
    oam: O,
    screen: Rc<RefCell<NesScreen>>,
}


#[derive(Eq, PartialEq)]
pub enum StepAction {
    None,
    VBlankNmi,
}

impl<V: Vram, S: ScrollRegister, O: ObjectAttributeMemory> PpuBase<V, S, O> {
    fn draw_pixel(&mut self, frame_cycle: u64) {
        let scanline = frame_cycle / CYCLES_PER_SCANLINE;
        let x = frame_cycle % CYCLES_PER_SCANLINE;
        if scanline < 240 && x < 256 {
            let (base, x_index, y_index) = self.nametable_index(x as _, scanline as _);
            let vram_addr = base + 32 * y_index + x_index;
            let tile = self.vram.read(vram_addr).unwrap(); // TODO: revisit Result for vram fns
            let pixel = self.background_pixel(tile as u16, (x % 8) as u8, (scanline % 8) as u8);
            println!("rendering {},{}. tile_index: {:0>4X}, tile: {:0>2X}, pixel: {}",
                     x,
                     scanline,
                     vram_addr,
                     tile,
                     pixel);
        }
    }

    fn background_pixel(&self, tile: u16, x: u8, y: u8) -> u8 {
        let offset = (tile << 4) + (y as u16);
        let offset = offset + self.control.background_pattern_table();

        // Determine the color of this pixel.
        let plane0 = self.vram.read(offset).unwrap();
        let plane1 = self.vram.read(offset + 8).unwrap();
        let bit0 = (plane0 >> ((7 - ((x % 8) as u8)) as usize)) & 1;
        let bit1 = (plane1 >> ((7 - ((x % 8) as u8)) as usize)) & 1;
        (bit1 << 1) | bit0
    }

    fn nametable_index(&self, x: u16, y: u16) -> (u16, u16, u16) {
        let tile_x = (x / 8) % 64;
        let tile_y = (y / 8) % 60;

        let base = match (tile_x >= 32, tile_y >= 30) {
            (false, false) => 0x2000,
            (true, false) => 0x2400,
            (false, true) => 0x2800,
            (true, true) => 0x2c00,
        };

        (base, tile_x % 32, tile_y % 30)
    }
}

impl<V: Vram, S: ScrollRegister, O: ObjectAttributeMemory> Ppu for PpuBase<V, S, O> {
    type Scr = NesScreen;

    fn new(screen: Rc<RefCell<Self::Scr>>) -> Self {
        let mut ppu = Self::default();
        ppu.screen = screen;
        ppu
    }

    fn step(&mut self) -> StepAction {
        let frame_cycle = self.cycles % CYCLES_PER_FRAME;
        let result = match frame_cycle {
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

        self.draw_pixel(frame_cycle);
        self.cycles += 1;
        result
    }

    /// Accepts a PPU memory mapped address and writes it to the appropriate register
    fn write(&mut self, addr: u16, val: u8) -> Result<()> {
        debug_assert!(addr >= 0x2000 && addr < 0x4000,
                      "Invalid memory mapped ppu address");
        match addr & 7 {
            0x0 => self.control.write(val),
            0x1 => self.mask.write(val),
            0x2 => (), // readonly
            0x3 => self.oam.write_address(val),
            0x4 => self.oam.write_data(val),
            0x5 => self.scroll.write(val),
            0x6 => self.vram.write_ppu_addr(val),
            0x7 => self.vram.write_ppu_data(val)?,
            _ => unreachable!(),
        }
        Ok(())
    }

    /// Accepts a PPU memory mapped address and returns the value
    fn read(&self, addr: u16) -> Result<u8> {
        debug_assert!(addr >= 0x2000 && addr < 0x4000,
                      "Invalid memory mapped ppu address");
        let val = match addr & 7 {
            0x0 => *self.control,
            0x1 => *self.mask,
            0x2 => {
                let status = self.status.read();
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
            0x7 => self.vram.read_ppu_data()?,
            0x3 | 0x5 | 0x6 => 0, // Write-only
            _ => unreachable!(),
        };
        Ok(val)
    }

    /// Dump register memory
    fn dump_registers<T: Write>(&self, writer: &mut T) {

        let regs = [*self.control,
                    *self.mask,
                    self.status.read(),
                    0, // Write-only
                    self.oam.read_data(),
                    0, // Write-only
                    0, // Write-only
                    self.vram.ppu_data().unwrap()];

        writer.write_all(&regs).unwrap()
    }
}
