extern crate rs_nes;

use rs_nes::memory::{Memory, MemoryError};

#[test]
fn store_and_load_from_ram_succeeds() {
  let mut mem = Memory::new();
  let stored = 0xB;
  let _ = mem.store(0x0, stored);
  let loaded = mem.load(0x0).unwrap();
  assert_eq!(stored, loaded);
}

#[test]
fn ram_should_be_mirrored() {
  let mut mem = Memory::new();
  let stored = 0xB;
  let _ = mem.store(0x0, stored);
  let mirror1 = mem.load(0x800).unwrap();
  let mirror2 = mem.load(0x1000).unwrap();
  let mirror3 = mem.load(0x1800).unwrap();
  assert_eq!(stored, mirror1);
  assert_eq!(stored, mirror2);
  assert_eq!(stored, mirror3);
}

#[test]
fn write_to_mirrored_ram_should_error() {
  let mut mem = Memory::new();
  match mem.store(0x1000, 0x0) {
    Err(MemoryError::WriteToMirror) => { /* expected */ },
    _ => assert!(false)
  }
}

#[test]
fn store_to_rom_should_error() {
  let mut mem = Memory::new();
  match mem.store(0x8000, 0x0) {
    Err(MemoryError::WriteToROM) => { /* expected */ },
    _ => assert!(false)
  }
}

#[test]
fn store_out_of_bounds_address_returns_error() {
  let mut mem = Memory::new();
  match mem.store(0xffffff, 0xb) {
    Err(MemoryError::AddressOutOfBounds) => { /* expected */ },
    _ => assert!(false)
  };
}
