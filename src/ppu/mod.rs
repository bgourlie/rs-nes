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
use ppu::background_renderer::BackgroundRenderer;
use ppu::control_register::ControlRegister;
use ppu::cycle_table::CYCLE_TABLE;
use ppu::mask_register::MaskRegister;
use ppu::sprite_renderer::{SpritePixel, SpritePriority, SpriteRenderer, SpriteRendererBase};
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
    fn write(&mut self, addr: u16, val: u8);
    fn read(&self, addr: u16) -> u8;
    fn step(&mut self) -> Interrupt;
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
    fn draw_pixel(&mut self, x: u16, scanline: u16) {
        let fine_x = self.vram.fine_x();
        let bg_pixel = self.background_renderer.current_pixel(fine_x);
        let sprite_pixel = self.sprite_renderer.current_pixel();

        let color = match (bg_pixel, sprite_pixel.value) {
            (0, 0) => self.background_renderer.pixel_color(fine_x),
            (0, _) => sprite_pixel.color,
            (_, 0) => self.background_renderer.pixel_color(fine_x),
            (_, _) => {
                if sprite_pixel.priority == SpritePriority::OnTopOfBackground {
                    sprite_pixel.color
                } else {
                    self.background_renderer.pixel_color(fine_x)
                }
            }
        };

        // TODO: Is it appropriate to evaluate sprite zero hit here considering the cycles
        // draw_pixel() is called on?
        if self.sprite_zero_hit(x, bg_pixel, &sprite_pixel) {
            self.status.set_sprite_zero_hit()
        }

        self.screen
            .borrow_mut()
            .put_pixel((x - 2) as _, scanline as _, color);

    }

    // TODO: tests
    fn sprite_zero_hit(&self, x: u16, bg_pixel: u8, sprite_pixel: &SpritePixel) -> bool {
        !self.status.sprite_zero_hit() &&
        (self.mask.show_background() && self.mask.show_sprites()) &&
        ((self.mask.background_render_leftmost_8_px() &&
          self.mask.sprites_render_leftmost_8_px()) || x > 7) && x != 255 &&
        (bg_pixel > 0 && sprite_pixel.value > 0) && sprite_pixel.is_sprite_zero
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

    fn step(&mut self) -> Interrupt {
        let frame_cycle = self.cycles % CYCLES_PER_FRAME;
        let scanline = (frame_cycle / CYCLES_PER_SCANLINE) as u16;
        let x = (frame_cycle % CYCLES_PER_SCANLINE) as u16;

        // Don't rely on self.cycles after the following line
        self.cycles += 1;

        match CYCLE_TABLE[scanline as usize][x as usize] {
            0 => Interrupt::None,
            1 => {
                if self.mask.rendering_enabled() {
                    self.background_renderer.fetch_nametable_byte(&self.vram);
                }
                Interrupt::None
            }
            2 => {
                if self.mask.rendering_enabled() {
                    self.draw_pixel(x, scanline);
                    self.sprite_renderer.dec_x_counters();
                    self.background_renderer.tick_shifters();
                }
                Interrupt::None
            }
            3 => {
                if self.mask.rendering_enabled() {
                    self.draw_pixel(x, scanline);
                    self.sprite_renderer.dec_x_counters();
                    self.background_renderer.fetch_attribute_byte(&self.vram);
                    self.background_renderer.tick_shifters();
                }
                Interrupt::None
            }
            4 => {
                if self.mask.rendering_enabled() {
                    self.draw_pixel(x, scanline);
                    self.sprite_renderer.dec_x_counters();
                    self.background_renderer
                        .fetch_pattern_low_byte(&self.vram, self.control);
                    self.background_renderer.tick_shifters();
                }
                Interrupt::None
            }
            5 => {
                if self.mask.rendering_enabled() {
                    self.draw_pixel(x, scanline);
                    self.sprite_renderer.dec_x_counters();
                    self.background_renderer
                        .fetch_pattern_high_byte(&self.vram, self.control);
                    self.background_renderer.tick_shifters();
                }
                Interrupt::None
            }
            6 => {
                if self.mask.rendering_enabled() {
                    self.draw_pixel(x, scanline);
                    self.sprite_renderer.dec_x_counters();
                    self.vram.coarse_x_increment();
                    self.background_renderer.tick_shifters();
                }
                Interrupt::None
            }
            7 => {
                if self.mask.rendering_enabled() {
                    self.draw_pixel(x, scanline);
                    self.sprite_renderer.dec_x_counters();
                    self.background_renderer.fetch_nametable_byte(&self.vram);
                    self.background_renderer.tick_shifters();
                    self.background_renderer
                        .fill_shift_registers(self.vram.addr());
                }
                Interrupt::None
            }
            8 => {
                if self.mask.rendering_enabled() {
                    self.draw_pixel(x, scanline);
                    self.sprite_renderer.dec_x_counters();
                    self.background_renderer.fetch_nametable_byte(&self.vram);
                    self.background_renderer.tick_shifters();
                    self.background_renderer
                        .fill_shift_registers(self.vram.addr());
                    self.sprite_renderer
                        .start_sprite_evaluation(scanline, self.control);
                    self.sprite_renderer.tick_sprite_evaluation();
                }
                Interrupt::None
            }
            9 => {
                if self.mask.rendering_enabled() {
                    self.draw_pixel(x, scanline);
                    self.sprite_renderer.dec_x_counters();
                    self.background_renderer.tick_shifters();
                    self.sprite_renderer.tick_sprite_evaluation();
                }
                Interrupt::None
            }
            10 => {
                if self.mask.rendering_enabled() {
                    self.draw_pixel(x, scanline);
                    self.sprite_renderer.dec_x_counters();
                    self.background_renderer.fetch_attribute_byte(&self.vram);
                    self.background_renderer.tick_shifters();
                    self.sprite_renderer.tick_sprite_evaluation();
                }
                Interrupt::None
            }
            11 => {
                if self.mask.rendering_enabled() {
                    self.draw_pixel(x, scanline);
                    self.sprite_renderer.dec_x_counters();
                    self.background_renderer
                        .fetch_pattern_low_byte(&self.vram, self.control);
                    self.background_renderer.tick_shifters();
                    self.sprite_renderer.tick_sprite_evaluation();
                }
                Interrupt::None
            }
            12 => {
                if self.mask.rendering_enabled() {
                    self.draw_pixel(x, scanline);
                    self.sprite_renderer.dec_x_counters();
                    self.background_renderer
                        .fetch_pattern_high_byte(&self.vram, self.control);
                    self.background_renderer.tick_shifters();
                    self.sprite_renderer.tick_sprite_evaluation();
                }
                Interrupt::None
            }
            13 => {
                if self.mask.rendering_enabled() {
                    self.draw_pixel(x, scanline);
                    self.sprite_renderer.dec_x_counters();
                    self.vram.coarse_x_increment();
                    self.background_renderer.tick_shifters();
                    self.sprite_renderer.tick_sprite_evaluation();
                }
                Interrupt::None
            }
            14 => {
                if self.mask.rendering_enabled() {
                    self.draw_pixel(x, scanline);
                    self.sprite_renderer.dec_x_counters();
                    self.background_renderer.fetch_nametable_byte(&self.vram);
                    self.background_renderer.tick_shifters();
                    self.background_renderer
                        .fill_shift_registers(self.vram.addr());
                    self.sprite_renderer.tick_sprite_evaluation();
                }
                Interrupt::None
            }
            15 => {
                if self.mask.rendering_enabled() {
                    self.draw_pixel(x, scanline);
                    self.sprite_renderer.dec_x_counters();
                    self.vram.fine_y_increment();
                    self.background_renderer.tick_shifters();
                    self.sprite_renderer.tick_sprite_evaluation();
                }
                Interrupt::None
            }
            16 => {
                if self.mask.rendering_enabled() {
                    self.draw_pixel(x, scanline);
                    self.vram.copy_horizontal_pos_to_addr();
                    self.background_renderer.fetch_nametable_byte(&self.vram);
                    self.background_renderer.tick_shifters();
                    self.background_renderer
                        .fill_shift_registers(self.vram.addr());
                }
                Interrupt::None
            }
            17 => {
                if self.mask.rendering_enabled() {
                    self.sprite_renderer
                        .fill_registers(&self.vram, self.control);
                }
                Interrupt::None
            }
            18 => {
                if self.mask.rendering_enabled() {
                    self.background_renderer.tick_shifters();
                }
                Interrupt::None
            }
            19 => {
                if self.mask.rendering_enabled() {
                    self.background_renderer.fetch_attribute_byte(&self.vram);
                    self.background_renderer.tick_shifters();
                }
                Interrupt::None
            }
            20 => {
                if self.mask.rendering_enabled() {
                    self.background_renderer
                        .fetch_pattern_low_byte(&self.vram, self.control);
                    self.background_renderer.tick_shifters();
                }
                Interrupt::None
            }
            21 => {
                if self.mask.rendering_enabled() {
                    self.background_renderer
                        .fetch_pattern_high_byte(&self.vram, self.control);
                    self.background_renderer.tick_shifters();
                }
                Interrupt::None
            }
            22 => {
                if self.mask.rendering_enabled() {
                    self.vram.coarse_x_increment();
                    self.background_renderer.tick_shifters();
                }
                Interrupt::None
            }
            23 => {
                if self.mask.rendering_enabled() {
                    self.background_renderer.fetch_nametable_byte(&self.vram);
                    self.background_renderer.tick_shifters();
                    self.background_renderer
                        .fill_shift_registers(self.vram.addr());
                }
                Interrupt::None
            }
            24 => {
                if self.mask.rendering_enabled() {
                    self.background_renderer.fetch_attribute_byte(&self.vram);
                }
                Interrupt::None
            }
            25 => {
                if self.mask.rendering_enabled() {
                    self.background_renderer.tick_shifters();
                    self.background_renderer
                        .fill_shift_registers(self.vram.addr());
                }
                Interrupt::None
            }
            26 => {
                self.status.set_in_vblank();
                if self.control.nmi_on_vblank_start() {
                    Interrupt::Nmi
                } else {
                    Interrupt::None
                }
            }
            27 => {
                // Updating palettes here isn't accurate, but should suffice for now
                self.background_renderer.update_palettes(&self.vram);
                self.sprite_renderer.update_palettes(&self.vram);
                self.status.clear_in_vblank();
                self.status.clear_sprite_zero_hit();
                if self.mask.rendering_enabled() {
                    self.background_renderer.fetch_nametable_byte(&self.vram);
                }
                Interrupt::None
            }
            28 => {
                if self.mask.rendering_enabled() {
                    self.sprite_renderer.dec_x_counters();
                    self.background_renderer.tick_shifters();
                }
                Interrupt::None
            }
            29 => {
                if self.mask.rendering_enabled() {
                    self.sprite_renderer.dec_x_counters();
                    self.background_renderer.fetch_attribute_byte(&self.vram);
                    self.background_renderer.tick_shifters();
                }
                Interrupt::None
            }
            30 => {
                if self.mask.rendering_enabled() {
                    self.sprite_renderer.dec_x_counters();
                    self.background_renderer
                        .fetch_pattern_low_byte(&self.vram, self.control);
                    self.background_renderer.tick_shifters();
                }
                Interrupt::None
            }
            31 => {
                if self.mask.rendering_enabled() {
                    self.sprite_renderer.dec_x_counters();
                    self.background_renderer
                        .fetch_pattern_high_byte(&self.vram, self.control);
                    self.background_renderer.tick_shifters();
                }
                Interrupt::None
            }
            32 => {
                if self.mask.rendering_enabled() {
                    self.sprite_renderer.dec_x_counters();
                    self.vram.coarse_x_increment();
                    self.background_renderer.tick_shifters();
                }
                Interrupt::None
            }
            33 => {
                if self.mask.rendering_enabled() {
                    self.sprite_renderer.dec_x_counters();
                    self.background_renderer.fetch_nametable_byte(&self.vram);
                    self.background_renderer.tick_shifters();
                    self.background_renderer
                        .fill_shift_registers(self.vram.addr());
                }
                Interrupt::None
            }
            34 => {
                if self.mask.rendering_enabled() {
                    self.sprite_renderer.dec_x_counters();
                    self.vram.fine_y_increment();
                    self.background_renderer.tick_shifters();
                }
                Interrupt::None
            }
            35 => {
                if self.mask.rendering_enabled() {
                    self.vram.copy_horizontal_pos_to_addr();
                    self.background_renderer.fetch_nametable_byte(&self.vram);
                    self.background_renderer.tick_shifters();
                    self.background_renderer
                        .fill_shift_registers(self.vram.addr());
                }
                Interrupt::None
            }
            36 => {
                if self.mask.rendering_enabled() {
                    self.vram.copy_vertical_pos_to_addr();
                }
                Interrupt::None
            }
            37 => {
                // This is the last cycle for odd frames
                // The additional cycle increment puts us to pixel 0,0
                if self.odd_frame && self.mask.show_background() {
                    self.cycles += 1;
                    self.odd_frame = false;
                }
                Interrupt::None
            }
            38 => {
                // This is the last cycle for even frames and when rendering disabled
                self.odd_frame = !self.odd_frame;
                Interrupt::None
            }
            _ => unreachable!(),
        }
    }

    /// Accepts a PPU memory mapped address and writes it to the appropriate register
    fn write(&mut self, addr: u16, val: u8) {
        debug_assert!(addr >= 0x2000 && addr < 0x4000,
                      "Invalid memory mapped ppu address");

        match addr & 7 {
            0x0 => {
                self.control.write(val);
                self.vram.control_write(val);
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
                self.vram.write_ppu_data(val, inc_amount)
            }
            _ => unreachable!(),
        }
    }

    /// Accepts a PPU memory mapped address and returns the value
    fn read(&self, addr: u16) -> u8 {
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
                self.vram.read_ppu_data(inc_amount)
            }
            0x3 | 0x5 | 0x6 => 0, // Write-only
            _ => unreachable!(),
        };
        val
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
