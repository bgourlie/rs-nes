use std::cell::Cell;
use std::num::Wrapping;

#[cfg(test)]
mod spec_tests;

#[derive(Debug, PartialEq)]
pub enum Priority {
    InFrontOfBackground,
    BehindBackground,
}

pub trait ObjectAttributeMemory: Default {
    fn read_data(&self) -> u8;
    fn read_data_increment_addr(&self) -> u8;
    fn write_address(&mut self, addr: u8);
    fn write_data(&mut self, val: u8);
    fn sprite_attributes(&self, tile_index: u8) -> SpriteAttributes;
}

pub struct ObjectAttributeMemoryBase {
    memory: [u8; 0x100],
    address: Cell<u8>, // Maps to the PPU's oam_addr register
}

impl Default for ObjectAttributeMemoryBase {
    fn default() -> Self {
        ObjectAttributeMemoryBase {
            memory: [0; 0x100],
            address: Cell::new(0),
        }
    }
}

impl ObjectAttributeMemoryBase {
    fn inc_address(&self) {
        let new_addr = (Wrapping(self.address.get()) + Wrapping(1_u8)).0;
        self.address.set(new_addr)
    }
}

impl ObjectAttributeMemory for ObjectAttributeMemoryBase {
    // Maps to the PPU's oam_data register
    fn read_data(&self) -> u8 {
        self.memory[self.address.get() as usize]
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
        self.memory[self.address.get() as usize] = val;
        self.inc_address();
    }

    fn sprite_attributes(&self, tile_index: u8) -> SpriteAttributes {
        debug_assert!(tile_index <= 64, "Tile index out of bounds: {}", tile_index);
        let mem = self.memory;
        let index = tile_index as usize * 4;
        let y = mem[index];
        let tile_index = mem[index + 1];
        let attributes = mem[index + 2];
        let x = mem[index + 3];

        let palette = attributes & 0b00000011;

        let priority = if attributes & 0b00100000 == 0 {
            Priority::InFrontOfBackground
        } else {
            Priority::BehindBackground
        };

        let horizontal_flip = attributes & 0b01000000 > 0;
        let vertical_flip = attributes & 0b10000000 > 0;

        SpriteAttributes {
            x: x,
            y: y,
            palette: palette,
            priority: priority,
            horizontal_flip: horizontal_flip,
            vertical_flip: vertical_flip,
            tile_index: tile_index,
        }
    }
}

pub struct SpriteAttributes {
    pub x: u8,
    pub y: u8,
    pub tile_index: u8,
    pub palette: u8,
    pub priority: Priority,
    pub horizontal_flip: bool,
    pub vertical_flip: bool,
}
