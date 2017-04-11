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

static REVERSE_LOOKUP: [u8; 256] =
    [0x00, 0x80, 0x40, 0xc0, 0x20, 0xa0, 0x60, 0xe0, 0x10, 0x90, 0x50, 0xd0, 0x30, 0xb0, 0x70,
     0xf0, 0x08, 0x88, 0x48, 0xc8, 0x28, 0xa8, 0x68, 0xe8, 0x18, 0x98, 0x58, 0xd8, 0x38, 0xb8,
     0x78, 0xf8, 0x04, 0x84, 0x44, 0xc4, 0x24, 0xa4, 0x64, 0xe4, 0x14, 0x94, 0x54, 0xd4, 0x34,
     0xb4, 0x74, 0xf4, 0x0c, 0x8c, 0x4c, 0xcc, 0x2c, 0xac, 0x6c, 0xec, 0x1c, 0x9c, 0x5c, 0xdc,
     0x3c, 0xbc, 0x7c, 0xfc, 0x02, 0x82, 0x42, 0xc2, 0x22, 0xa2, 0x62, 0xe2, 0x12, 0x92, 0x52,
     0xd2, 0x32, 0xb2, 0x72, 0xf2, 0x0a, 0x8a, 0x4a, 0xca, 0x2a, 0xaa, 0x6a, 0xea, 0x1a, 0x9a,
     0x5a, 0xda, 0x3a, 0xba, 0x7a, 0xfa, 0x06, 0x86, 0x46, 0xc6, 0x26, 0xa6, 0x66, 0xe6, 0x16,
     0x96, 0x56, 0xd6, 0x36, 0xb6, 0x76, 0xf6, 0x0e, 0x8e, 0x4e, 0xce, 0x2e, 0xae, 0x6e, 0xee,
     0x1e, 0x9e, 0x5e, 0xde, 0x3e, 0xbe, 0x7e, 0xfe, 0x01, 0x81, 0x41, 0xc1, 0x21, 0xa1, 0x61,
     0xe1, 0x11, 0x91, 0x51, 0xd1, 0x31, 0xb1, 0x71, 0xf1, 0x09, 0x89, 0x49, 0xc9, 0x29, 0xa9,
     0x69, 0xe9, 0x19, 0x99, 0x59, 0xd9, 0x39, 0xb9, 0x79, 0xf9, 0x05, 0x85, 0x45, 0xc5, 0x25,
     0xa5, 0x65, 0xe5, 0x15, 0x95, 0x55, 0xd5, 0x35, 0xb5, 0x75, 0xf5, 0x0d, 0x8d, 0x4d, 0xcd,
     0x2d, 0xad, 0x6d, 0xed, 0x1d, 0x9d, 0x5d, 0xdd, 0x3d, 0xbd, 0x7d, 0xfd, 0x03, 0x83, 0x43,
     0xc3, 0x23, 0xa3, 0x63, 0xe3, 0x13, 0x93, 0x53, 0xd3, 0x33, 0xb3, 0x73, 0xf3, 0x0b, 0x8b,
     0x4b, 0xcb, 0x2b, 0xab, 0x6b, 0xeb, 0x1b, 0x9b, 0x5b, 0xdb, 0x3b, 0xbb, 0x7b, 0xfb, 0x07,
     0x87, 0x47, 0xc7, 0x27, 0xa7, 0x67, 0xe7, 0x17, 0x97, 0x57, 0xd7, 0x37, 0xb7, 0x77, 0xf7,
     0x0f, 0x8f, 0x4f, 0xcf, 0x2f, 0xaf, 0x6f, 0xef, 0x1f, 0x9f, 0x5f, 0xdf, 0x3f, 0xbf, 0x7f,
     0xff];

#[derive(Eq, PartialEq)]
pub enum SpritePriority {
    OnTopOfBackground,
    BehindBackground,
}

#[derive(Copy, Clone)]
struct SpriteAttributes(u8);

pub struct SpritePixel {
    pub value: u8,
    pub priority: SpritePriority,
    pub color: Color,
    pub is_sprite_zero: bool,
}

impl SpriteAttributes {
    fn palette(&self) -> u8 {
        let SpriteAttributes(val) = *self;
        val & 0b11
    }

    fn flip_horizontally(&self) -> bool {
        let SpriteAttributes(val) = *self;
        val & 0b0100_0000 > 0
    }

    fn flip_vertically(&self) -> bool {
        let SpriteAttributes(val) = *self;
        val & 0b1000_0000 > 0
    }

    fn priority(&self) -> SpritePriority {
        let SpriteAttributes(val) = *self;
        if val & 0b0010_0000 == 0 {
            SpritePriority::OnTopOfBackground
        } else {
            SpritePriority::BehindBackground
        }
    }
}

impl Default for SpriteAttributes {
    fn default() -> Self {
        SpriteAttributes(0xff)
    }
}

pub trait SpriteRenderer: Default {
    fn read_data(&self) -> u8;
    fn read_data_increment_addr(&self) -> u8;
    fn write_address(&mut self, addr: u8);
    fn write_data(&mut self, val: u8);
    fn update_palettes<V: Vram>(&mut self, vram: &V) -> Result<()>;
    fn dec_x_counters(&mut self);
    fn start_sprite_evaluation(&mut self, scanline: u16, control: ControlRegister);
    fn tick_sprite_evaluation(&mut self);
    fn fill_registers<V: Vram>(&mut self, vram: &V, control: ControlRegister) -> Result<()>;
    fn current_pixel(&self) -> SpritePixel;
}

pub struct SpriteRendererBase {
    primary_oam: [u8; 0x100],
    address: Cell<u8>, // Maps to the PPU's oam_addr register
    palettes: [Color; 16],
    pattern_low_shift_registers: [u8; 8],
    pattern_high_shift_registers: [u8; 8],
    attribute_latches: [SpriteAttributes; 8],
    x_counters: [u8; 8],
    sprite_evaluation: SpriteEvaluation,
    sprite_zero_map: u8,
}

impl Default for SpriteRendererBase {
    fn default() -> Self {
        SpriteRendererBase {
            primary_oam: [0; 0x100],
            address: Cell::new(0),
            palettes: palette::EMPTY,
            pattern_low_shift_registers: [0; 8],
            pattern_high_shift_registers: [0; 8],
            attribute_latches: [SpriteAttributes::default(); 8],
            x_counters: [0; 8],
            sprite_evaluation: SpriteEvaluation::default(),
            sprite_zero_map: 0,
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

            let tile_y = self.sprite_evaluation.read_secondary_oam(sprite_base);

            let tile_index = self.sprite_evaluation
                .read_secondary_oam(sprite_base + 1);

            let attribute_byte = self.sprite_evaluation
                .read_secondary_oam(sprite_base + 2);

            let attribute = SpriteAttributes(attribute_byte);

            let fine_y = if attribute.flip_vertically() {
                7 - (self.sprite_evaluation.scanline() - tile_y)
            } else {
                self.sprite_evaluation.scanline() - tile_y
            };

            let tile_offset = match control.sprite_size() {
                SpriteSize::X8 => control.sprite_pattern_table_base() | ((tile_index as u16) << 4),
                SpriteSize::X16 => {
                    //                    let actual_tile_index = tile_index & !1;
                    //                    let sprite_table_select = (tile_index as u16 & 1) << 12;
                    //                    sprite_table_select | actual_tile_index as u16
                    unimplemented!()
                }
            } + fine_y as u16;


            let pattern_low = vram.read(tile_offset)?;
            let pattern_high = vram.read(tile_offset + 8)?;

            let (pattern_low, pattern_high) = if attribute.flip_horizontally() {
                (REVERSE_LOOKUP[pattern_low as usize], REVERSE_LOOKUP[pattern_high as usize])
            } else {
                (pattern_low, pattern_high)
            };

            let x = self.sprite_evaluation
                .read_secondary_oam(sprite_base + 3);

            self.pattern_low_shift_registers[sprites_fetched as usize] = pattern_low;
            self.pattern_high_shift_registers[sprites_fetched as usize] = pattern_high;
            self.attribute_latches[sprites_fetched as usize] = attribute;
            self.x_counters[sprites_fetched as usize] = x;
        }
        self.sprite_zero_map = self.sprite_evaluation.sprite_zero_map();
        Ok(())
    }

    fn start_sprite_evaluation(&mut self, scanline: u16, control: ControlRegister) {
        // Current scanline is passed in, we evaluate the sprites for the next scanline
        self.sprite_evaluation = SpriteEvaluation::new(scanline as u8, control.sprite_size());
    }

    fn current_pixel(&self) -> SpritePixel {
        let mut pixel = 0;
        let mut attributes = SpriteAttributes::default();
        let mut is_sprite_zero = false;

        for i in 0..8 {
            if self.x_counters[i] == 0 && pixel == 0 {
                let high_bit = self.pattern_high_shift_registers[i] >> 7;
                let low_bit = self.pattern_low_shift_registers[i] >> 7;
                pixel = (high_bit << 1) | low_bit;
                attributes = self.attribute_latches[i];
                is_sprite_zero = self.sprite_zero_map & (1 << i) > 0;
            }
        }
        let palette = attributes.palette() << 2;
        let palette_index = (palette | pixel) as usize;
        SpritePixel {
            value: pixel,
            priority: attributes.priority(),
            color: self.palettes[palette_index],
            is_sprite_zero: is_sprite_zero,
        }
    }
}
