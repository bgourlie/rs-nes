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
fn ram_should_be_mirrored() {
  let mut mem = Memory::new();
  let stored = 0xB;
  let _ = mem.store(0x0, stored);
  let mirror1 = mem.load(0x800);
  let mirror2 = mem.load(0x1000);
  let mirror3 = mem.load(0x1800);
  assert_eq!(stored, mirror1);
  assert_eq!(stored, mirror2);
  assert_eq!(stored, mirror3);
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

#[test]
#[should_panic(expected = "write to mirrored memory")]
fn write_to_mirrored_ram_should_error() {
  let mut mem = Memory::new();
  mem.store(0x1000, 0x0);
}

#[test]
#[should_panic(expected = "write to rom")]
fn store_to_rom_should_error() {
  let mut mem = Memory::new();
  mem.store(0x8000, 0x0);
}
