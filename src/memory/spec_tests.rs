use memory::Memory;

#[test]
fn store_and_load_from_ram_succeeds() {
  let mut mem = Memory::new();
  let stored = 0xB;
  let _ = mem.store(0x0, stored);
  let loaded = mem.load(0x0);
  assert_eq!(stored, loaded);
}

#[test]
fn store16_and_load16() {
  let mut mem = Memory::new();
  let stored = 0xbeef;
  let _ = mem.store16(0x0, stored);
  let loaded = mem.load16(0x0);
  assert_eq!(stored, loaded);
}

#[test]
fn load16_zp_indexed_should_wrap_on_carry() {
  let mut mem = Memory::new();
  mem.store16(0x20, 0xbeef);
  // 0xC0 + 0x60 = 0x120, but the carry will be dropped (wrap) back around
  // into the zero page.  So, we'll expect to find 0xbeef at 0x20
  let val = mem.load16_zp_indexed(0xC0, 0x60);
  assert_eq!(0xbeef, val);
}

#[test]
fn load16_zp_indexed_shouldnt_wrap_no_carry() {
  let mut mem = Memory::new();
  mem.store16(0x20, 0xbeef);
  let val = mem.load16_zp_indexed(0x10, 0x10);
  assert_eq!(0xbeef, val);
}
