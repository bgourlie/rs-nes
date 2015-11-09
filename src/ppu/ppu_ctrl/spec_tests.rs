use ppu::ppu_ctrl::*;

#[test]
fn flag_getters_and_setters_smoke() {
    let mut ppu_ctrl = PpuCtrl::new();
    ppu_ctrl.set_flag(FLG_NMI_ENABLE, true);
    let flg = ppu_ctrl.get_flag(FLG_NMI_ENABLE);
    assert_eq!(true, flg);

    ppu_ctrl.set_flag(FLG_NMI_ENABLE, false);
    let flg = ppu_ctrl.get_flag(FLG_NMI_ENABLE);
    assert_eq!(false, flg);
}

#[test]
fn set_nametable_select_only_affects_two_bits() {
    let mut ppu_ctrl = PpuCtrl::new();
    ppu_ctrl.set_nametable_select(0xff);
    let reg = ppu_ctrl.get_reg();
    assert_eq!(0x3, reg);
}

#[test]
fn get_nametable_base_addr_returns_correct_values() {
    let mut ppu_ctrl = PpuCtrl::new();
    ppu_ctrl.set_nametable_select(0);
    let addr = ppu_ctrl.get_nametable_base_addr();
    assert_eq!(0x2000_u16, addr);
    ppu_ctrl.set_nametable_select(1);
    let addr = ppu_ctrl.get_nametable_base_addr();
    assert_eq!(0x2400_u16, addr);
    ppu_ctrl.set_nametable_select(2);
    let addr = ppu_ctrl.get_nametable_base_addr();
    assert_eq!(0x2800_u16, addr);
    ppu_ctrl.set_nametable_select(3);
    let addr = ppu_ctrl.get_nametable_base_addr();
    assert_eq!(0x2c00_u16, addr);
}
