use super::*;
use ppu::palette::EMPTY;

#[test]
fn oam_write_and_read() {
    let mut oam = oam_fixture(&[0xa, 0xb, 0xc, 0xd]);
    oam.write_address(0x1);
    assert_eq!(0xb, oam.read_data());
    assert_eq!(0xb, oam.read_data());
    assert_eq!(0x1, oam.address.get())
}

#[test]
fn oam_write_and_read_increment() {
    let mut mem = [0_u8; 0x100];

    for i in 0..0x100 {
        mem[i] = i as u8;
    }

    let mut oam = oam_fixture(&mem);
    oam.write_address(0xfe);
    assert_eq!(0xfe, oam.read_data_increment_addr());
    assert_eq!(0xff, oam.address.get());
    assert_eq!(0xff, oam.read_data_increment_addr());
    assert_eq!(0x0, oam.address.get());
    assert_eq!(0x0, oam.read_data_increment_addr());
    assert_eq!(0x1, oam.address.get())
}

fn oam_fixture(initial_values: &[u8]) -> SpriteRendererBase {
    let mut mem = [0_u8; 0x100];
    for (i, byte) in initial_values.iter().enumerate() {
        mem[i] = *byte;
    }

    SpriteRendererBase {
        primary_oam: mem,
        address: Cell::new(0),
        palettes: EMPTY,
        pattern_low_shift_registers: [0_u8; 8],
        pattern_high_shift_registers: [0_u8; 8],
        attribute_latches: [0_u8; 8],
        x_counters: [0_u8; 8],
        sprite_evaluation: SpriteEvaluation::default(),
    }
}
