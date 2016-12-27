#[cfg(test)]
mod spec_tests;

#[derive(Debug, PartialEq, Clone)]
pub enum PaletteIndex {
    Zero,
    One,
    Two,
    Three,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Priority {
    InFrontOfBackground,
    BehindBackground,
}

pub struct ObjectAttributeMemory {
    memory: [u8; 0x100],
}

impl Clone for ObjectAttributeMemory {
    fn clone(&self) -> Self {
        let cloned_mem = self.memory;
        ObjectAttributeMemory { memory: cloned_mem }
    }
}

impl ObjectAttributeMemory {
    pub fn new() -> Self {
        ObjectAttributeMemory { memory: [0; 0x100] }
    }

    pub fn sprite_attributes(&self, tile_index: u8) -> SpriteAttributes {
        debug_assert!(tile_index <= 64, "Tile index out of bounds: {}", tile_index);
        let index = (tile_index * 4) as usize;
        let y = self.memory[index];
        let tile_index = self.memory[index + 1];
        let attributes = self.memory[index + 2];
        let x = self.memory[index + 3];

        let palette_index = match attributes & 0b00000011 {
            0 => PaletteIndex::Zero,
            1 => PaletteIndex::One,
            2 => PaletteIndex::Two,
            3 => PaletteIndex::Three,
            _ => panic!("impossible"),
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
