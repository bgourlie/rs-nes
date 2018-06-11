use super::*;

#[test]
fn write_and_read() {
    let mut oam = fixture(&[0xa, 0xb, 0xc, 0xd]);
    oam.write_address(0x1);
    assert_eq!(0xb, oam.read_data());
    assert_eq!(0xb, oam.read_data());
    assert_eq!(0x1, oam.address.get())
}

#[test]
fn write_and_read_increment() {
    let mut mem = [0_u8; 0x100];

    for i in 0..0x100 {
        mem[i] = i as u8;
    }

    let mut oam = fixture(&mem);
    oam.write_address(0xfe);
    assert_eq!(0xfe, oam.read_data_increment_addr());
    assert_eq!(0xff, oam.address.get());
    assert_eq!(0xff, oam.read_data_increment_addr());
    assert_eq!(0x0, oam.address.get());
    assert_eq!(0x0, oam.read_data_increment_addr());
    assert_eq!(0x1, oam.address.get())
}

fn fixture(initial_values: &[u8]) -> SpriteRenderer {
    let mut mem = [0_u8; 0x100];
    for (i, byte) in initial_values.iter().enumerate() {
        mem[i] = *byte;
    }
    let mut fixture = SpriteRenderer::default();
    fixture.primary_oam = mem;
    fixture
}
