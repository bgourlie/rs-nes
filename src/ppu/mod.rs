#[cfg(test)]
mod spec_tests;

mod palette;
mod control_register;
mod mask_register;
mod status_register;
mod sprite_renderer;
mod vram;
mod write_latch;
mod background_renderer;
mod cycle_table;

use self::write_latch::WriteLatch;
use cpu::Interrupt;
use errors::*;
use ppu::background_renderer::BackgroundRenderer;
use ppu::control_register::ControlRegister;
use ppu::cycle_table::CYCLE_TABLE;
use ppu::mask_register::MaskRegister;
use ppu::sprite_renderer::{SpriteRenderer, SpriteRendererBase};
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

pub type PpuImpl = PpuBase<VramBase, SpriteRendererBase>;

pub trait Ppu {
    type Scr: Screen;

    fn new(rom: NesRom, screen: Rc<RefCell<Self::Scr>>) -> Self;
    fn write(&mut self, addr: u16, val: u8) -> Result<()>;
    fn read(&self, addr: u16) -> Result<u8>;
    fn step(&mut self) -> Result<Interrupt>;
    fn dump_registers<T: Write>(&self, writer: &mut T);
}

#[derive(Debug, PartialEq)]
pub enum SpriteSize {
    X8, // 8x8
    X16, // 8x16
}

impl Default for SpriteSize {
    fn default() -> Self {
        SpriteSize::X8
    }
}

pub struct PpuBase<V: Vram, S: SpriteRenderer> {
    cycles: u64,
    control: ControlRegister,
    mask: MaskRegister,
    status: StatusRegister,
    vram: V,
    sprite_renderer: S,
    screen: Rc<RefCell<NesScreen>>,
    write_latch: WriteLatch,
    background_renderer: BackgroundRenderer,
    odd_frame: bool,
}

impl<V: Vram, S: SpriteRenderer> PpuBase<V, S> {
    fn draw_pixel(&mut self, x: u16, scanline: u16) -> Result<()> {
        let bg_pixel = self.background_renderer.pixel_color();
        self.screen
            .borrow_mut()
            .put_pixel(x as _, scanline as _, bg_pixel);
        Ok(())
    }
}

impl<V: Vram, S: SpriteRenderer> Ppu for PpuBase<V, S> {
    type Scr = NesScreen;

    fn new(rom: NesRom, screen: Rc<RefCell<Self::Scr>>) -> Self {
        PpuBase {
            cycles: 0,
            control: ControlRegister::default(),
            mask: MaskRegister::default(),
            status: StatusRegister::default(),
            vram: V::new(rom),
            sprite_renderer: S::default(),
            screen: screen,
            write_latch: WriteLatch::default(),
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

        // FIXME: TEMP HACK
        if frame_cycle == 4 {
            self.status.set_sprite_zero_hit()
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
                    // SPRITE_DEC_X
                    self.sprite_renderer.dec_x_counters();

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
                    // SPRITE_DEC_X
                    self.sprite_renderer.dec_x_counters();

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
                    // SPRITE_DEC_X
                    self.sprite_renderer.dec_x_counters();

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
                    // SPRITE_DEC_X
                    self.sprite_renderer.dec_x_counters();

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
                    // SPRITE_DEC_X
                    self.sprite_renderer.dec_x_counters();

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
                    // SPRITE_DEC_X
                    self.sprite_renderer.dec_x_counters();

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
                    // SPRITE_DEC_X
                    self.sprite_renderer.dec_x_counters();

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
                    // SPRITE_DEC_X
                    self.sprite_renderer.dec_x_counters();

                    // FETCH_NT
                    self.background_renderer
                        .fetch_nametable_byte(&self.vram)?;

                    // SHIFT_BG_REGISTERS
                    self.background_renderer
                        .tick_shifters(self.vram.fine_x());

                    // FILL_BG_REGISTERS
                    self.background_renderer
                        .fill_shift_registers(self.vram.addr());

                    // START_SPRITE_EVALUATION
                    self.sprite_renderer
                        .start_sprite_evaluation(scanline, self.control.sprite_size());

                    // TICK_SPRITE_EVALUATION
                    self.sprite_renderer.tick_sprite_evaluation();

                    // DRAW_PIXEL
                    self.draw_pixel(x, scanline)?;
                }
                Ok(Interrupt::None)
            }
            9 => {
                if self.mask.rendering_enabled() {
                    // SPRITE_DEC_X
                    self.sprite_renderer.dec_x_counters();

                    // SHIFT_BG_REGISTERS
                    self.background_renderer
                        .tick_shifters(self.vram.fine_x());

                    // TICK_SPRITE_EVALUATION
                    self.sprite_renderer.tick_sprite_evaluation();

                    // DRAW_PIXEL
                    self.draw_pixel(x, scanline)?;
                }
                Ok(Interrupt::None)
            }
            10 => {
                if self.mask.rendering_enabled() {
                    // SPRITE_DEC_X
                    self.sprite_renderer.dec_x_counters();

                    // FETCH_AT
                    self.background_renderer
                        .fetch_attribute_byte(&self.vram)?;

                    // SHIFT_BG_REGISTERS
                    self.background_renderer
                        .tick_shifters(self.vram.fine_x());

                    // TICK_SPRITE_EVALUATION
                    self.sprite_renderer.tick_sprite_evaluation();

                    // DRAW_PIXEL
                    self.draw_pixel(x, scanline)?;
                }
                Ok(Interrupt::None)
            }
            11 => {
                if self.mask.rendering_enabled() {
                    // SPRITE_DEC_X
                    self.sprite_renderer.dec_x_counters();

                    // FETCH_BG_LOW
                    self.background_renderer
                        .fetch_pattern_low_byte(&self.vram, *self.control)?;

                    // SHIFT_BG_REGISTERS
                    self.background_renderer
                        .tick_shifters(self.vram.fine_x());

                    // TICK_SPRITE_EVALUATION
                    self.sprite_renderer.tick_sprite_evaluation();

                    // DRAW_PIXEL
                    self.draw_pixel(x, scanline)?;
                }
                Ok(Interrupt::None)
            }
            12 => {
                if self.mask.rendering_enabled() {
                    // SPRITE_DEC_X
                    self.sprite_renderer.dec_x_counters();

                    // FETCH_BG_HIGH
                    self.background_renderer
                        .fetch_pattern_high_byte(&self.vram, *self.control)?;

                    // SHIFT_BG_REGISTERS
                    self.background_renderer
                        .tick_shifters(self.vram.fine_x());

                    // TICK_SPRITE_EVALUATION
                    self.sprite_renderer.tick_sprite_evaluation();

                    // DRAW_PIXEL
                    self.draw_pixel(x, scanline)?;
                }
                Ok(Interrupt::None)
            }
            13 => {
                if self.mask.rendering_enabled() {
                    // SPRITE_DEC_X
                    self.sprite_renderer.dec_x_counters();

                    // INC_COARSE_X
                    self.vram.coarse_x_increment();

                    // SHIFT_BG_REGISTERS
                    self.background_renderer
                        .tick_shifters(self.vram.fine_x());

                    // TICK_SPRITE_EVALUATION
                    self.sprite_renderer.tick_sprite_evaluation();

                    // DRAW_PIXEL
                    self.draw_pixel(x, scanline)?;
                }
                Ok(Interrupt::None)
            }
            14 => {
                if self.mask.rendering_enabled() {
                    // SPRITE_DEC_X
                    self.sprite_renderer.dec_x_counters();

                    // FETCH_NT
                    self.background_renderer
                        .fetch_nametable_byte(&self.vram)?;

                    // SHIFT_BG_REGISTERS
                    self.background_renderer
                        .tick_shifters(self.vram.fine_x());

                    // FILL_BG_REGISTERS
                    self.background_renderer
                        .fill_shift_registers(self.vram.addr());

                    // TICK_SPRITE_EVALUATION
                    self.sprite_renderer.tick_sprite_evaluation();

                    // DRAW_PIXEL
                    self.draw_pixel(x, scanline)?;
                }
                Ok(Interrupt::None)
            }
            15 => {
                if self.mask.rendering_enabled() {
                    // SPRITE_DEC_X
                    self.sprite_renderer.dec_x_counters();

                    // INC_FINE_Y
                    self.vram.fine_y_increment();

                    // SHIFT_BG_REGISTERS
                    self.background_renderer
                        .tick_shifters(self.vram.fine_x());

                    // TICK_SPRITE_EVALUATION
                    self.sprite_renderer.tick_sprite_evaluation();
                }
                Ok(Interrupt::None)
            }
            16 => {
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
            17 => {
                // NOP

                Ok(Interrupt::None)
            }
            18 => {
                if self.mask.rendering_enabled() {
                    // FETCH_SPRITE_LOW
                    self.sprite_renderer
                        .fetch_pattern_low_byte(&self.vram, *self.control)?;
                }
                Ok(Interrupt::None)
            }
            19 => {
                if self.mask.rendering_enabled() {
                    // FETCH_SPRITE_HIGH
                    self.sprite_renderer
                        .fetch_pattern_high_byte(&self.vram, *self.control)?;
                }
                Ok(Interrupt::None)
            }
            20 => {
                if self.mask.rendering_enabled() {
                    // FETCH_NT
                    self.background_renderer
                        .fetch_nametable_byte(&self.vram)?;
                }
                Ok(Interrupt::None)
            }
            21 => {
                if self.mask.rendering_enabled() {
                    // SHIFT_BG_REGISTERS
                    self.background_renderer
                        .tick_shifters(self.vram.fine_x());
                }
                Ok(Interrupt::None)
            }
            22 => {
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
            23 => {
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
            24 => {
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
            25 => {
                if self.mask.rendering_enabled() {
                    // INC_COARSE_X
                    self.vram.coarse_x_increment();

                    // SHIFT_BG_REGISTERS
                    self.background_renderer
                        .tick_shifters(self.vram.fine_x());
                }
                Ok(Interrupt::None)
            }
            26 => {
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
            27 => {
                if self.mask.rendering_enabled() {
                    // FETCH_AT
                    self.background_renderer
                        .fetch_attribute_byte(&self.vram)?;
                }
                Ok(Interrupt::None)
            }
            28 => {
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
            29 => {
                // SET_VBLANK
                self.status.set_in_vblank();
                if self.control.nmi_on_vblank_start() {
                    Ok(Interrupt::Nmi)
                } else {
                    Ok(Interrupt::None)
                }
            }
            30 => {
                // CLEAR_VBLANK_AND_SPRITE_ZERO_HIT

                // Reading palettes here isn't accurate, but should suffice for now
                self.background_renderer.update_palettes(&self.vram)?;
                self.sprite_renderer.update_palettes(&self.vram)?;

                self.status.clear_in_vblank();
                self.status.clear_sprite_zero_hit();

                if self.mask.rendering_enabled() {
                    // SPRITE_DEC_X
                    self.sprite_renderer.dec_x_counters();

                    // FETCH_NT
                    self.background_renderer
                        .fetch_nametable_byte(&self.vram)?;
                }
                Ok(Interrupt::None)
            }
            31 => {
                if self.mask.rendering_enabled() {
                    // SPRITE_DEC_X
                    self.sprite_renderer.dec_x_counters();

                    // SHIFT_BG_REGISTERS
                    self.background_renderer
                        .tick_shifters(self.vram.fine_x());
                }
                Ok(Interrupt::None)
            }
            32 => {
                if self.mask.rendering_enabled() {
                    // SPRITE_DEC_X
                    self.sprite_renderer.dec_x_counters();

                    // FETCH_AT
                    self.background_renderer
                        .fetch_attribute_byte(&self.vram)?;

                    // SHIFT_BG_REGISTERS
                    self.background_renderer
                        .tick_shifters(self.vram.fine_x());
                }
                Ok(Interrupt::None)
            }
            33 => {
                if self.mask.rendering_enabled() {
                    // SPRITE_DEC_X
                    self.sprite_renderer.dec_x_counters();

                    // FETCH_BG_LOW
                    self.background_renderer
                        .fetch_pattern_low_byte(&self.vram, *self.control)?;

                    // SHIFT_BG_REGISTERS
                    self.background_renderer
                        .tick_shifters(self.vram.fine_x());
                }
                Ok(Interrupt::None)
            }
            34 => {
                if self.mask.rendering_enabled() {
                    // SPRITE_DEC_X
                    self.sprite_renderer.dec_x_counters();

                    // FETCH_BG_HIGH
                    self.background_renderer
                        .fetch_pattern_high_byte(&self.vram, *self.control)?;

                    // SHIFT_BG_REGISTERS
                    self.background_renderer
                        .tick_shifters(self.vram.fine_x());
                }
                Ok(Interrupt::None)
            }
            35 => {
                if self.mask.rendering_enabled() {
                    // SPRITE_DEC_X
                    self.sprite_renderer.dec_x_counters();

                    // INC_COARSE_X
                    self.vram.coarse_x_increment();

                    // SHIFT_BG_REGISTERS
                    self.background_renderer
                        .tick_shifters(self.vram.fine_x());
                }
                Ok(Interrupt::None)
            }
            36 => {
                if self.mask.rendering_enabled() {
                    // SPRITE_DEC_X
                    self.sprite_renderer.dec_x_counters();

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
            37 => {
                if self.mask.rendering_enabled() {
                    // SPRITE_DEC_X
                    self.sprite_renderer.dec_x_counters();

                    // INC_FINE_Y
                    self.vram.fine_y_increment();

                    // SHIFT_BG_REGISTERS
                    self.background_renderer
                        .tick_shifters(self.vram.fine_x());
                }
                Ok(Interrupt::None)
            }
            38 => {
                if self.mask.rendering_enabled() {
                    // VERT_V_EQ_VERT_T
                    self.vram.copy_vertical_pos_to_addr();
                }
                Ok(Interrupt::None)
            }
            39 => {
                if self.mask.rendering_enabled() {
                    // FETCH_SPRITE_LOW
                    self.sprite_renderer
                        .fetch_pattern_low_byte(&self.vram, *self.control)?;

                    // VERT_V_EQ_VERT_T
                    self.vram.copy_vertical_pos_to_addr();
                }
                Ok(Interrupt::None)
            }
            40 => {
                if self.mask.rendering_enabled() {
                    // FETCH_SPRITE_HIGH
                    self.sprite_renderer
                        .fetch_pattern_high_byte(&self.vram, *self.control)?;

                    // VERT_V_EQ_VERT_T
                    self.vram.copy_vertical_pos_to_addr();
                }
                Ok(Interrupt::None)
            }
            41 => {
                // ODD_FRAME_SKIP_CYCLE
                // This is the last cycle for odd frames
                // The additional cycle increment puts us to pixel 0,0
                if self.odd_frame && self.mask.show_background() {
                    self.cycles += 1;
                    self.odd_frame = false;
                }

                Ok(Interrupt::None)
            }
            42 => {
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
            0x3 => self.sprite_renderer.write_address(val),
            0x4 => self.sprite_renderer.write_data(val),
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
                    self.sprite_renderer.read_data()
                } else {
                    self.sprite_renderer.read_data_increment_addr()
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
                    self.sprite_renderer.read_data(),
                    0, // Write-only
                    0, // Write-only
                    0];

        writer.write_all(&regs).unwrap()
    }
}
