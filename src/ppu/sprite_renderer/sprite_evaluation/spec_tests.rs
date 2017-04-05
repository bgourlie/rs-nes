use super::*;

#[test]
fn is_sprite_on_scanline() {
    let eval = SpriteEvaluation::new(42);
    assert_eq!(false, eval.is_sprite_on_scanline(41));
    assert_eq!(true, eval.is_sprite_on_scanline(42));
    assert_eq!(true, eval.is_sprite_on_scanline(43));
    assert_eq!(true, eval.is_sprite_on_scanline(44));
    assert_eq!(true, eval.is_sprite_on_scanline(45));
    assert_eq!(true, eval.is_sprite_on_scanline(46));
    assert_eq!(true, eval.is_sprite_on_scanline(47));
    assert_eq!(true, eval.is_sprite_on_scanline(48));
    assert_eq!(true, eval.is_sprite_on_scanline(49));
    assert_eq!(false, eval.is_sprite_on_scanline(50));
}
