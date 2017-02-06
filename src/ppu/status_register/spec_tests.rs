use super::StatusRegister;
use std::cell::Cell;

#[test]
fn sprite_overflow() {
    let reg = new_status_register(0b00000000);
    assert_eq!(false, reg.sprite_overflow());

    let reg = new_status_register(0b00100000);
    assert_eq!(true, reg.sprite_overflow());
}

#[test]
fn sprite_zero_hit() {
    let reg = new_status_register(0b00000000);
    assert_eq!(false, reg.sprite_zero_hit());

    let reg = new_status_register(0b01000000);
    assert_eq!(true, reg.sprite_zero_hit());
}

#[test]
fn in_vblank() {
    let reg = new_status_register(0b00000000);
    assert_eq!(false, reg.in_vblank());

    let reg = new_status_register(0b10000000);
    assert_eq!(true, reg.in_vblank());
}

fn new_status_register(val: u8) -> StatusRegister {
    StatusRegister { reg: Cell::new(val) }
}
