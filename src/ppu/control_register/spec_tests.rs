use super::*;

#[test]
fn vram_addr_increment() {
    let ppu_ctrl = new_control_register(0b00000000);
    assert_eq!(IncrementAmount::One, ppu_ctrl.vram_addr_increment());

    let ppu_ctrl = new_control_register(0b00000100);
    assert_eq!(IncrementAmount::ThirtyTwo, ppu_ctrl.vram_addr_increment());
}

#[test]
fn sprite_size() {
    let ppu_ctrl = new_control_register(0b00000000);
    assert_eq!(SpriteSize::X8, ppu_ctrl.sprite_size());

    let ppu_ctrl = new_control_register(0b00100000);
    assert_eq!(SpriteSize::X16, ppu_ctrl.sprite_size());
}

#[test]
fn nmi_on_vblank_start() {
    let ppu_ctrl = new_control_register(0b00000000);
    assert_eq!(false, ppu_ctrl.nmi_on_vblank_start());

    let ppu_ctrl = new_control_register(0b10000000);
    assert_eq!(true, ppu_ctrl.nmi_on_vblank_start());
}

fn new_control_register(val: u8) -> ControlRegister {
    ControlRegister { reg: val }
}
