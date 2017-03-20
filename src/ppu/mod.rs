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
mod background_pattern;
mod write_latch;
mod pattern;

use self::write_latch::WriteLatch;
use cpu::Interrupt;
use errors::*;
use ppu::background_pattern::BackgroundPattern;
use ppu::control_register::{ControlRegister, SpriteSize};
use ppu::mask_register::MaskRegister;
use ppu::object_attribute_memory::{ObjectAttributeMemory, ObjectAttributeMemoryBase,
                                   SpriteAttributes};
use ppu::pattern::Sprite;
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
    fn step(&mut self) -> Result<Interrupt>;
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
    sprite_palettes: [Palette; 4],
    bg_palettes: [Palette; 4],
    write_latch: WriteLatch,
    sprite_buffer: [Option<Sprite>; 8],
    scroll_x: u16,
    scroll_y: u16,
}


impl<V: Vram, S: ScrollRegister, O: ObjectAttributeMemory> PpuBase<V, S, O> {
    fn draw_pixel(&mut self, x: u16, scanline: u16) -> Result<()> {
        if x < 256 && scanline < 240 {
            let bg_pixel = BackgroundPattern::new(x + self.scroll_x,
                                                  scanline,
                                                  self.control.bg_pattern_table(),
                                                  &self.vram)?;

            let (bg_palette_index, bg_color_index) = {
                (bg_pixel.palette_index(&self.vram)?, bg_pixel.color_index(&self.vram)?)
            };

            // draw sprites
            let sprite_pixel = {
                let mut ret = None;
                for i in 0..8 {
                    if let Some(ref sprite) = self.sprite_buffer[i] {
                        if let Some(sprite_color_index) = sprite.pixel_at(x) {
                            ret = Some((sprite.palette(), sprite_color_index));
                            if i == 0 && bg_color_index != 0 && self.mask.show_sprites() &&
                               self.mask.show_background() &&
                               !((!self.mask.sprites_render_leftmost_8_px() ||
                                  !self.mask.background_render_leftmost_8_px()) &&
                                 x < 8) && x != 255 &&
                               !self.status.sprite_zero_hit() {
                                self.status.set_sprite_zero_hit();
                            }
                            break;
                        }
                    } else {
                        break;
                    }
                }
                ret
            };

            let pixel_color = {
                if let Some((sprite_palette_index, sprite_color_index)) = sprite_pixel {
                    self.sprite_palettes[sprite_palette_index as usize][sprite_color_index as usize]
                } else {
                    self.bg_palettes[bg_palette_index as usize][bg_color_index as usize]
                }
            };

            self.screen.borrow_mut().put_pixel(x as _, scanline as _, pixel_color);
        }
        Ok(())
    }

    fn fill_secondary_oam(&mut self, scanline: u8) -> Result<()> {
        let pattern_table_base = self.control.sprite_pattern_table();
        let mut sprites_on_scanline = 0;
        for i in 0..64 {
            let sprite_attributes = self.oam.sprite_attributes(i);
            if self.is_on_scanline(&sprite_attributes, scanline) {
                let sprite = pattern::Sprite::read(scanline,
                                                   sprite_attributes,
                                                   pattern_table_base,
                                                   &self.vram)?;

                if sprites_on_scanline < 8 {
                    self.sprite_buffer[sprites_on_scanline] = Some(sprite);
                    sprites_on_scanline += 1;
                } else {
                    if !self.status.sprite_overflow() {
                        self.status.set_sprite_overflow();
                    }
                    break;
                }
            }
        }

        // Clear any remaining sprites from last scanline
        for i in sprites_on_scanline..8 {
            self.sprite_buffer[i] = None;
        }

        Ok(())
    }

    fn is_on_scanline(&self, sprite: &SpriteAttributes, scanline: u8) -> bool {
        let height = match self.control.sprite_size() {
            SpriteSize::X8 => 8,
            SpriteSize::X16 => 16,
        };
        scanline >= sprite.y && scanline < sprite.y + height
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

    fn sprite_palettes(&self) -> Result<[Palette; 4]> {
        let bg = self.vram.read(0x3f00)? as usize;
        let bg = PALETTE[bg];

        let color0 = self.vram.read(0x3f11)? as usize;
        let color1 = self.vram.read(0x3f12)? as usize;
        let color2 = self.vram.read(0x3f13)? as usize;
        let palette0: [Color; 4] = [bg, PALETTE[color0], PALETTE[color1], PALETTE[color2]];

        let color0 = self.vram.read(0x3f15)? as usize;
        let color1 = self.vram.read(0x3f16)? as usize;
        let color2 = self.vram.read(0x3f17)? as usize;
        let palette1: [Color; 4] = [bg, PALETTE[color0], PALETTE[color1], PALETTE[color2]];

        let color0 = self.vram.read(0x3f19)? as usize;
        let color1 = self.vram.read(0x3f1a)? as usize;
        let color2 = self.vram.read(0x3f1b)? as usize;
        let palette2: [Color; 4] = [bg, PALETTE[color0], PALETTE[color1], PALETTE[color2]];

        let color0 = self.vram.read(0x3f1d)? as usize;
        let color1 = self.vram.read(0x3f1e)? as usize;
        let color2 = self.vram.read(0x3f1f)? as usize;
        let palette3: [Color; 4] = [bg, PALETTE[color0], PALETTE[color1], PALETTE[color2]];

        Ok([palette0, palette1, palette2, palette3])
    }
}

impl<V: Vram, S: ScrollRegister, O: ObjectAttributeMemory> Ppu for PpuBase<V, S, O> {
    type Scr = NesScreen;

    fn new(rom: NesRom, screen: Rc<RefCell<Self::Scr>>) -> Self {
        let empty: [Color; 4] = [Color(0x00, 0x00, 0x00),
                                 Color(0x00, 0x00, 0x00),
                                 Color(0x00, 0x00, 0x00),
                                 Color(0x00, 0x00, 0x00)];
        PpuBase {
            cycles: VBLANK_SET_CYCLE,
            control: ControlRegister::default(),
            mask: MaskRegister::default(),
            status: StatusRegister::default(),
            scroll: S::default(),
            vram: V::new(rom),
            oam: O::default(),
            screen: screen,
            sprite_palettes: [empty, empty, empty, empty],
            bg_palettes: [empty, empty, empty, empty],
            write_latch: WriteLatch::default(),
            sprite_buffer: [None, None, None, None, None, None, None, None],
            scroll_x: 0,
            scroll_y: 0,
        }
    }

    fn step(&mut self) -> Result<Interrupt> {
        let frame_cycle = self.cycles % CYCLES_PER_FRAME;
        let scanline = (frame_cycle / CYCLES_PER_SCANLINE) as u16;
        let x = (frame_cycle % CYCLES_PER_SCANLINE) as u16;

        // Fill OAM buffer just before the scanline begins to render.
        // This is not hardware accurate behavior but should produce correct results for most games.
        if scanline < 240 && x == 0 {
            self.fill_secondary_oam(scanline as u8)?;
        }

        let result = match frame_cycle {
            VBLANK_SET_CYCLE => {
                self.status.set_in_vblank();
                if self.control.nmi_on_vblank_start() {
                    Interrupt::Nmi
                } else {
                    Interrupt::None
                }
            }
            VBLANK_CLEAR_CYCLE => {
                // Reading palettes here isn't accurate, but should suffice for now
                self.bg_palettes = self.background_palettes()?;
                self.sprite_palettes = self.sprite_palettes()?;

                self.status.clear_in_vblank();
                self.status.clear_sprite_zero_hit();
                Interrupt::None
            }
            _ => Interrupt::None,
        };

        self.draw_pixel(x, scanline)?;
        self.cycles += 1;
        Ok(result)
    }

    /// Accepts a PPU memory mapped address and writes it to the appropriate register
    fn write(&mut self, addr: u16, val: u8) -> Result<()> {
        debug_assert!(addr >= 0x2000 && addr < 0x4000,
                      "Invalid memory mapped ppu address");

        match addr & 7 {
            0x0 => {
                self.control.write(val);
                if self.control.sprite_size() == SpriteSize::X16 {
                    let msg = "8X16 sprites".to_owned();
                    bail!(ErrorKind::Crash(CrashReason::UnimplementedOperation(msg)));
                }

                self.scroll_x = (self.scroll_x & 0x00ff) | self.control.scroll_x_base();
                self.scroll_y = (self.scroll_y & 0x00ff) | self.control.scroll_y_base();
            }
            0x1 => self.mask.write(val),
            0x2 => (), // readonly
            0x3 => self.oam.write_address(val),
            0x4 => self.oam.write_data(val),
            0x5 => {
                let latch_state = self.write_latch.write(val);
                match latch_state {
                    write_latch::LatchState::FirstWrite(x) => {
                        self.scroll_x = (self.scroll_x & 0xff00) | (x as u16);
                    }
                    write_latch::LatchState::SecondWrite(y) => {
                        self.scroll_y = (self.scroll_y & 0xff00) | (y as u16);
                    }
                }
                self.scroll.write(latch_state);
            }
            0x6 => {
                let latch_state = self.write_latch.write(val);
                self.vram.write_ppu_addr(latch_state);
                if let write_latch::LatchState::SecondWrite(_) = latch_state {
                    let addr = self.vram.addr() & 0x07ff;
                    let xscroll_base = if addr < 0x400 { 0 } else { 256 };
                    self.scroll_x = (self.scroll_x & 0xff) | xscroll_base;
                }
            }
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
                self.write_latch.clear();
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
