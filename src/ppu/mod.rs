// TODO: Remove once PPU is integrated
#![allow(dead_code)]

#[cfg(test)]
mod spec_tests;

mod palette;
mod control_register;
mod mask_register;
mod status_register;
mod object_attribute_memory;
mod vram;
mod write_latch;
mod pattern;
mod background_renderer;
mod cycle_table;

use self::write_latch::WriteLatch;
use cpu::Interrupt;
use errors::*;
use ppu::background_renderer::BackgroundRenderer;
use ppu::control_register::{ControlRegister, SPRITE_PATTERN_SELECT, SpriteSize};
use ppu::cycle_table::CYCLE_TABLE;
use ppu::mask_register::MaskRegister;
use ppu::object_attribute_memory::{ObjectAttributeMemory, ObjectAttributeMemoryBase,
                                   SpriteAttributes};
use ppu::pattern::Sprite;
use ppu::status_register::StatusRegister;
use ppu::vram::{Vram, VramBase};
use rom::NesRom;
use screen::{NesScreen, Screen};
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
    write_latch: WriteLatch,
    sprite_buffer: [Option<Sprite>; 8],
    background_renderer: BackgroundRenderer,
    odd_frame: bool,
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
            self.oam
                .pixel_color(sprite_palette_index << 2 | sprite_color_index)
        } else {
            self.background_renderer.pixel_color()
        };

        self.screen
            .borrow_mut()
            .put_pixel(x as _, scanline as _, pixel_color);
        Ok(())
    }

    fn fill_secondary_oam(&mut self, scanline: u8) -> Result<()> {
        let pattern_table_base = ((*self.control & SPRITE_PATTERN_SELECT) as u16) << 8;

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
}

impl<V: Vram, O: ObjectAttributeMemory> Ppu for PpuBase<V, O> {
    type Scr = NesScreen;

    fn new(rom: NesRom, screen: Rc<RefCell<Self::Scr>>) -> Self {
        PpuBase {
            cycles: VBLANK_SET_CYCLE,
            control: ControlRegister::default(),
            mask: MaskRegister::default(),
            status: StatusRegister::default(),
            vram: V::new(rom),
            oam: O::default(),
            screen: screen,
            write_latch: WriteLatch::default(),
            sprite_buffer: [None, None, None, None, None, None, None, None],
            background_renderer: BackgroundRenderer::default(),
            odd_frame: false,
        }
    }

    fn step(&mut self) -> Result<Interrupt> {
        let frame_cycle = self.cycles % CYCLES_PER_FRAME;
        let scanline = (frame_cycle / CYCLES_PER_SCANLINE) as u16;
        let x = (frame_cycle % CYCLES_PER_SCANLINE) as u16;

        // Don't rely on self.cycles after the following line
        self.cycles += 1;

        // Fill OAM buffer just before the scanline begins to render.
        // This is not hardware accurate behavior but should produce correct results for most games.
        if scanline < 240 && x == 0 {
            self.fill_secondary_oam(scanline as u8)?;
        }

        match CYCLE_TABLE[scanline as usize][x as usize] {
            0 => {
                if self.mask.rendering_enabled() {
                    // DRAW_PIXEL
                    self.draw_pixel(x, scanline)?;
                }
                Ok(Interrupt::None)
            }
            1 => {
                if self.mask.rendering_enabled() {
                    // INIT_SECONDARY_OAM
                    // TODO

                    // FETCH_NT
                    self.background_renderer
                        .fetch_nametable_byte(&self.vram)?;

                    // DRAW_PIXEL
                    self.draw_pixel(x, scanline)?;
                }
                Ok(Interrupt::None)
            }
            2 => {
                if self.mask.rendering_enabled() {
                    // INIT_SECONDARY_OAM
                    // TODO

                    // SHIFT_BG_REGISTERS
                    self.background_renderer
                        .tick_shifters(self.vram.fine_x());

                    // DRAW_PIXEL
                    self.draw_pixel(x, scanline)?;
                }
                Ok(Interrupt::None)
            }
            3 => {
                if self.mask.rendering_enabled() {
                    // INIT_SECONDARY_OAM
                    // TODO

                    // FETCH_AT
                    self.background_renderer
                        .fetch_attribute_byte(&self.vram)?;

                    // SHIFT_BG_REGISTERS
                    self.background_renderer
                        .tick_shifters(self.vram.fine_x());

                    // DRAW_PIXEL
                    self.draw_pixel(x, scanline)?;
                }
                Ok(Interrupt::None)
            }
            4 => {
                if self.mask.rendering_enabled() {
                    // INIT_SECONDARY_OAM
                    // TODO

                    // FETCH_BG_LOW
                    self.background_renderer
                        .fetch_pattern_low_byte(&self.vram, *self.control)?;

                    // SHIFT_BG_REGISTERS
                    self.background_renderer
                        .tick_shifters(self.vram.fine_x());

                    // DRAW_PIXEL
                    self.draw_pixel(x, scanline)?;
                }
                Ok(Interrupt::None)
            }
            5 => {
                if self.mask.rendering_enabled() {
                    // INIT_SECONDARY_OAM
                    // TODO

                    // FETCH_BG_HIGH
                    self.background_renderer
                        .fetch_pattern_high_byte(&self.vram, *self.control)?;

                    // SHIFT_BG_REGISTERS
                    self.background_renderer
                        .tick_shifters(self.vram.fine_x());

                    // DRAW_PIXEL
                    self.draw_pixel(x, scanline)?;
                }
                Ok(Interrupt::None)
            }
            6 => {
                if self.mask.rendering_enabled() {
                    // INIT_SECONDARY_OAM
                    // TODO

                    // INC_COARSE_X
                    self.vram.coarse_x_increment();

                    // SHIFT_BG_REGISTERS
                    self.background_renderer
                        .tick_shifters(self.vram.fine_x());

                    // DRAW_PIXEL
                    self.draw_pixel(x, scanline)?;
                }
                Ok(Interrupt::None)
            }
            7 => {
                if self.mask.rendering_enabled() {
                    // INIT_SECONDARY_OAM
                    // TODO

                    // FETCH_NT
                    self.background_renderer
                        .fetch_nametable_byte(&self.vram)?;

                    // SHIFT_BG_REGISTERS
                    self.background_renderer
                        .tick_shifters(self.vram.fine_x());

                    // FILL_BG_REGISTERS
                    self.background_renderer
                        .fill_shift_registers(self.vram.addr());

                    // DRAW_PIXEL
                    self.draw_pixel(x, scanline)?;
                }
                Ok(Interrupt::None)
            }
            8 => {
                if self.mask.rendering_enabled() {
                    // SPRITE_EVALUATION
                    // TODO

                    // FETCH_NT
                    self.background_renderer
                        .fetch_nametable_byte(&self.vram)?;

                    // SHIFT_BG_REGISTERS
                    self.background_renderer
                        .tick_shifters(self.vram.fine_x());

                    // FILL_BG_REGISTERS
                    self.background_renderer
                        .fill_shift_registers(self.vram.addr());

                    // DRAW_PIXEL
                    self.draw_pixel(x, scanline)?;
                }
                Ok(Interrupt::None)
            }
            9 => {
                if self.mask.rendering_enabled() {
                    // SPRITE_EVALUATION
                    // TODO

                    // SHIFT_BG_REGISTERS
                    self.background_renderer
                        .tick_shifters(self.vram.fine_x());

                    // DRAW_PIXEL
                    self.draw_pixel(x, scanline)?;
                }
                Ok(Interrupt::None)
            }
            10 => {
                if self.mask.rendering_enabled() {
                    // SPRITE_EVALUATION
                    // TODO

                    // FETCH_AT
                    self.background_renderer
                        .fetch_attribute_byte(&self.vram)?;

                    // SHIFT_BG_REGISTERS
                    self.background_renderer
                        .tick_shifters(self.vram.fine_x());

                    // DRAW_PIXEL
                    self.draw_pixel(x, scanline)?;
                }
                Ok(Interrupt::None)
            }
            11 => {
                if self.mask.rendering_enabled() {
                    // SPRITE_EVALUATION
                    // TODO

                    // FETCH_BG_LOW
                    self.background_renderer
                        .fetch_pattern_low_byte(&self.vram, *self.control)?;

                    // SHIFT_BG_REGISTERS
                    self.background_renderer
                        .tick_shifters(self.vram.fine_x());

                    // DRAW_PIXEL
                    self.draw_pixel(x, scanline)?;
                }
                Ok(Interrupt::None)
            }
            12 => {
                if self.mask.rendering_enabled() {
                    // SPRITE_EVALUATION
                    // TODO

                    // FETCH_BG_HIGH
                    self.background_renderer
                        .fetch_pattern_high_byte(&self.vram, *self.control)?;

                    // SHIFT_BG_REGISTERS
                    self.background_renderer
                        .tick_shifters(self.vram.fine_x());

                    // DRAW_PIXEL
                    self.draw_pixel(x, scanline)?;
                }
                Ok(Interrupt::None)
            }
            13 => {
                if self.mask.rendering_enabled() {
                    // SPRITE_EVALUATION
                    // TODO

                    // INC_COARSE_X
                    self.vram.coarse_x_increment();

                    // SHIFT_BG_REGISTERS
                    self.background_renderer
                        .tick_shifters(self.vram.fine_x());

                    // DRAW_PIXEL
                    self.draw_pixel(x, scanline)?;
                }
                Ok(Interrupt::None)
            }
            14 => {
                if self.mask.rendering_enabled() {
                    // SPRITE_EVALUATION
                    // TODO

                    // INC_FINE_Y
                    self.vram.fine_y_increment();

                    // SHIFT_BG_REGISTERS
                    self.background_renderer
                        .tick_shifters(self.vram.fine_x());
                }
                Ok(Interrupt::None)
            }
            15 => {
                if self.mask.rendering_enabled() {
                    // HORI_V_EQ_HORI_T
                    self.vram.copy_horizontal_pos_to_addr();

                    // FETCH_NT
                    self.background_renderer
                        .fetch_nametable_byte(&self.vram)?;

                    // SHIFT_BG_REGISTERS
                    self.background_renderer
                        .tick_shifters(self.vram.fine_x());

                    // FILL_BG_REGISTERS
                    self.background_renderer
                        .fill_shift_registers(self.vram.addr());
                }
                Ok(Interrupt::None)
            }
            16 => {
                // NOP

                Ok(Interrupt::None)
            }
            17 => {
                if self.mask.rendering_enabled() {
                    // FETCH_SPRITE_LOW
                    // TODO
                }
                Ok(Interrupt::None)
            }
            18 => {
                if self.mask.rendering_enabled() {
                    // FETCH_SPRITE_HIGH
                    // TODO
                }
                Ok(Interrupt::None)
            }
            19 => {
                if self.mask.rendering_enabled() {
                    // FETCH_NT
                    self.background_renderer
                        .fetch_nametable_byte(&self.vram)?;
                }
                Ok(Interrupt::None)
            }
            20 => {
                if self.mask.rendering_enabled() {
                    // SHIFT_BG_REGISTERS
                    self.background_renderer
                        .tick_shifters(self.vram.fine_x());
                }
                Ok(Interrupt::None)
            }
            21 => {
                if self.mask.rendering_enabled() {
                    // FETCH_AT
                    self.background_renderer
                        .fetch_attribute_byte(&self.vram)?;

                    // SHIFT_BG_REGISTERS
                    self.background_renderer
                        .tick_shifters(self.vram.fine_x());
                }
                Ok(Interrupt::None)
            }
            22 => {
                if self.mask.rendering_enabled() {
                    // FETCH_BG_LOW
                    self.background_renderer
                        .fetch_pattern_low_byte(&self.vram, *self.control)?;

                    // SHIFT_BG_REGISTERS
                    self.background_renderer
                        .tick_shifters(self.vram.fine_x());
                }
                Ok(Interrupt::None)
            }
            23 => {
                if self.mask.rendering_enabled() {
                    // FETCH_BG_HIGH
                    self.background_renderer
                        .fetch_pattern_high_byte(&self.vram, *self.control)?;

                    // SHIFT_BG_REGISTERS
                    self.background_renderer
                        .tick_shifters(self.vram.fine_x());
                }
                Ok(Interrupt::None)
            }
            24 => {
                if self.mask.rendering_enabled() {
                    // INC_COARSE_X
                    self.vram.coarse_x_increment();

                    // SHIFT_BG_REGISTERS
                    self.background_renderer
                        .tick_shifters(self.vram.fine_x());
                }
                Ok(Interrupt::None)
            }
            25 => {
                if self.mask.rendering_enabled() {
                    // FETCH_NT
                    self.background_renderer
                        .fetch_nametable_byte(&self.vram)?;

                    // SHIFT_BG_REGISTERS
                    self.background_renderer
                        .tick_shifters(self.vram.fine_x());

                    // FILL_BG_REGISTERS
                    self.background_renderer
                        .fill_shift_registers(self.vram.addr());
                }
                Ok(Interrupt::None)
            }
            26 => {
                if self.mask.rendering_enabled() {
                    // FETCH_AT
                    self.background_renderer
                        .fetch_attribute_byte(&self.vram)?;
                }
                Ok(Interrupt::None)
            }
            27 => {
                if self.mask.rendering_enabled() {
                    // SHIFT_BG_REGISTERS
                    self.background_renderer
                        .tick_shifters(self.vram.fine_x());

                    // FILL_BG_REGISTERS
                    self.background_renderer
                        .fill_shift_registers(self.vram.addr());
                }
                Ok(Interrupt::None)
            }
            28 => {
                // SET_VBLANK
                self.status.set_in_vblank();
                if self.control.nmi_on_vblank_start() {
                    Ok(Interrupt::Nmi)
                } else {
                    Ok(Interrupt::None)
                }
            }
            29 => {
                // CLEAR_VBLANK_AND_SPRITE_ZERO_HIT

                // Reading palettes here isn't accurate, but should suffice for now
                self.background_renderer.update_palettes(&self.vram)?;
                self.oam.update_palettes(&self.vram)?;

                self.status.clear_in_vblank();
                self.status.clear_sprite_zero_hit();

                if self.mask.rendering_enabled() {
                    // FETCH_NT
                    self.background_renderer
                        .fetch_nametable_byte(&self.vram)?;
                }
                Ok(Interrupt::None)
            }
            30 => {
                if self.mask.rendering_enabled() {
                    // INC_FINE_Y
                    self.vram.fine_y_increment();

                    // SHIFT_BG_REGISTERS
                    self.background_renderer
                        .tick_shifters(self.vram.fine_x());
                }
                Ok(Interrupt::None)
            }
            31 => {
                if self.mask.rendering_enabled() {
                    // VERT_V_EQ_VERT_T
                    self.vram.copy_vertical_pos_to_addr();
                }
                Ok(Interrupt::None)
            }
            32 => {
                if self.mask.rendering_enabled() {
                    // FETCH_SPRITE_LOW
                    // TODO

                    // VERT_V_EQ_VERT_T
                    self.vram.copy_vertical_pos_to_addr();
                }
                Ok(Interrupt::None)
            }
            33 => {
                if self.mask.rendering_enabled() {
                    // FETCH_SPRITE_HIGH
                    // TODO

                    // VERT_V_EQ_VERT_T
                    self.vram.copy_vertical_pos_to_addr();
                }
                Ok(Interrupt::None)
            }
            34 => {
                // ODD_FRAME_SKIP_CYCLE
                // This is the last cycle for odd frames
                // The additional cycle increment puts us to pixel 0,0
                if self.odd_frame && self.mask.show_background() {
                    self.cycles += 1;
                    self.odd_frame = false;
                }

                Ok(Interrupt::None)
            }
            35 => {
                // FRAME_INC
                // This is the last cycle for even frames and when rendering disabled
                self.odd_frame = !self.odd_frame;

                Ok(Interrupt::None)
            }
            _ => unreachable!(),
        }
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
