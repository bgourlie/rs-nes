#![allow(dead_code)]

// TODO: Explore SIMD
// TODO: Tests

mod sprite_evaluation;

#[cfg(test)]
mod spec_tests;

use errors::*;
use ppu::palette::{self, Color, PALETTE};
use ppu::sprite_renderer::sprite_evaluation::SpriteEvaluation;
pub use ppu::sprite_renderer::sprite_evaluation::SpriteEvaluationAction;
use ppu::vram::Vram;
use std::cell::Cell;
use std::num::Wrapping;

pub trait SpriteRenderer: Default {
    fn read_data(&self) -> u8;
    fn read_data_increment_addr(&self) -> u8;
    fn write_address(&mut self, addr: u8);
    fn write_data(&mut self, val: u8);
    fn update_palettes<V: Vram>(&mut self, vram: &V) -> Result<()>;
    fn pixel_color(&self, pixel: u8) -> Color;
    fn dec_x_counters(&mut self);
    fn start_secondary_oam_init(&mut self);
    fn start_sprite_evaluation(&mut self, scanline: u16);
    fn tick_secondary_oam_init(&mut self);
    fn tick_sprite_evaluation(&mut self) -> SpriteEvaluationAction;
    fn fetch_pattern_low_byte<V: Vram>(&mut self, vram: &V, control_reg: u8) -> Result<()>;
    fn fetch_pattern_high_byte<V: Vram>(&mut self, vram: &V, control_reg: u8) -> Result<()>;
}

pub struct SpriteRendererBase {
    primary_oam: [u8; 0x100],
    secondary_oam: [u8; 0x20],
    address: Cell<u8>, // Maps to the PPU's oam_addr register
    palettes: [Color; 16],
    pattern_shift_registers: [u8; 16], // odd index pattern low, even index pattern high
    attribute_latches: [u8; 8],
    x_counters: [u8; 8],
    secondary_oam_init_cycle: u8,
    sprite_evaluation: SpriteEvaluation,
    sprites_fetched: u8,
}

impl Default for SpriteRendererBase {
    fn default() -> Self {
        SpriteRendererBase {
            primary_oam: [0; 0x100],
            secondary_oam: [0; 0x20],
            address: Cell::new(0),
            palettes: palette::EMPTY,
            pattern_shift_registers: [0; 16],
            attribute_latches: [0; 8],
            x_counters: [0; 8],
            secondary_oam_init_cycle: 0,
            sprite_evaluation: SpriteEvaluation::default(),
            sprites_fetched: 0,
        }
    }
}

impl SpriteRendererBase {
    fn inc_address(&self) {
        let new_addr = (Wrapping(self.address.get()) + Wrapping(1_u8)).0;
        self.address.set(new_addr)
    }
}

impl SpriteRenderer for SpriteRendererBase {
    // Maps to the PPU's oam_data register
    fn read_data(&self) -> u8 {
        self.primary_oam[self.address.get() as usize]
    }

    fn read_data_increment_addr(&self) -> u8 {
        let ret = self.read_data();
        self.inc_address();
        ret
    }

    fn write_address(&mut self, val: u8) {
        self.address.set(val);
    }

    fn write_data(&mut self, val: u8) {
        self.primary_oam[self.address.get() as usize] = val;
        self.inc_address();
    }

    fn update_palettes<V: Vram>(&mut self, vram: &V) -> Result<()> {
        let bg = vram.read(0x3f00)? as usize;
        self.palettes = [PALETTE[bg],
                         PALETTE[vram.read(0x3f11)? as usize],
                         PALETTE[vram.read(0x3f12)? as usize],
                         PALETTE[vram.read(0x3f13)? as usize],
                         PALETTE[bg],
                         PALETTE[vram.read(0x3f15)? as usize],
                         PALETTE[vram.read(0x3f16)? as usize],
                         PALETTE[vram.read(0x3f17)? as usize],
                         PALETTE[bg],
                         PALETTE[vram.read(0x3f19)? as usize],
                         PALETTE[vram.read(0x3f1a)? as usize],
                         PALETTE[vram.read(0x3f1b)? as usize],
                         PALETTE[bg],
                         PALETTE[vram.read(0x3f1d)? as usize],
                         PALETTE[vram.read(0x3f1e)? as usize],
                         PALETTE[vram.read(0x3f1f)? as usize]];
        Ok(())
    }

    fn pixel_color(&self, pixel: u8) -> Color {
        self.palettes[pixel as usize]
    }

    fn dec_x_counters(&mut self) {
        for i in 0..8 {
            if self.x_counters[i] > 0 {
                self.x_counters[i] -= 1;
            } else {
                self.pattern_shift_registers[i * 2] <<= 1;
                self.pattern_shift_registers[i * 2 + 1] <<= 1;
            }
        }
    }

    fn tick_secondary_oam_init(&mut self) {
        debug_assert!(self.secondary_oam_init_cycle < 64);
        if self.secondary_oam_init_cycle % 2 == 1 {
            self.secondary_oam[(self.secondary_oam_init_cycle / 2) as usize] = 0xff
        }
        self.secondary_oam_init_cycle += 1;
    }

    fn tick_sprite_evaluation(&mut self) -> SpriteEvaluationAction {
        self.sprite_evaluation.tick(&self.primary_oam)
    }
    fn fetch_pattern_low_byte<V: Vram>(&mut self, vram: &V, control_reg: u8) -> Result<()> {
        let tile_index = self.secondary_oam[self.sprites_fetched as usize * 4 + 1] as u16;
        let table_select = ((control_reg as u16) << 9) & 0x1000;
        let tile_offset = table_select | tile_index;
        let pattern_low = vram.read(tile_offset)?;
        self.pattern_shift_registers[self.sprites_fetched as usize * 2] |= pattern_low;
        Ok(())
    }

    fn fetch_pattern_high_byte<V: Vram>(&mut self, vram: &V, control_reg: u8) -> Result<()> {
        let tile_index = self.secondary_oam[self.sprites_fetched as usize * 4 + 1] as u16;
        let table_select = ((control_reg as u16) << 9) & 0x1000;
        let tile_offset = table_select | tile_index;
        let pattern_high = vram.read(tile_offset + 8)?;
        self.pattern_shift_registers[self.sprites_fetched as usize * 2 + 1] |= pattern_high;
        self.sprites_fetched += 1;
        Ok(())
    }

    fn start_secondary_oam_init(&mut self) {
        self.secondary_oam_init_cycle = 0;
        self.sprites_fetched = 0;
    }

    fn start_sprite_evaluation(&mut self, scanline: u16) {
        // Current scanline is passed in, we evaluate the sprites for the next scanline
        let scanline = if scanline == 261 { 0 } else { scanline + 1 };
        self.sprite_evaluation = SpriteEvaluation::new(scanline as u8);
    }
}
