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
