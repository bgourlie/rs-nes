#![allow(dead_code)]

// TODO: Explore SIMD
// TODO: Tests

mod sprite_evaluation;

#[cfg(test)]
mod spec_tests;

use errors::*;
use ppu::SpriteSize;
use ppu::control_register::ControlRegister;
use ppu::palette::{self, Color, PALETTE};
use ppu::sprite_renderer::sprite_evaluation::SpriteEvaluation;
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
    fn start_sprite_evaluation(&mut self, scanline: u16, control: ControlRegister);
    fn tick_sprite_evaluation(&mut self);
    fn fill_registers<V: Vram>(&mut self, vram: &V, control: ControlRegister) -> Result<()>;
}

pub struct SpriteRendererBase {
    primary_oam: [u8; 0x100],
    address: Cell<u8>, // Maps to the PPU's oam_addr register
    palettes: [Color; 16],
    pattern_low_shift_registers: [u8; 8],
    pattern_high_shift_registers: [u8; 8],
    attribute_latches: [u8; 8],
    x_counters: [u8; 8],
    sprite_evaluation: SpriteEvaluation,
}

impl Default for SpriteRendererBase {
    fn default() -> Self {
        SpriteRendererBase {
            primary_oam: [0; 0x100],
            address: Cell::new(0),
            palettes: palette::EMPTY,
            pattern_low_shift_registers: [0; 8],
            pattern_high_shift_registers: [0; 8],
            attribute_latches: [0; 8],
            x_counters: [0; 8],
            sprite_evaluation: SpriteEvaluation::default(),
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
                self.pattern_low_shift_registers[i] <<= 1;
                self.pattern_high_shift_registers[i] <<= 1;
            }
        }
    }

    fn tick_sprite_evaluation(&mut self) {
        self.sprite_evaluation.tick(&self.primary_oam)
    }

    fn fill_registers<V: Vram>(&mut self, vram: &V, control: ControlRegister) -> Result<()> {
        for sprites_fetched in 0..8 {
            let sprite_base = sprites_fetched * 4;
            let tile_index = self.sprite_evaluation
                .read_secondary_oam(sprite_base + 1);

            let tile_offset = match control.sprite_size() {
                SpriteSize::X8 => {
                    let sprite_table_select = ((*control as u16) << 9) & 0x1000;
                    sprite_table_select | tile_index as u16
                }
                SpriteSize::X16 => {
                    let actual_tile_index = tile_index & !1;
                    let sprite_table_select = (tile_index as u16 & 1) << 12;
                    sprite_table_select | actual_tile_index as u16
                }
            };

            let pattern_low = vram.read(tile_offset)?;
            let pattern_high = vram.read(tile_offset + 8)?;
            let attribute = self.sprite_evaluation
                .read_secondary_oam(sprite_base + 2);
            let x = self.sprite_evaluation
                .read_secondary_oam(sprite_base + 3);
            self.pattern_low_shift_registers[sprites_fetched as usize] = pattern_low;
            self.pattern_high_shift_registers[sprites_fetched as usize] = pattern_high;
            self.attribute_latches[sprites_fetched as usize] = attribute;
            self.x_counters[sprites_fetched as usize] = x;
        }
        Ok(())
    }

    fn start_sprite_evaluation(&mut self, scanline: u16, control: ControlRegister) {
        // Current scanline is passed in, we evaluate the sprites for the next scanline
        let scanline = if scanline == 261 { 0 } else { scanline + 1 };
        self.sprite_evaluation = SpriteEvaluation::new(scanline as u8, control.sprite_size());
    }
}
