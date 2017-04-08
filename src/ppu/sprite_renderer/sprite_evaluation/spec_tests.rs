use super::*;
use ppu::SpriteSize;

#[test]
fn sprite_overflow() {
    // Put 8 sprites on a scanline, assert that overflow occurs

    #[cfg_attr(rustfmt, rustfmt_skip)]
    let oam_fixture = [
        10, 01, 02, 03,
        09, 04, 05, 06,
        08, 07, 08, 09,
        07, 10, 11, 12,
        06, 13, 14, 15,
        05, 16, 17, 18,
        04, 19, 20, 21,
        03, 22, 23, 24,
        02, 25, 26, 27,
        10, 28, 29, 30];

    let expected_states = vec![/* First Sprite */
                               State::FirstWrite(10),
                               State::SecondRead,
                               State::SecondWrite(1),
                               State::ThirdRead,
                               State::ThirdWrite(2),
                               State::FourthRead,
                               State::FourthWrite(3),
                               State::FirstRead,

                               /* Second Sprite */
                               State::FirstWrite(9),
                               State::SecondRead,
                               State::SecondWrite(4),
                               State::ThirdRead,
                               State::ThirdWrite(5),
                               State::FourthRead,
                               State::FourthWrite(6),
                               State::FirstRead,

                               /* Third Sprite */
                               State::FirstWrite(8),
                               State::SecondRead,
                               State::SecondWrite(7),
                               State::ThirdRead,
                               State::ThirdWrite(8),
                               State::FourthRead,
                               State::FourthWrite(9),
                               State::FirstRead,

                               /* Fourth Sprite */
                               State::FirstWrite(7),
                               State::SecondRead,
                               State::SecondWrite(10),
                               State::ThirdRead,
                               State::ThirdWrite(11),
                               State::FourthRead,
                               State::FourthWrite(12),
                               State::FirstRead,

                               /* Fifth Sprite */
                               State::FirstWrite(6),
                               State::SecondRead,
                               State::SecondWrite(13),
                               State::ThirdRead,
                               State::ThirdWrite(14),
                               State::FourthRead,
                               State::FourthWrite(15),
                               State::FirstRead,

                               /* Sixth Sprite */
                               State::FirstWrite(5),
                               State::SecondRead,
                               State::SecondWrite(16),
                               State::ThirdRead,
                               State::ThirdWrite(17),
                               State::FourthRead,
                               State::FourthWrite(18),
                               State::FirstRead,

                               /* Seventh Sprite */
                               State::FirstWrite(4),
                               State::SecondRead,
                               State::SecondWrite(19),
                               State::ThirdRead,
                               State::ThirdWrite(20),
                               State::FourthRead,
                               State::FourthWrite(21),
                               State::FirstRead,

                               /* Eighth Sprite */
                               State::FirstWrite(3),
                               State::SecondRead,
                               State::SecondWrite(22),
                               State::ThirdRead,
                               State::ThirdWrite(23),
                               State::FourthRead,
                               State::FourthWrite(24),
                               State::SpriteOverflowEvaluationRead];


    let mut eval = SpriteEvaluation::new(10, SpriteSize::X8);

    // Evaluate 8 sprites all on same scanline
    for cycle_num in 0..64 {
        let result = eval.tick(&oam_fixture);
        let ref expected_state = expected_states[cycle_num];
        assert_eq!(*expected_state, eval.state, "Cycle num is {}", cycle_num);
        assert_eq!(SpriteEvaluationAction::None, result);
        assert_eq!((cycle_num / 8) as u8, eval.sprites_found());
    }

    // Evaluate 9th sprite on same scanline
    let result = eval.tick(&oam_fixture);
    assert_eq!(State::SpriteOverflowEvaluationWrite(10), eval.state);
    assert_eq!(SpriteEvaluationAction::SetSpriteOverflowFlag, result);
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
