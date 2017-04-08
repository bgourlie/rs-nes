#![cfg_attr(rustfmt, rustfmt_skip)]

use super::*;
use ppu::SpriteSize;

#[test]
fn sprite_overflow_triggered() {
    // Put 8 sprites on a scanline, assert that overflow occurs

    let oam_fixture = oam_fixture(&[
        10, 01, 02, 03,
        09, 04, 05, 06,
        08, 07, 08, 09,
        07, 10, 11, 12,
        06, 13, 14, 15,
        05, 16, 17, 18,
        04, 19, 20, 21,
        03, 22, 23, 24,
        03, 25, 26, 27,
        10, 28, 29, 30,
    ]);

    let mut eval = SpriteEvaluation::new(10, SpriteSize::X8);

    // Evaluate 8 sprites all on same scanline
    for frame_cycle in 65..256 {
        eval.tick(&oam_fixture, frame_cycle);
        if frame_cycle >= 130 {
            assert_eq!(true, eval.overflow, "frame_cycle = {}", frame_cycle);
        } else {
            assert_eq!(false, eval.overflow, "frame_cycle = {}", frame_cycle);
        }
    }
}

#[test]
fn sprite_overflow_triggered_via_hardware_bug() {
    // Put 8 sprites on a scanline, assert that overflow occurs

    let oam_fixture = oam_fixture(&[
        10, 01, 02, 03,
        09, 04, 05, 06,
        08, 07, 08, 09,
        07, 10, 11, 12,
        06, 13, 14, 15,
        05, 16, 17, 18,
        04, 19, 20, 21,
        03, 22, 23, 24,
        02, 25, 26, 27,
        10, 03, 29, 30, // Due to hardware bug, sprite overflow evaluation will interpret the tile
                        // byte (second byte) of this tile as "Y"
    ]);

    let mut eval = SpriteEvaluation::new(10, SpriteSize::X8);

    // Evaluate 8 sprites all on same scanline
    for frame_cycle in 65..256 {
        eval.tick(&oam_fixture, frame_cycle);
        assert_eq!(false, eval.overflow, "frame_cycle = {}", frame_cycle);
    }
}

#[test]
fn is_sprite_on_scanline_8x8_sprite() {
    let eval = SpriteEvaluation::new(42, SpriteSize::X8);
    assert_eq!(false, eval.is_sprite_on_scanline(34));
    assert_eq!(true, eval.is_sprite_on_scanline(35));
    assert_eq!(true, eval.is_sprite_on_scanline(36));
    assert_eq!(true, eval.is_sprite_on_scanline(37));
    assert_eq!(true, eval.is_sprite_on_scanline(38));
    assert_eq!(true, eval.is_sprite_on_scanline(39));
    assert_eq!(true, eval.is_sprite_on_scanline(40));
    assert_eq!(true, eval.is_sprite_on_scanline(41));
    assert_eq!(true, eval.is_sprite_on_scanline(42));
    assert_eq!(false, eval.is_sprite_on_scanline(43));
}

#[test]
fn is_sprite_on_scanline_8x16_sprite() {
    let eval = SpriteEvaluation::new(42, SpriteSize::X16);
    assert_eq!(false, eval.is_sprite_on_scanline(26));
    assert_eq!(true, eval.is_sprite_on_scanline(27));
    assert_eq!(true, eval.is_sprite_on_scanline(28));
    assert_eq!(true, eval.is_sprite_on_scanline(29));
    assert_eq!(true, eval.is_sprite_on_scanline(30));
    assert_eq!(true, eval.is_sprite_on_scanline(31));
    assert_eq!(true, eval.is_sprite_on_scanline(32));
    assert_eq!(true, eval.is_sprite_on_scanline(33));
    assert_eq!(true, eval.is_sprite_on_scanline(34));
    assert_eq!(true, eval.is_sprite_on_scanline(35));
    assert_eq!(true, eval.is_sprite_on_scanline(36));
    assert_eq!(true, eval.is_sprite_on_scanline(37));
    assert_eq!(true, eval.is_sprite_on_scanline(38));
    assert_eq!(true, eval.is_sprite_on_scanline(39));
    assert_eq!(true, eval.is_sprite_on_scanline(40));
    assert_eq!(true, eval.is_sprite_on_scanline(41));
    assert_eq!(true, eval.is_sprite_on_scanline(42));
    assert_eq!(false, eval.is_sprite_on_scanline(43));
}

fn oam_fixture(oam: &[u8]) -> [u8; 0x100] {
    let mut oam_fixture = [0xff_u8; 0x100];
    for i in 0..oam.len() {
        oam_fixture[i] = oam[i]
    }
    oam_fixture
}