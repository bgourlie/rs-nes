use memory::*;

#[test]
fn store_and_load_from_ram_succeeds() {
    let mut mem = SimpleMemory::new();
    let stored = 0xB;
    let _ = mem.store(0x0, stored);
    let loaded = mem.load(0x0);
    assert_eq!(stored, loaded);
}
