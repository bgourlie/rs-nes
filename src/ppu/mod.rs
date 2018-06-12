#[cfg(test)]
pub mod mocks;

#[cfg(test)]
mod spec_tests;

#[cfg(test)]
mod bench_test;

mod background_renderer;
mod control_register;
mod mask_register;
mod sprite_renderer;
mod status_register;
mod vram;
mod write_latch;

use self::write_latch::WriteLatch;
use cpu6502::cpu::Interrupt;
use ppu::background_renderer::BackgroundRenderer;
use ppu::control_register::ControlRegister;
use ppu::mask_register::MaskRegister;
pub use ppu::sprite_renderer::SpriteRenderer;
use ppu::sprite_renderer::{ISpriteRenderer, SpritePixel};
use ppu::status_register::StatusRegister;
use ppu::vram::IVram;
pub use ppu::vram::Vram;
use rs_nes_macros::ppu_loop;

const SCANLINES: usize = 262;
const CYCLES_PER_SCANLINE: usize = 341;
const CYCLES_PER_FRAME: usize = SCANLINES * CYCLES_PER_SCANLINE;
pub const SCREEN_WIDTH: usize = 256;
pub const SCREEN_HEIGHT: usize = 240;

pub trait IPpu {
    fn write(&mut self, addr: u16, val: u8);
    fn read(&self, addr: u16) -> u8;
    fn step(&mut self) -> Interrupt;
    fn screen(&self) -> &[u8; SCREEN_WIDTH * SCREEN_HEIGHT * 2];
}

#[derive(Debug, PartialEq)]
pub enum SpriteSize {
    X8,  // 8x8
    X16, // 8x16
}

impl Default for SpriteSize {
    fn default() -> Self {
        SpriteSize::X8
    }
}

pub struct Ppu<V: IVram, S: ISpriteRenderer> {
    cycles: usize,
    control: ControlRegister,
    mask: MaskRegister,
    status: StatusRegister,
    vram: V,
    sprite_renderer: S,
    screen: [u8; SCREEN_WIDTH * SCREEN_HEIGHT * 2],
    write_latch: WriteLatch,
    background_renderer: BackgroundRenderer,
    odd_frame: bool,
}

impl<V: IVram, S: ISpriteRenderer> Ppu<V, S> {
    pub fn new(vram: V) -> Self {
        Ppu {
            cycles: 0,
            control: ControlRegister::default(),
            mask: MaskRegister::default(),
            status: StatusRegister::default(),
            vram,
            sprite_renderer: S::default(),
            screen: [0; SCREEN_WIDTH * SCREEN_HEIGHT * 2],
            write_latch: WriteLatch::default(),
            background_renderer: BackgroundRenderer::default(),
            odd_frame: false,
        }
    }

    /// Outputs pixel information to a buffer. Each pixel is encoded as two bytes, as follows:
    ///
    /// **Byte 1 (palette indices)**: `bbbb ssss`
    ///
    /// - `b`: Background pixel palette index
    /// - `s`: Sprite pixel palette index
    ///
    /// **Byte 2 (pixel properties)**: `bbss rgbp`
    ///
    /// - `b`: 2-bit pixel value
    /// - `s`: 2-bit pixel value
    /// - `r`: Emphasize red (not yet implemented)
    /// - `g`: Emphasize green (not yet implemented)
    /// - `b`: Emphasize blue (not yet implemented)
    /// - `p`: Sprite pixel priority
    ///
    /// This format will need to be decoded and properly displayed by the front-end.
    fn draw_pixel(&mut self, x: u16, scanline: u16) {
        let fine_x = self.vram.fine_x();
        let (bg_pixel, bg_color) = self.background_renderer.current_pixel(fine_x);
        let sprite_pixel = self.sprite_renderer.current_pixel();
        let color_byte = (bg_color << 4) | sprite_pixel.color_index;
        let property_byte =
            sprite_pixel.has_priority as u8 | bg_pixel << 6 | sprite_pixel.color_index << 4;

        // TODO: Is it appropriate to evaluate sprite zero hit here considering the cycles
        // draw_pixel() is called on?
        if self.sprite_zero_hit(x, bg_pixel, &sprite_pixel) {
            self.status.set_sprite_zero_hit()
        }

        let i = ((scanline as usize) * SCREEN_WIDTH + ((x - 2) as usize)) * 2;
        self.screen[i] = color_byte;
        self.screen[i + 1] = property_byte;
    }

    // TODO: tests
    fn sprite_zero_hit(&self, x: u16, bg_pixel: u8, sprite_pixel: &SpritePixel) -> bool {
        !self.status.sprite_zero_hit() && (self.mask.show_background() && self.mask.show_sprites())
            && ((self.mask.background_render_leftmost_8_px()
                && self.mask.sprites_render_leftmost_8_px()) || x > 7) && x != 255
            && (bg_pixel > 0 && sprite_pixel.value > 0) && sprite_pixel.is_sprite_zero
    }
}

impl<V: IVram, S: ISpriteRenderer> IPpu for Ppu<V, S> {
    /// Accepts a PPU memory mapped address and writes it to the appropriate register
    fn write(&mut self, addr: u16, val: u8) {
        debug_assert!(
            addr >= 0x2000 && addr < 0x4000,
            "Invalid memory mapped ppu address"
        );

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
        debug_assert!(
            addr >= 0x2000 && addr < 0x4000,
            "Invalid memory mapped ppu address"
        );

        match addr & 7 {
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
            0x3 | 0x5 | 0x6 => 0, // Write-only
            0x7 => {
                let inc_amount = self.control.vram_addr_increment();
                self.vram.read_ppu_data(inc_amount)
            }
            _ => unreachable!(),
        }
    }

    #[ppu_loop]
    fn step(&mut self) -> Interrupt {
        Interrupt::None
    }

    fn screen(&self) -> &[u8; SCREEN_WIDTH * SCREEN_HEIGHT * 2] {
        &self.screen
    }
}
