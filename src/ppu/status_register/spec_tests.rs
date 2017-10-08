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
fn clear_sprite_overflow() {
    let reg = new_status_register(0b11111111);
    reg.clear_sprite_overflow();
    assert_eq!(0b11011111, reg.reg.get());
}

#[test]
fn sprite_zero_hit() {
    let reg = new_status_register(0b00000000);
    assert_eq!(false, reg.sprite_zero_hit());

    let reg = new_status_register(0b01000000);
    assert_eq!(true, reg.sprite_zero_hit());
}

#[test]
fn set_sprite_zero_hit() {
    let reg = new_status_register(0b00000000);
    reg.set_sprite_zero_hit();
    assert_eq!(0b01000000, reg.reg.get());
}

#[test]
fn clear_sprite_zero_hit() {
    let reg = new_status_register(0b11111111);
    reg.clear_sprite_zero_hit();
    assert_eq!(0b10111111, reg.reg.get());
}

#[test]
fn in_vblank() {
    let reg = new_status_register(0b00000000);
    assert_eq!(false, reg.in_vblank());

    let reg = new_status_register(0b10000000);
    assert_eq!(true, reg.in_vblank());
}

#[test]
fn set_in_vblank() {
    let reg = new_status_register(0b00000000);
    reg.set_in_vblank();
    assert_eq!(0b10000000, reg.reg.get());
}

#[test]
fn clear_in_vblank() {
    let reg = new_status_register(0b11111111);
    reg.clear_in_vblank();
    assert_eq!(0b01111111, reg.reg.get());
}

fn new_status_register(val: u8) -> StatusRegister {
    StatusRegister {
        reg: Cell::new(val),
    }
}
