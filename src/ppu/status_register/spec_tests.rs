use super::StatusRegister;

#[test]
fn sprite_overflow() {
    let reg = StatusRegister::new(0b00000000);
    assert_eq!(false, reg.sprite_overflow());

    let reg = StatusRegister::new(0b00100000);
    assert_eq!(true, reg.sprite_overflow());
}

#[test]
fn sprite_zero_hit() {
    let reg = StatusRegister::new(0b00000000);
    assert_eq!(false, reg.sprite_zero_hit());

    let reg = StatusRegister::new(0b01000000);
    assert_eq!(true, reg.sprite_zero_hit());
}

#[test]
fn in_vblank() {
    let reg = StatusRegister::new(0b00000000);
    assert_eq!(false, reg.in_vblank());

    let reg = StatusRegister::new(0b10000000);
    assert_eq!(true, reg.in_vblank());
}
