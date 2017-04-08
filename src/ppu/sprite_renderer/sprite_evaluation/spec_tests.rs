use super::*;

#[test]
fn state_transitions() {
    let oam = [10, 01, 02, 03, 09, 03, 04, 05, 08, 06, 07, 08, 07, 09, 10, 11, 06, 12, 13, 14, 05,
               15, 16, 17, 04, 18, 19, 20, 03, 21, 22, 23, 02, 24, 25, 26];

    let mut eval = SpriteEvaluation::new(10);
    assert_eq!(State::ReadOamY, eval.state);

    // First Sprite - On Scanline

    let result = eval.tick(&oam);
    assert_eq!(State::WriteSecondaryOamY(10), eval.state);
    assert_eq!(SpriteEvaluationAction::None, result);

    let result = eval.tick(&oam);
    assert_eq!(State::ReadOamTile, eval.state);
    assert_eq!(SpriteEvaluationAction::None, result);

    let result = eval.tick(&oam);
    assert_eq!(State::WriteSecondaryOamTile(1), eval.state);
    assert_eq!(SpriteEvaluationAction::None, result);

    let result = eval.tick(&oam);
    assert_eq!(State::ReadOamAttributes, eval.state);
    assert_eq!(SpriteEvaluationAction::None, result);

    let result = eval.tick(&oam);
    assert_eq!(State::WriteSecondaryOamAttributes(2), eval.state);
    assert_eq!(SpriteEvaluationAction::None, result);

    let result = eval.tick(&oam);
    assert_eq!(State::ReadOamX, eval.state);
    assert_eq!(SpriteEvaluationAction::None, result);

    let result = eval.tick(&oam);
    assert_eq!(State::WriteSecondaryOamX(3), eval.state);
    assert_eq!(SpriteEvaluationAction::None, result);

    assert_eq!(1, eval.sprites_found());

    // Second Sprite - On Scanline

    let result = eval.tick(&oam);
    assert_eq!(State::ReadOamY, eval.state);
    assert_eq!(SpriteEvaluationAction::None, result);

    let result = eval.tick(&oam);
    assert_eq!(State::WriteSecondaryOamY(9), eval.state);
    assert_eq!(SpriteEvaluationAction::None, result);

    let result = eval.tick(&oam);
    assert_eq!(State::ReadOamTile, eval.state);
    assert_eq!(SpriteEvaluationAction::None, result);

    let result = eval.tick(&oam);
    assert_eq!(State::WriteSecondaryOamTile(4), eval.state);
    assert_eq!(SpriteEvaluationAction::None, result);

    let result = eval.tick(&oam);
    assert_eq!(State::ReadOamAttributes, eval.state);
    assert_eq!(SpriteEvaluationAction::None, result);

    let result = eval.tick(&oam);
    assert_eq!(State::WriteSecondaryOamAttributes(5), eval.state);
    assert_eq!(SpriteEvaluationAction::None, result);

    let result = eval.tick(&oam);
    assert_eq!(State::ReadOamX, eval.state);
    assert_eq!(SpriteEvaluationAction::None, result);

    let result = eval.tick(&oam);
    assert_eq!(State::WriteSecondaryOamX(6), eval.state);
    assert_eq!(SpriteEvaluationAction::None, result);
}

#[test]
fn is_sprite_on_scanline() {
    let eval = SpriteEvaluation::new(42);
    assert_eq!(true, eval.is_sprite_on_scanline(41));
    assert_eq!(true, eval.is_sprite_on_scanline(42));
    assert_eq!(false, eval.is_sprite_on_scanline(43));
    assert_eq!(false, eval.is_sprite_on_scanline(44));
    assert_eq!(false, eval.is_sprite_on_scanline(45));
    assert_eq!(false, eval.is_sprite_on_scanline(46));
    assert_eq!(false, eval.is_sprite_on_scanline(47));
    assert_eq!(false, eval.is_sprite_on_scanline(48));
    assert_eq!(false, eval.is_sprite_on_scanline(49));
    assert_eq!(false, eval.is_sprite_on_scanline(50));


}
