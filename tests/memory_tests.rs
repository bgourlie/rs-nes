extern crate rs_nes;

use rs_nes::memory::Memory;

#[test]
fn store_and_load() {
  let mut mem = Memory::new();
  let stored = 0x1234;
  mem.store(0x0, stored);
  let loaded = mem.load(0x0).unwrap();
  assert_eq!(stored, loaded);
}


