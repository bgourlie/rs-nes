use ppu::registers::ppu_status::*;

#[test]
fn flag_getters_and_setters_smoke() {
    let mut ppu_mask = PpuStatus::new();
    ppu_mask.set_flag(FLG_VBLANK, true);
    let flg = ppu_mask.get_flag(FLG_VBLANK);
    assert_eq!(true, flg);

    ppu_mask.set_flag(FLG_VBLANK, false);
    let flg = ppu_mask.get_flag(FLG_VBLANK);
    assert_eq!(false, flg);
}
