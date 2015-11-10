use ppu::ppu_mask::*;

#[test]
fn flag_getters_and_setters_smoke() {
    let mut ppu_mask = PpuMask::new();
    ppu_mask.set_flag(FLG_GRAYSCALE, true);
    let flg = ppu_mask.get_flag(FLG_GRAYSCALE);
    assert_eq!(true, flg);

    ppu_mask.set_flag(FLG_GRAYSCALE, false);
    let flg = ppu_mask.get_flag(FLG_GRAYSCALE);
    assert_eq!(false, flg);
}
