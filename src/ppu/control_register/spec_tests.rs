use super::*;

#[test]
fn vram_addr_increment() {
    let ppu_ctrl = new_control_register(0b00000000);
    assert_eq!(IncrementAmount::One, ppu_ctrl.vram_addr_increment());

    let ppu_ctrl = new_control_register(0b00000100);
    assert_eq!(IncrementAmount::ThirtyTwo, ppu_ctrl.vram_addr_increment());
}

#[test]
fn sprite_pattern_table_addr() {
    let ppu_ctrl = new_control_register(0b00000000);
    assert_eq!(PatternTableSelect::Left, ppu_ctrl.sprite_pattern_table());

    let ppu_ctrl = new_control_register(0b00001000);
    assert_eq!(PatternTableSelect::Right, ppu_ctrl.sprite_pattern_table());
}

#[test]
fn background_pattern_table_addr() {
    let ppu_ctrl = new_control_register(0b00000000);
    assert_eq!(PatternTableSelect::Left, ppu_ctrl.bg_pattern_table());

    let ppu_ctrl = new_control_register(0b00010000);
    assert_eq!(PatternTableSelect::Right, ppu_ctrl.bg_pattern_table());
}

#[test]
fn sprite_size() {
    let ppu_ctrl = new_control_register(0b00000000);
    assert_eq!(SpriteSize::X8, ppu_ctrl.sprite_size());

    let ppu_ctrl = new_control_register(0b00100000);
    assert_eq!(SpriteSize::X16, ppu_ctrl.sprite_size());
}

#[test]
fn ppu_mode() {
    let ppu_ctrl = new_control_register(0b00000000);
    assert_eq!(PpuMode::Master, ppu_ctrl.ppu_mode());

    let ppu_ctrl = new_control_register(0b01000000);
    assert_eq!(PpuMode::Slave, ppu_ctrl.ppu_mode());
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
