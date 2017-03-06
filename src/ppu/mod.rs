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
mod pixel;

use errors::*;
use ppu::control_register::{ControlRegister, SpriteSize};
use ppu::mask_register::MaskRegister;
use ppu::object_attribute_memory::{ObjectAttributeMemory, ObjectAttributeMemoryBase};
use ppu::pixel::{BackgroundPixel, Pixel};
use ppu::scroll_register::{ScrollRegister, ScrollRegisterBase};
use ppu::status_register::StatusRegister;
use ppu::vram::{Vram, VramBase};
use rom::NesRom;
use screen::{Color, NesScreen, Screen};
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

static PALETTE: [Color; 64] = [Color(0x7C, 0x7C, 0x7C),
                               Color(0x00, 0x00, 0xFC),
                               Color(0x00, 0x00, 0xBC),
                               Color(0x44, 0x28, 0xBC),
                               Color(0x94, 0x00, 0x84),
                               Color(0xA8, 0x00, 0x20),
                               Color(0xA8, 0x10, 0x00),
                               Color(0x88, 0x14, 0x00),
                               Color(0x50, 0x30, 0x00),
                               Color(0x00, 0x78, 0x00),
                               Color(0x00, 0x68, 0x00),
                               Color(0x00, 0x58, 0x00),
                               Color(0x00, 0x40, 0x58),
                               Color(0x00, 0x00, 0x00),
                               Color(0x00, 0x00, 0x00),
                               Color(0x00, 0x00, 0x00),
                               Color(0xBC, 0xBC, 0xBC),
                               Color(0x00, 0x78, 0xF8),
                               Color(0x00, 0x58, 0xF8),
                               Color(0x68, 0x44, 0xFC),
                               Color(0xD8, 0x00, 0xCC),
                               Color(0xE4, 0x00, 0x58),
                               Color(0xF8, 0x38, 0x00),
                               Color(0xE4, 0x5C, 0x10),
                               Color(0xAC, 0x7C, 0x00),
                               Color(0x00, 0xB8, 0x00),
                               Color(0x00, 0xA8, 0x00),
                               Color(0x00, 0xA8, 0x44),
                               Color(0x00, 0x88, 0x88),
                               Color(0x00, 0x00, 0x00),
                               Color(0x00, 0x00, 0x00),
                               Color(0x00, 0x00, 0x00),
                               Color(0xF8, 0xF8, 0xF8),
                               Color(0x3C, 0xBC, 0xFC),
                               Color(0x68, 0x88, 0xFC),
                               Color(0x98, 0x78, 0xF8),
                               Color(0xF8, 0x78, 0xF8),
                               Color(0xF8, 0x58, 0x98),
                               Color(0xF8, 0x78, 0x58),
                               Color(0xFC, 0xA0, 0x44),
                               Color(0xF8, 0xB8, 0x00),
                               Color(0xB8, 0xF8, 0x18),
                               Color(0x58, 0xD8, 0x54),
                               Color(0x58, 0xF8, 0x98),
                               Color(0x00, 0xE8, 0xD8),
                               Color(0x78, 0x78, 0x78),
                               Color(0x00, 0x00, 0x00),
                               Color(0x00, 0x00, 0x00),
                               Color(0xFC, 0xFC, 0xFC),
                               Color(0xA4, 0xE4, 0xFC),
                               Color(0xB8, 0xB8, 0xF8),
                               Color(0xD8, 0xB8, 0xF8),
                               Color(0xF8, 0xB8, 0xF8),
                               Color(0xF8, 0xA4, 0xC0),
                               Color(0xF0, 0xD0, 0xB0),
                               Color(0xFC, 0xE0, 0xA8),
                               Color(0xF8, 0xD8, 0x78),
                               Color(0xD8, 0xF8, 0x78),
                               Color(0xB8, 0xF8, 0xB8),
                               Color(0xB8, 0xF8, 0xD8),
                               Color(0x00, 0xFC, 0xFC),
                               Color(0xF8, 0xD8, 0xF8),
                               Color(0x00, 0x00, 0x00),
                               Color(0x00, 0x00, 0x00)];

pub type PpuImpl = PpuBase<VramBase, ScrollRegisterBase, ObjectAttributeMemoryBase>;

type Palette = [Color; 4];

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
        if self.control.sprite_size() == SpriteSize::X16 {
            bail!(ErrorKind::Crash(CrashReason::UnimplementedOperation("8X16 sprites".to_owned())));
        }

        let scanline = frame_cycle / CYCLES_PER_SCANLINE;
        let x = frame_cycle % CYCLES_PER_SCANLINE;
        let bg_pixel = BackgroundPixel::new(x as _, scanline as _, self.control.bg_pattern_table());
        if bg_pixel.is_visible() {
            let bg_color = {
                let tile_index = self.vram.read(bg_pixel.name_table_offset)?;
                let attribute_table_entry = self.vram.read(bg_pixel.attribute_table_offset)?;
                let pattern_offset = bg_pixel.pattern_offset(tile_index as u16);
                let pattern_lower = self.vram.read(pattern_offset)?;
                let pattern_upper = self.vram.read(pattern_offset + 8)?;
                let color_index = bg_pixel.color(pattern_lower, pattern_upper) as usize;
                let palette_index = bg_pixel.palette(attribute_table_entry) as usize;
                let palettes = self.background_palettes()?;
                palettes[palette_index][color_index]
            };
            self.screen.borrow_mut().put_pixel(x as _, scanline as _, bg_color);
        }
        Ok(())
    }

    fn sprite_zero_hit(&self) -> bool {
        false
    }

    fn background_palettes(&self) -> Result<[Palette; 4]> {
        let bg = self.vram.read(0x3f00)? as usize;
        let bg = PALETTE[bg];

        let color0 = self.vram.read(0x3f01)? as usize;
        let color1 = self.vram.read(0x3f02)? as usize;
        let color2 = self.vram.read(0x3f03)? as usize;
        let palette0: [Color; 4] = [bg, PALETTE[color0], PALETTE[color1], PALETTE[color2]];

        let color0 = self.vram.read(0x3f05)? as usize;
        let color1 = self.vram.read(0x3f06)? as usize;
        let color2 = self.vram.read(0x3f07)? as usize;
        let palette1: [Color; 4] = [bg, PALETTE[color0], PALETTE[color1], PALETTE[color2]];

        let color0 = self.vram.read(0x3f09)? as usize;
        let color1 = self.vram.read(0x3f0a)? as usize;
        let color2 = self.vram.read(0x3f0b)? as usize;
        let palette2: [Color; 4] = [bg, PALETTE[color0], PALETTE[color1], PALETTE[color2]];

        let color0 = self.vram.read(0x3f0d)? as usize;
        let color1 = self.vram.read(0x3f0e)? as usize;
        let color2 = self.vram.read(0x3f0f)? as usize;
        let palette3: [Color; 4] = [bg, PALETTE[color0], PALETTE[color1], PALETTE[color2]];

        Ok([palette0, palette1, palette2, palette3])
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
