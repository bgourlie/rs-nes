use super::*;

#[test]
fn test_sprite_attributes_x_pos() {
    let oam = ObjectAttributeMemory { memory: mem_fixture(&[0, 0, 0, 2, 0, 0, 0, 1, 0, 0, 0, 0]) };
    let attrs = oam.sprite_attributes(0);
    assert_eq!(2, attrs.x);

    let attrs = oam.sprite_attributes(1);
    assert_eq!(1, attrs.x);

    let attrs = oam.sprite_attributes(2);
    assert_eq!(0, attrs.x);
}

#[test]
fn test_sprite_attributes_y_pos() {
    let oam = ObjectAttributeMemory { memory: mem_fixture(&[2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0]) };
    let attrs = oam.sprite_attributes(0);
    assert_eq!(2, attrs.y);

    let attrs = oam.sprite_attributes(1);
    assert_eq!(1, attrs.y);

    let attrs = oam.sprite_attributes(2);
    assert_eq!(0, attrs.y);
}

#[test]
fn test_sprite_attributes_tile_index() {
    let oam = ObjectAttributeMemory { memory: mem_fixture(&[0, 2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0]) };
    let attrs = oam.sprite_attributes(0);
    assert_eq!(2, attrs.tile_index);

    let attrs = oam.sprite_attributes(1);
    assert_eq!(1, attrs.tile_index);

    let attrs = oam.sprite_attributes(2);
    assert_eq!(0, attrs.tile_index);
}

#[test]
fn test_sprite_attributes_palette_index() {
    let oam = ObjectAttributeMemory {
        memory: mem_fixture(&[0, 0, 0b00000011, 0, 0, 0, 0b00000010, 0, 0, 0, 0b00000001, 0, 0,
                              0, 0b00000000, 0]),
    };

    let attrs = oam.sprite_attributes(0);
    assert_eq!(PaletteIndex::Three, attrs.palette_index);

    let attrs = oam.sprite_attributes(1);
    assert_eq!(PaletteIndex::Two, attrs.palette_index);

    let attrs = oam.sprite_attributes(2);
    assert_eq!(PaletteIndex::One, attrs.palette_index);

    let attrs = oam.sprite_attributes(3);
    assert_eq!(PaletteIndex::Zero, attrs.palette_index);
}

#[test]
fn test_sprite_attributes_priority() {
    let oam =
        ObjectAttributeMemory { memory: mem_fixture(&[0, 0, 0b00100000, 0, 0, 0, 0b00000000, 0]) };

    let attrs = oam.sprite_attributes(0);
    assert_eq!(Priority::BehindBackground, attrs.priority);

    let attrs = oam.sprite_attributes(1);
    assert_eq!(Priority::InFrontOfBackground, attrs.priority);
}

#[test]
fn test_sprite_attributes_horizontal_flip() {
    let oam =
        ObjectAttributeMemory { memory: mem_fixture(&[0, 0, 0b01000000, 0, 0, 0, 0b00000000, 0]) };

    let attrs = oam.sprite_attributes(0);
    assert_eq!(true, attrs.horizontal_flip);

    let attrs = oam.sprite_attributes(1);
    assert_eq!(false, attrs.horizontal_flip);
}

#[test]
fn test_sprite_attributes_vertical_flip() {
    let oam =
        ObjectAttributeMemory { memory: mem_fixture(&[0, 0, 0b10000000, 0, 0, 0, 0b00000000, 0]) };

    let attrs = oam.sprite_attributes(0);
    assert_eq!(true, attrs.vertical_flip);

    let attrs = oam.sprite_attributes(1);
    assert_eq!(false, attrs.vertical_flip);
}

fn mem_fixture(initial_values: &[u8]) -> [u8; 0x100] {
    let mut ret = [0_u8; 0x100];
    for (i, byte) in initial_values.iter().enumerate() {
        ret[i] = *byte;
    }
    ret
}
