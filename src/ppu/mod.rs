#[cfg(test)]
mod spec_tests;

mod background_renderer;
mod control_register;
mod mask_register;
mod palette;
mod sprite_renderer;
mod status_register;
mod vram;
mod write_latch;

use self::write_latch::WriteLatch;
use cpu::Interrupt;
use ppu::background_renderer::BackgroundRenderer;
use ppu::control_register::ControlRegister;
use ppu::mask_register::MaskRegister;
use ppu::sprite_renderer::{SpritePixel, SpritePriority, SpriteRenderer, SpriteRendererBase};
use ppu::status_register::StatusRegister;
use ppu::vram::{Vram, VramBase};
use rom::NesRom;
use rs_nes_macros::ppu_loop;
use screen::{NesScreen, Screen};
use std::io::Write;
use std::rc::Rc;

const SCANLINES: usize = 262;
const CYCLES_PER_SCANLINE: usize = 341;
const CYCLES_PER_FRAME: usize = SCANLINES * CYCLES_PER_SCANLINE;

pub type PpuImpl = PpuBase<VramBase, SpriteRendererBase>;

pub trait Ppu {
    type Scr: Screen;

    fn new(rom: Rc<Box<NesRom>>) -> Self;
    fn write(&mut self, addr: u16, val: u8);
    fn read(&self, addr: u16) -> u8;
    fn step(&mut self) -> Interrupt;
    fn screen(&self) -> &NesScreen;
    fn dump_registers<T: Write>(&self, writer: &mut T);
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

pub struct PpuBase<V: Vram, S: SpriteRenderer> {
    cycles: usize,
    control: ControlRegister,
    mask: MaskRegister,
    status: StatusRegister,
    vram: V,
    sprite_renderer: S,
    screen: NesScreen,
    write_latch: WriteLatch,
    background_renderer: BackgroundRenderer,
    odd_frame: bool,
}

impl<V: Vram, S: SpriteRenderer> PpuBase<V, S> {
    fn draw_pixel(&mut self, x: u16, scanline: u16) {
        let fine_x = self.vram.fine_x();
        let (bg_pixel, bg_color) = self.background_renderer.current_pixel(fine_x);
        let sprite_pixel = self.sprite_renderer.current_pixel();

        let color = match (bg_pixel, sprite_pixel.value) {
            (0, 0) | (_, 0) => bg_color,
            (0, _) => sprite_pixel.color,
            _ => if sprite_pixel.priority == SpritePriority::OnTopOfBackground {
                sprite_pixel.color
            } else {
                bg_color
            },
        };

        // TODO: Is it appropriate to evaluate sprite zero hit here considering the cycles
        // draw_pixel() is called on?
        if self.sprite_zero_hit(x, bg_pixel, &sprite_pixel) {
            self.status.set_sprite_zero_hit()
        }

        self.screen.put_pixel((x - 2) as _, scanline as _, color);
    }

    // TODO: tests
    fn sprite_zero_hit(&self, x: u16, bg_pixel: u8, sprite_pixel: &SpritePixel) -> bool {
        !self.status.sprite_zero_hit() && (self.mask.show_background() && self.mask.show_sprites())
            && ((self.mask.background_render_leftmost_8_px()
                && self.mask.sprites_render_leftmost_8_px()) || x > 7) && x != 255
            && (bg_pixel > 0 && sprite_pixel.value > 0) && sprite_pixel.is_sprite_zero
    }
}

impl<V: Vram, S: SpriteRenderer> Ppu for PpuBase<V, S> {
    type Scr = NesScreen;

    fn new(rom: Rc<Box<NesRom>>) -> Self {
        PpuBase {
            cycles: 0,
            control: ControlRegister::default(),
            mask: MaskRegister::default(),
            status: StatusRegister::default(),
            vram: V::new(rom),
            sprite_renderer: S::default(),
            screen: Self::Scr::default(),
            write_latch: WriteLatch::default(),
            background_renderer: BackgroundRenderer::default(),
            odd_frame: false,
        }
    }

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
            0x7 => {
                let inc_amount = self.control.vram_addr_increment();
                self.vram.read_ppu_data(inc_amount)
            }
            0x3 | 0x5 | 0x6 => 0, // Write-only
            _ => unreachable!(),
        }
    }

    #[ppu_loop]
    fn step(&mut self) -> Interrupt {
        Interrupt::None
    }

    fn screen(&self) -> &NesScreen {
        &self.screen
    }

    /// Dump register memory
    fn dump_registers<T: Write>(&self, writer: &mut T) {
        let regs = [
            *self.control,
            *self.mask,
            self.status.read(),
            0, // Write-only
            self.sprite_renderer.read_data(),
            0, // Write-only
            0, // Write-only
            0,
        ];

        writer.write_all(&regs).unwrap()
    }
}
