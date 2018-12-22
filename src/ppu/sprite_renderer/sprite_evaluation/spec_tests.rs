#![cfg_attr(rustfmt, rustfmt_skip)]

use super::*;
use crate::ppu::SpriteSize;

#[test]
fn sprite_evaluation() {
    // Every other sprite in OAM is on the scanline, starting with the first
    let oam_fixture = oam_fixture(&[
        10, 01, 02, 03,
        50, 04, 05, 06,
        08, 07, 08, 09,
        60, 10, 11, 12,
        06, 13, 14, 15,
        70, 16, 17, 18,
        04, 19, 20, 21,
        80, 22, 23, 24,
        03, 25, 26, 27,
        90, 01, 29, 30,
        10, 01, 30, 30,
        110, 01, 04, 30,
        10, 30, 30, 30,
        120, 01, 04, 30,
    ]);

    let expected_secondary_oam = [
        10, 01, 02, 03,
        08, 07, 08, 09,
        06, 13, 14, 15,
        04, 19, 20, 21,
        03, 25, 26, 27,
        10, 01, 30, 30,
        10, 30, 30, 30,
        255, 255, 255, 255
    ];

    let mut eval = SpriteEvaluation::new(10, SpriteSize::X8);

    // Assert that every other sprite evaluated is on the scanline. There are only 7 on the
    // scanline, so once we've evaluated those, assert correct n increment behavior
    let mut expected_n = 0;
    for i in 0..7 { // cycles 65 to 135
        let expected_sprites_found = i + 1;

        // 8 ticks to evaluate the first scanline
        for _ in 0..8 {
            eval.tick(&oam_fixture);
        }
        expected_n += 1;
        assert_eq!(expected_sprites_found, eval.sprites_found);
        assert_eq!(expected_n, eval.n);
        assert_eq!(0, eval.m);

        // Two ticks to determine next isn't on scanline
        eval.tick(&oam_fixture);
        eval.tick(&oam_fixture);
        expected_n += 1;
        assert_eq!(expected_sprites_found, eval.sprites_found);
        assert_eq!(expected_n, eval.n);
        assert_eq!(0, eval.m);
    }

    // Evaluate rest of sprites in OAM, which are not on scanline and assert n and m
    for _ in 0..50 {
        eval.tick(&oam_fixture);
        eval.tick(&oam_fixture);
        expected_n += 1;
        assert_eq!(7, eval.sprites_found);
        assert_eq!(expected_n, eval.n);
        assert_eq!(0, eval.m);
    }

    // Evaluated all sprites, assert correct sprites and no n or m increment
    for _ in 0..22 {
        eval.tick(&oam_fixture);
        assert_eq!(7, eval.sprites_found);
        assert_eq!(64, eval.n);
        assert_eq!(0, eval.m);
    }

    assert_eq!(expected_secondary_oam, eval.secondary_oam);
}

#[test]
fn sprite_overflow_triggered() {
    // Put 8 sprites on a scanline, assert sprite overflow (non-buggy behavior)

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

    let expected_secondary_oam = [
        10, 01, 02, 03,
        09, 04, 05, 06,
        08, 07, 08, 09,
        07, 10, 11, 12,
        06, 13, 14, 15,
        05, 16, 17, 18,
        04, 19, 20, 21,
        03, 22, 23, 24,
    ];

    let mut eval = SpriteEvaluation::new(10, SpriteSize::X8);

    // Evaluate 8 sprites all on same scanline
    for frame_cycle in 65..256 {
        eval.tick(&oam_fixture);
        if frame_cycle >= 130 {
            assert_eq!(true, eval.sprite_overflow, "frame_cycle = {}", frame_cycle);
        } else {
            assert_eq!(false, eval.sprite_overflow, "frame_cycle = {}", frame_cycle);
        }
    }
    assert_eq!(expected_secondary_oam, eval.secondary_oam)
}

#[test]
fn sprite_overflow_triggered_via_hardware_bug() {
    // Put 8 sprites on a scanline, assert buggy sprite overflow behavior

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
        10, 03, 29, 30, // Due to hardware bug, sprite overflow evaluation will interpret the second
                        // byte of this tile as "Y"
    ]);

    let expected_secondary_oam = [
        10, 01, 02, 03,
        09, 04, 05, 06,
        08, 07, 08, 09,
        07, 10, 11, 12,
        06, 13, 14, 15,
        05, 16, 17, 18,
        04, 19, 20, 21,
        03, 22, 23, 24,
    ];

    let mut eval = SpriteEvaluation::new(10, SpriteSize::X8);

    // Evaluate 8 sprites all on same scanline
    for frame_cycle in 65..256 {
        eval.tick(&oam_fixture);
        if frame_cycle >= 132 {
            assert_eq!(true, eval.sprite_overflow, "frame_cycle = {}", frame_cycle);
        } else {
            assert_eq!(false, eval.sprite_overflow, "frame_cycle = {}", frame_cycle);
        }
    }
    assert_eq!(expected_secondary_oam, eval.secondary_oam)
}

#[test]
fn sprite_overflow_triggered_via_hardware_bug_2() {
    // Put 8 sprites on a scanline, assert buggy sprite overflow behavior

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
        10, 01, 29, 30,
        10, 01, 04, 30, // Due to hardware bug, sprite overflow evaluation will interpret the third
                        // byte of this tile as "Y"
    ]);

    let expected_secondary_oam = [
        10, 01, 02, 03,
        09, 04, 05, 06,
        08, 07, 08, 09,
        07, 10, 11, 12,
        06, 13, 14, 15,
        05, 16, 17, 18,
        04, 19, 20, 21,
        03, 22, 23, 24,
    ];

    let mut eval = SpriteEvaluation::new(10, SpriteSize::X8);

    // Evaluate 8 sprites all on same scanline
    for frame_cycle in 65..256 {
        eval.tick(&oam_fixture);
        if frame_cycle >= 134 {
            assert_eq!(true, eval.sprite_overflow, "frame_cycle = {}", frame_cycle);
        } else {
            assert_eq!(false, eval.sprite_overflow, "frame_cycle = {}", frame_cycle);
        }
    }
    assert_eq!(expected_secondary_oam, eval.secondary_oam)
}

#[test]
fn sprite_overflow_triggered_via_hardware_bug_3() {
    // Put 8 sprites on a scanline, assert buggy sprite overflow behavior

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
        10, 01, 29, 30,
        10, 01, 30, 30,
        10, 01, 04, 03, // Due to hardware bug, sprite overflow evaluation will interpret the fourth
                        // byte of this tile as "Y"
    ]);

    let expected_secondary_oam = [
        10, 01, 02, 03,
        09, 04, 05, 06,
        08, 07, 08, 09,
        07, 10, 11, 12,
        06, 13, 14, 15,
        05, 16, 17, 18,
        04, 19, 20, 21,
        03, 22, 23, 24,
    ];

    let mut eval = SpriteEvaluation::new(10, SpriteSize::X8);

    // Evaluate 8 sprites all on same scanline
    for frame_cycle in 65..256 {
        eval.tick(&oam_fixture);
        if frame_cycle >= 136 {
            assert_eq!(true, eval.sprite_overflow, "frame_cycle = {}", frame_cycle);
        } else {
            assert_eq!(false, eval.sprite_overflow, "frame_cycle = {}", frame_cycle);
        }
    }
    assert_eq!(expected_secondary_oam, eval.secondary_oam)
}

#[test]
fn sprite_overflow_triggered_via_hardware_bug_4() {
    // Put 8 sprites on a scanline, assert buggy sprite overflow behavior

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
        10, 01, 29, 30,
        10, 01, 30, 30,
        10, 01, 04, 30,
        10, 30, 30, 30, // Every fourth sprite evaluated for overflow should correctly interpret the
                        // first byte as y
    ]);

    let expected_secondary_oam = [
        10, 01, 02, 03,
        09, 04, 05, 06,
        08, 07, 08, 09,
        07, 10, 11, 12,
        06, 13, 14, 15,
        05, 16, 17, 18,
        04, 19, 20, 21,
        03, 22, 23, 24,
    ];

    let mut eval = SpriteEvaluation::new(10, SpriteSize::X8);

    // Evaluate 8 sprites all on same scanline
    for frame_cycle in 65..256 {
        eval.tick(&oam_fixture);
        if frame_cycle >= 138 {
            assert_eq!(true, eval.sprite_overflow, "frame_cycle = {}", frame_cycle);
        } else {
            assert_eq!(false, eval.sprite_overflow, "frame_cycle = {}", frame_cycle);
        }
    }
    assert_eq!(expected_secondary_oam, eval.secondary_oam)
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