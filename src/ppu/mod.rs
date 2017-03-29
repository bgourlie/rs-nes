// TODO: Remove once PPU is integrated
#![allow(dead_code)]

#[cfg(test)]
mod spec_tests;

mod control_register;
mod mask_register;
mod status_register;
mod object_attribute_memory;
mod vram;
mod write_latch;
mod pattern;
mod background_renderer;

use self::write_latch::WriteLatch;
use cpu::Interrupt;
use errors::*;
use ppu::background_renderer::BackgroundRenderer;
use ppu::control_register::{ControlRegister, PatternTableSelect, SpriteSize};
use ppu::mask_register::MaskRegister;
use ppu::object_attribute_memory::{ObjectAttributeMemory, ObjectAttributeMemoryBase,
                                   SpriteAttributes};
use ppu::pattern::Sprite;
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
const TWO_FRAMES: u64 = CYCLES_PER_FRAME * 2;
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

pub type PpuImpl = PpuBase<VramBase, ObjectAttributeMemoryBase>;

pub trait Ppu {
    type Scr: Screen;

    fn new(rom: NesRom, screen: Rc<RefCell<Self::Scr>>) -> Self;
    fn write(&mut self, addr: u16, val: u8) -> Result<()>;
    fn read(&self, addr: u16) -> Result<u8>;
    fn step(&mut self) -> Result<Interrupt>;
    fn dump_registers<T: Write>(&self, writer: &mut T);
}

pub struct PpuBase<V: Vram, O: ObjectAttributeMemory> {
    cycles: u64,
    control: ControlRegister,
    mask: MaskRegister,
    status: StatusRegister,
    vram: V,
    oam: O,
    screen: Rc<RefCell<NesScreen>>,
    sprite_palettes: [Color; 16],
    bg_palettes: [Color; 16],
    write_latch: WriteLatch,
    sprite_buffer: [Option<Sprite>; 8],
    background_renderer: BackgroundRenderer,
}


impl<V: Vram, O: ObjectAttributeMemory> PpuBase<V, O> {
    fn draw_pixel(&mut self, x: u16, scanline: u16) -> Result<()> {
        let bg_pixel = self.background_renderer.current_pixel();

        // draw sprites
        let sprite_pixel = {
            let mut ret = None;
            for i in 0..8 {
                if let Some(ref sprite) = self.sprite_buffer[i] {
                    if let Some(sprite_color_index) = sprite.pixel_at(x) {
                        ret = Some((sprite.palette(), sprite_color_index));
                        if i == 0 && bg_pixel != 0 && self.mask.show_sprites() &&
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

        let pixel_color = if let Some((sprite_palette_index, sprite_color_index)) = sprite_pixel {
            self.sprite_palettes[(sprite_palette_index as usize) << 2 | sprite_color_index as usize]
        } else {
            self.bg_palettes[self.background_renderer.current_pixel() as usize]
        };

        self.screen.borrow_mut().put_pixel(x as _, scanline as _, pixel_color);
        Ok(())
    }

    fn fill_secondary_oam(&mut self, scanline: u8) -> Result<()> {
        let pattern_table_base = match self.control.sprite_pattern_table() {
            PatternTableSelect::Left => 0x0,
            PatternTableSelect::Right => 0x1000,
        };

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

    fn background_palettes(&self) -> Result<[Color; 16]> {
        let bg = self.vram.read(0x3f00)? as usize;
        Ok([PALETTE[bg],
            PALETTE[self.vram.read(0x3f01)? as usize],
            PALETTE[self.vram.read(0x3f02)? as usize],
            PALETTE[self.vram.read(0x3f03)? as usize],
            PALETTE[bg],
            PALETTE[self.vram.read(0x3f05)? as usize],
            PALETTE[self.vram.read(0x3f06)? as usize],
            PALETTE[self.vram.read(0x3f07)? as usize],
            PALETTE[bg],
            PALETTE[self.vram.read(0x3f09)? as usize],
            PALETTE[self.vram.read(0x3f0a)? as usize],
            PALETTE[self.vram.read(0x3f0b)? as usize],
            PALETTE[bg],
            PALETTE[self.vram.read(0x3f0d)? as usize],
            PALETTE[self.vram.read(0x3f0e)? as usize],
            PALETTE[self.vram.read(0x3f0f)? as usize]])
    }

    fn sprite_palettes(&self) -> Result<[Color; 16]> {
        let bg = self.vram.read(0x3f00)? as usize;
        Ok([PALETTE[bg],
            PALETTE[self.vram.read(0x3f11)? as usize],
            PALETTE[self.vram.read(0x3f12)? as usize],
            PALETTE[self.vram.read(0x3f13)? as usize],
            PALETTE[bg],
            PALETTE[self.vram.read(0x3f15)? as usize],
            PALETTE[self.vram.read(0x3f16)? as usize],
            PALETTE[self.vram.read(0x3f17)? as usize],
            PALETTE[bg],
            PALETTE[self.vram.read(0x3f19)? as usize],
            PALETTE[self.vram.read(0x3f1a)? as usize],
            PALETTE[self.vram.read(0x3f1b)? as usize],
            PALETTE[bg],
            PALETTE[self.vram.read(0x3f1d)? as usize],
            PALETTE[self.vram.read(0x3f1e)? as usize],
            PALETTE[self.vram.read(0x3f1f)? as usize]])
    }
}

impl<V: Vram, O: ObjectAttributeMemory> Ppu for PpuBase<V, O> {
    type Scr = NesScreen;

    fn new(rom: NesRom, screen: Rc<RefCell<Self::Scr>>) -> Self {
        let empty: [Color; 16] = [Color(0x00, 0x00, 0x00),
                                  Color(0x00, 0x00, 0x00),
                                  Color(0x00, 0x00, 0x00),
                                  Color(0x00, 0x00, 0x00),
                                  Color(0x00, 0x00, 0x00),
                                  Color(0x00, 0x00, 0x00),
                                  Color(0x00, 0x00, 0x00),
                                  Color(0x00, 0x00, 0x00),
                                  Color(0x00, 0x00, 0x00),
                                  Color(0x00, 0x00, 0x00),
                                  Color(0x00, 0x00, 0x00),
                                  Color(0x00, 0x00, 0x00),
                                  Color(0x00, 0x00, 0x00),
                                  Color(0x00, 0x00, 0x00),
                                  Color(0x00, 0x00, 0x00),
                                  Color(0x00, 0x00, 0x00)];
        PpuBase {
            cycles: VBLANK_SET_CYCLE,
            control: ControlRegister::default(),
            mask: MaskRegister::default(),
            status: StatusRegister::default(),
            vram: V::new(rom),
            oam: O::default(),
            screen: screen,
            sprite_palettes: empty,
            bg_palettes: empty,
            write_latch: WriteLatch::default(),
            sprite_buffer: [None, None, None, None, None, None, None, None],
            background_renderer: BackgroundRenderer::default(),
        }
    }

    fn step(&mut self) -> Result<Interrupt> {
        let frame_cycle = self.cycles % CYCLES_PER_FRAME;
        let scanline = (frame_cycle / CYCLES_PER_SCANLINE) as u16;
        let x = (frame_cycle % CYCLES_PER_SCANLINE) as u16;

        // Cycle-specific behavior
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

        // Fill OAM buffer just before the scanline begins to render.
        // This is not hardware accurate behavior but should produce correct results for most games.
        if scanline < 240 && x == 0 {
            self.fill_secondary_oam(scanline as u8)?;
        }

        // VRAM position increments and copies occur when rendering is enabled
        if self.mask.rendering_enabled() && (scanline < 240 || scanline == 261) {

            if scanline == 261 && x >= 280 && x <= 304 {
                // During dots 280 to 304 of the pre-render scanline (end of vblank), if rendering
                // is enabled, at the end of vblank, shortly after the horizontal bits are copied
                // from t to v at dot 257, the PPU will repeatedly copy the vertical bits from t to
                // v from dots 280 to 304.
                self.vram.copy_vertical_pos_to_addr();
            }

            if (x > 0 && x < 256) || x >= 321 {
                let bg_fetch_cycle = x % 8;

                // Background rendering reads
                match bg_fetch_cycle {
                    0 => {
                        // At dot 256 of each scanline, if rendering is enabled, the PPU increments
                        // the horizontal position in v many times across the scanline, it begins at
                        // dots 328 and 336, and will continue through the next scanline at 8, 16,
                        // 24... 240, 248, 256 (every 8 dots across the scanline until 256). The
                        // effective X scroll coordinate is incremented, which will wrap to the next
                        // nametable appropriately.
                        self.vram.coarse_x_increment();

                        self.background_renderer.fill_shift_registers(self.vram.addr());
                    }

                    // Nametable fetch
                    1 => self.background_renderer.fetch_nametable_byte(&self.vram)?,

                    // Attribute table byte
                    3 => self.background_renderer.fetch_attribute_byte(&self.vram)?,

                    // Tile low
                    5 => {
                        self.background_renderer
                            .fetch_pattern_low_byte(&self.vram, self.control.bg_pattern_table())?
                    }

                    // Tile high
                    7 => {
                        self.background_renderer
                            .fetch_pattern_high_byte(&self.vram, self.control.bg_pattern_table())?
                    }

                    _ => (),
                }
            }

            if x == 256 {
                // If rendering is enabled, fine Y is incremented at dot 256 of each scanline,
                // overflowing to coarse Y, and finally adjusted to wrap among the nametables
                // vertically.
                self.vram.fine_y_increment();
            } else if x == 257 {
                // At dot 257 of each scanline, if rendering is enabled, the PPU copies all bits
                // related to horizontal position from t to v.
                self.vram.copy_horizontal_pos_to_addr();
            }

            // Tick the background rendering shifters
            // See https://forums.nesdev.com/viewtopic.php?f=3&t=10348#p116095 for explanation of
            // specific tick cycles
            if (x > 1 && x < 258) || (x > 321 && x < 338) {
                self.background_renderer.tick_shifters(self.vram.fine_x())
            }

            if x < 256 && scanline < 240 {
                self.draw_pixel(x, scanline)?
            }
        }

        // For odd frames, the cycle at the end of the scanline is skipped (this is done
        // internally by jumping directly from (339,261) to (0,0), replacing the idle tick at
        // the beginning of the first visible scanline with the last tick of the last dummy
        // nametable fetch). For even frames, the last cycle occurs normally.
        //
        // The PPU timing diagram seems to imply that the skipped cycle only occurs when
        // background rendering is enabled, although I haven't found anything that states that
        // outright.
        if self.cycles % TWO_FRAMES == (TWO_FRAMES - 2) && self.mask.show_background() {
            self.cycles += 2;
        } else {
            self.cycles += 1;
        }

        Ok(result)
    }

    /// Accepts a PPU memory mapped address and writes it to the appropriate register
    fn write(&mut self, addr: u16, val: u8) -> Result<()> {
        debug_assert!(addr >= 0x2000 && addr < 0x4000,
                      "Invalid memory mapped ppu address");

        match addr & 7 {
            0x0 => {
                self.control.write(val);
                self.vram.control_write(val);
                if self.control.sprite_size() == SpriteSize::X16 {
                    let msg = "8X16 sprites".to_owned();
                    bail!(ErrorKind::Crash(CrashReason::UnimplementedOperation(msg)));
                }
            }
            0x1 => self.mask.write(val),
            0x2 => (), // readonly
            0x3 => self.oam.write_address(val),
            0x4 => self.oam.write_data(val),
            0x5 => {
                let latch_state = self.write_latch.write(val);
                self.vram.scroll_write(latch_state);
            }
            0x6 => {
                let latch_state = self.write_latch.write(val);
                self.vram.write_ppu_addr(latch_state);
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
                if self.status.in_vblank() || !self.mask.rendering_enabled() {
                    // No OAM addr increment during vblank or forced blank
                    self.oam.read_data()
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
