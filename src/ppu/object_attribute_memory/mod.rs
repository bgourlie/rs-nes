use std::cell::Cell;
use std::num::Wrapping;

#[cfg(test)]
mod spec_tests;

#[derive(Debug, PartialEq)]
pub enum PaletteIndex {
    Zero,
    One,
    Two,
    Three,
}

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
    // Maps to the PPU's oam_data register
    pub fn read_data(&self) -> u8 {
        self.memory[self.address.get() as usize]
    }

    pub fn read_data_increment_addr(&self) -> u8 {
        let ret = self.read_data();
        self.inc_address();
        ret
    }

    pub fn write_address(&mut self, addr: u8) {
        self.address.set(addr);
    }

    pub fn write_data(&mut self, val: u8) {
        self.memory[self.address.get() as usize] = val;
        self.inc_address();
    }

    pub fn sprite_attributes(&self, tile_index: u8) -> SpriteAttributes {
        debug_assert!(tile_index <= 64, "Tile index out of bounds: {}", tile_index);
        let mem = self.memory;
        let index = (tile_index * 4) as usize;
        let y = mem[index];
        let tile_index = mem[index + 1];
        let attributes = mem[index + 2];
        let x = mem[index + 3];

        let palette_index = match attributes & 0b00000011 {
            0 => PaletteIndex::Zero,
            1 => PaletteIndex::One,
            2 => PaletteIndex::Two,
            3 => PaletteIndex::Three,
            _ => unreachable!(),
        };

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
            palette_index: palette_index,
            priority: priority,
            horizontal_flip: horizontal_flip,
            vertical_flip: vertical_flip,
            tile_index: tile_index,
        }
    }

    fn inc_address(&self) {
        let new_addr = (Wrapping(self.address.get()) + Wrapping(1_u8)).0;
        self.address.set(new_addr)
    }
}

impl ObjectAttributeMemory for ObjectAttributeMemoryBase {
    fn read_data(&self) -> u8 {
        self.read_data()
    }

    fn read_data_increment_addr(&self) -> u8 {
        self.read_data_increment_addr()
    }

    fn write_address(&mut self, addr: u8) {
        self.write_address(addr)
    }

    fn write_data(&mut self, val: u8) {
        self.write_data(val)
    }

    fn sprite_attributes(&self, tile_index: u8) -> SpriteAttributes {
        self.sprite_attributes(tile_index)
    }
}

pub struct SpriteAttributes {
    x: u8,
    y: u8,
    tile_index: u8,
    palette_index: PaletteIndex,
    priority: Priority,
    horizontal_flip: bool,
    vertical_flip: bool,
}
