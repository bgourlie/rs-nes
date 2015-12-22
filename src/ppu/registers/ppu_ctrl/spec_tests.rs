use ppu::registers::ppu_ctrl::*;


#[test]
fn nametable_base_addr() {
    let mut ppu_ctrl = PpuCtrl::new();
    *ppu_ctrl = 0x0;
    let addr = ppu_ctrl.nametable_base_addr();
    assert_eq!(0x2000, addr);
    *ppu_ctrl = 0x1;
    let addr = ppu_ctrl.nametable_base_addr();
    assert_eq!(0x2400, addr);
    *ppu_ctrl = 0x2;
    let addr = ppu_ctrl.nametable_base_addr();
    assert_eq!(0x2800, addr);
    *ppu_ctrl = 0x3;
    let addr = ppu_ctrl.nametable_base_addr();
    assert_eq!(0x2c00, addr);
}
