use ppu::mask_register::{ColorMode, MaskRegister};

#[test]
fn color_mode() {
    let reg = MaskRegister::new(0b00000000);
    assert_eq!(ColorMode::Normal, reg.color_mode());

    let reg = MaskRegister::new(0b00000001);
    assert_eq!(ColorMode::Greyscale, reg.color_mode());
}

#[test]
fn background_render_leftmost_8_px() {
    let reg = MaskRegister::new(0b00000000);
    assert_eq!(false, reg.background_render_leftmost_8_px());

    let reg = MaskRegister::new(0b00000010);
    assert_eq!(true, reg.background_render_leftmost_8_px());
}

#[test]
fn sprites_render_leftmost_8_px() {
    let reg = MaskRegister::new(0b00000000);
    assert_eq!(false, reg.sprites_render_leftmost_8_px());

    let reg = MaskRegister::new(0b00000100);
    assert_eq!(true, reg.sprites_render_leftmost_8_px());
}

#[test]
fn show_background() {
    let reg = MaskRegister::new(0b00000000);
    assert_eq!(false, reg.show_background());

    let reg = MaskRegister::new(0b00001000);
    assert_eq!(true, reg.show_background());
}

#[test]
fn show_sprites() {
    let reg = MaskRegister::new(0b00000000);
    assert_eq!(false, reg.show_sprites());

    let reg = MaskRegister::new(0b00010000);
    assert_eq!(true, reg.show_sprites());
}

#[test]
fn emphasize_red() {
    let reg = MaskRegister::new(0b00000000);
    assert_eq!(false, reg.emphasize_red());

    let reg = MaskRegister::new(0b00100000);
    assert_eq!(true, reg.emphasize_red());
}

#[test]
fn emphasize_green() {
    let reg = MaskRegister::new(0b00000000);
    assert_eq!(false, reg.emphasize_green());

    let reg = MaskRegister::new(0b01000000);
    assert_eq!(true, reg.emphasize_green());
}

#[test]
fn emphasize_blue() {
    let reg = MaskRegister::new(0b00000000);
    assert_eq!(false, reg.emphasize_blue());

    let reg = MaskRegister::new(0b10000000);
    assert_eq!(true, reg.emphasize_blue());
}
