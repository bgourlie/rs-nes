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
use rom::NesRom;
use screen::{NesScreen, Pixel, Screen};
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

static PALETTE: [Pixel; 64] = [Pixel(0x7C, 0x7C, 0x7C),
                               Pixel(0x00, 0x00, 0xFC),
                               Pixel(0x00, 0x00, 0xBC),
                               Pixel(0x44, 0x28, 0xBC),
                               Pixel(0x94, 0x00, 0x84),
                               Pixel(0xA8, 0x00, 0x20),
                               Pixel(0xA8, 0x10, 0x00),
                               Pixel(0x88, 0x14, 0x00),
                               Pixel(0x50, 0x30, 0x00),
                               Pixel(0x00, 0x78, 0x00),
                               Pixel(0x00, 0x68, 0x00),
                               Pixel(0x00, 0x58, 0x00),
                               Pixel(0x00, 0x40, 0x58),
                               Pixel(0x00, 0x00, 0x00),
                               Pixel(0x00, 0x00, 0x00),
                               Pixel(0x00, 0x00, 0x00),
                               Pixel(0xBC, 0xBC, 0xBC),
                               Pixel(0x00, 0x78, 0xF8),
                               Pixel(0x00, 0x58, 0xF8),
                               Pixel(0x68, 0x44, 0xFC),
                               Pixel(0xD8, 0x00, 0xCC),
                               Pixel(0xE4, 0x00, 0x58),
                               Pixel(0xF8, 0x38, 0x00),
                               Pixel(0xE4, 0x5C, 0x10),
                               Pixel(0xAC, 0x7C, 0x00),
                               Pixel(0x00, 0xB8, 0x00),
                               Pixel(0x00, 0xA8, 0x00),
                               Pixel(0x00, 0xA8, 0x44),
                               Pixel(0x00, 0x88, 0x88),
                               Pixel(0x00, 0x00, 0x00),
                               Pixel(0x00, 0x00, 0x00),
                               Pixel(0x00, 0x00, 0x00),
                               Pixel(0xF8, 0xF8, 0xF8),
                               Pixel(0x3C, 0xBC, 0xFC),
                               Pixel(0x68, 0x88, 0xFC),
                               Pixel(0x98, 0x78, 0xF8),
                               Pixel(0xF8, 0x78, 0xF8),
                               Pixel(0xF8, 0x58, 0x98),
                               Pixel(0xF8, 0x78, 0x58),
                               Pixel(0xFC, 0xA0, 0x44),
                               Pixel(0xF8, 0xB8, 0x00),
                               Pixel(0xB8, 0xF8, 0x18),
                               Pixel(0x58, 0xD8, 0x54),
                               Pixel(0x58, 0xF8, 0x98),
                               Pixel(0x00, 0xE8, 0xD8),
                               Pixel(0x78, 0x78, 0x78),
                               Pixel(0x00, 0x00, 0x00),
                               Pixel(0x00, 0x00, 0x00),
                               Pixel(0xFC, 0xFC, 0xFC),
                               Pixel(0xA4, 0xE4, 0xFC),
                               Pixel(0xB8, 0xB8, 0xF8),
                               Pixel(0xD8, 0xB8, 0xF8),
                               Pixel(0xF8, 0xB8, 0xF8),
                               Pixel(0xF8, 0xA4, 0xC0),
                               Pixel(0xF0, 0xD0, 0xB0),
                               Pixel(0xFC, 0xE0, 0xA8),
                               Pixel(0xF8, 0xD8, 0x78),
                               Pixel(0xD8, 0xF8, 0x78),
                               Pixel(0xB8, 0xF8, 0xB8),
                               Pixel(0xB8, 0xF8, 0xD8),
                               Pixel(0x00, 0xFC, 0xFC),
                               Pixel(0xF8, 0xD8, 0xF8),
                               Pixel(0x00, 0x00, 0x00),
                               Pixel(0x00, 0x00, 0x00)];

pub type PpuImpl = PpuBase<VramBase, ScrollRegisterBase, ObjectAttributeMemoryBase>;

type Palette = [Pixel; 3];

pub trait Ppu {
    type Scr: Screen;

    fn new(rom: NesRom, screen: Rc<RefCell<Self::Scr>>) -> Self;
    fn write(&mut self, addr: u16, val: u8) -> Result<()>;
    fn read(&self, addr: u16) -> Result<u8>;
    fn step(&mut self) -> Result<StepAction>;
    fn dump_registers<T: Write>(&self, writer: &mut T);
}

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
    fn draw_pixel(&mut self, frame_cycle: u64) -> Result<()> {
        let scanline = frame_cycle / CYCLES_PER_SCANLINE;
        let x = frame_cycle % CYCLES_PER_SCANLINE;
        if scanline < 240 && x < 256 {
            let (base, x_index, y_index) = self.nametable_index(x as _, scanline as _);
            let tile_offset = base + 32 * y_index + x_index;
            let palette_index = self.palette_index(base, x as _, scanline as _)?;

            println!("tile_offset: {:0>4X}, palette_index: {}",
                     tile_offset,
                     palette_index);
            let tile = self.vram.read(tile_offset)?;
            let palette = self.background_pixel(tile as u16, (x % 8) as u8, (scanline % 8) as u8)?;

            let (bg, palettes) = self.background_palettes()?; // TODO: perf

            let pixel = match palette {
                0 => bg,
                1 => palettes[palette_index as usize][0],
                2 => palettes[palette_index as usize][1],
                3 => palettes[palette_index as usize][2],
                _ => unreachable!(),
            };

            // FIXME: TEMPORARY and not correct behavior
            if !self.status.sprite_zero_hit() && palette > 0 {
                //self.status.set_sprite_zero_hit();
            }

            self.screen.borrow_mut().put_pixel(x as _, scanline as _, pixel);
        }
        Ok(())
    }

    fn background_pixel(&self, tile: u16, x: u8, y: u8) -> Result<u8> {
        // Graciously adapted from sprocket nes
        let offset = (tile << 4) + (y as u16);
        let offset = offset + self.control.background_pattern_table();

        // Determine the color of this pixel.
        let plane0 = self.vram.read(offset)?;
        let plane1 = self.vram.read(offset + 8)?;
        let bit0 = (plane0 >> ((7 - ((x % 8) as u8)) as usize)) & 1;
        let bit1 = (plane1 >> ((7 - ((x % 8) as u8)) as usize)) & 1;
        Ok((bit1 << 1) | bit0)
    }

    fn palette_index(&self, nametable_base: u16, x: u8, y: u8) -> Result<u8> {
        let tile_x = (x as u16 / 16) % 8;
        let tile_y = (y as u16 / 16) % 8;

        let attr_offset = (nametable_base + 0x3c0) + 8 * tile_y + tile_x;
        let attr = self.vram.read(attr_offset)?;
        println!("attr_offset: {:0>4X}, attr value: {:0>2X}",
                 attr_offset,
                 attr);
        let palette_index = match ((tile_y % 16) / 8 == 0, (tile_x % 16) / 8 == 0) {
            (true, true) => {
                // top left
                attr & 0x3
            }
            (true, false) => {
                // top right
                (attr >> 2) & 0x3
            }
            (false, true) => {
                // bottom left
                (attr >> 4) & 0x3
            }
            (false, false) => {
                // bottom right
                (attr >> 6) & 0x3
            }
        };

        Ok(palette_index)
    }

    fn nametable_index(&self, x: u16, y: u16) -> (u16, u16, u16) {
        // Graciously adapted from sprocket nes
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

    fn background_palettes(&self) -> Result<(Pixel, [Palette; 4])> {
        let bg = self.vram.read(0x3f00)? as usize;
        let bg = PALETTE[bg];

        let color0 = self.vram.read(0x3f01)? as usize;
        let color1 = self.vram.read(0x3f02)? as usize;
        let color2 = self.vram.read(0x3f03)? as usize;
        let palette0: [Pixel; 3] = [PALETTE[color0], PALETTE[color1], PALETTE[color2]];

        let color0 = self.vram.read(0x3f05)? as usize;
        let color1 = self.vram.read(0x3f06)? as usize;
        let color2 = self.vram.read(0x3f07)? as usize;
        let palette1: [Pixel; 3] = [PALETTE[color0], PALETTE[color1], PALETTE[color2]];

        let color0 = self.vram.read(0x3f09)? as usize;
        let color1 = self.vram.read(0x3f0a)? as usize;
        let color2 = self.vram.read(0x3f0b)? as usize;
        let palette2: [Pixel; 3] = [PALETTE[color0], PALETTE[color1], PALETTE[color2]];

        let color0 = self.vram.read(0x3f0d)? as usize;
        let color1 = self.vram.read(0x3f0e)? as usize;
        let color2 = self.vram.read(0x3f0f)? as usize;
        let palette3: [Pixel; 3] = [PALETTE[color0], PALETTE[color1], PALETTE[color2]];

        Ok((bg, [palette0, palette1, palette2, palette3]))
    }
}

impl<V: Vram, S: ScrollRegister, O: ObjectAttributeMemory> Ppu for PpuBase<V, S, O> {
    type Scr = NesScreen;

    fn new(rom: NesRom, screen: Rc<RefCell<Self::Scr>>) -> Self {
        PpuBase {
            cycles: 0,
            control: ControlRegister::default(),
            mask: MaskRegister::default(),
            status: StatusRegister::default(),
            scroll: S::default(),
            vram: V::new(rom),
            oam: O::default(),
            screen: screen,
        }
    }

    fn step(&mut self) -> Result<StepAction> {
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
                self.status.clear_sprite_zero_hit();
                StepAction::None
            }
            _ => StepAction::None,
        };

        self.draw_pixel(frame_cycle)?;
        self.cycles += 1;
        Ok(result)
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
            0x7 => {
                let inc_amount = self.control.vram_addr_increment();
                self.vram.write_ppu_data(val, inc_amount)?
            }
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
            0x7 => {
                let inc_amount = self.control.vram_addr_increment();
                self.vram.read_ppu_data(inc_amount)?
            }
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
                    0];

        writer.write_all(&regs).unwrap()
    }
}
