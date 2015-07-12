extern crate rs_nes;

use rs_nes::memory::Memory;

#[test]
fn store_and_load() {
  let mut mem = Memory::new();
  let stored = 0xB;
  let _ = mem.store(0x0, stored);
  let loaded = mem.load(0x0).unwrap();
  assert_eq!(stored, loaded);
}

#[test]
fn store_and_load_errors_when_address_out_of_bounds() {
  let mut mem = Memory::new();
  match mem.store(0xffffff, 0xb) {
    Err(_) => {
      // we want an error to be thrown!
      assert!(true);
    }
    _ => {
      assert!(false);
    }
  };
}
