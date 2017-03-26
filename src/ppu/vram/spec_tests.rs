use super::*;
use ppu::control_register::IncrementAmount;
use ppu::write_latch::LatchState;
use rom::NesRom;

#[test]
fn write_address() {
    let vram = vram_fixture();
    assert_eq!(0, vram.address.get());
    assert_eq!(0, vram.address.get());

    vram.write_ppu_addr(LatchState::FirstWrite(0x10));
    vram.write_ppu_addr(LatchState::SecondWrite(0x11));
    assert_eq!(0x1011, vram.address.get());

    vram.write_ppu_addr(LatchState::FirstWrite(0x12));
    vram.write_ppu_addr(LatchState::SecondWrite(0x13));
    assert_eq!(0x1213, vram.address.get());
}

#[test]
#[ignore] // TODO: This is failing due to new ppu data buffering
fn internal_memory_mapping_read() {
    let mut vram = vram_fixture();
    vram.rom.chr = vec![1; 0x2000];
    vram.name_tables = [2; 0x1000];

    for _ in 0..0x2000 {
        assert_eq!(1, vram.read_ppu_data(IncrementAmount::One).unwrap())
    }

    for _ in 0x2000..0x3f00 {
        assert_eq!(2, vram.read_ppu_data(IncrementAmount::One).unwrap())
    }
}

#[test]
fn write_mapping() {
    // Tests pattern and nametable write mappings, palette mapping tested separately

    let mut vram = vram_fixture();

    for _ in 0..0x2000 {
        vram.write_ppu_data(1, IncrementAmount::One).unwrap()
    }

    for _ in 0x2000..0x3f00 {
        vram.write_ppu_data(2, IncrementAmount::One).unwrap()
    }

    assert_eq!(true,
               vram.rom
                   .chr
                   .into_iter()
                   .all(|val| val == 1));
    assert_eq!(true, vram.name_tables.into_iter().all(|val| *val == 2));
}


#[test]
fn ppu_addr_mirroring() {
    let vram = vram_fixture();

    vram.write_ppu_addr(LatchState::FirstWrite(0x10));
    vram.write_ppu_addr(LatchState::SecondWrite(0x20));

    assert_eq!(0x1020, vram.address.get());

    vram.write_ppu_addr(LatchState::FirstWrite(0x3f));
    vram.write_ppu_addr(LatchState::SecondWrite(0xff));
    assert_eq!(0x3fff, vram.address.get());

    vram.write_ppu_addr(LatchState::FirstWrite(0x40));
    vram.write_ppu_addr(LatchState::SecondWrite(0x00));
    assert_eq!(0x0, vram.address.get());

    vram.write_ppu_addr(LatchState::FirstWrite(0x40));
    vram.write_ppu_addr(LatchState::SecondWrite(0x01));
    assert_eq!(0x1, vram.address.get());

    vram.write_ppu_addr(LatchState::FirstWrite(0x7f));
    vram.write_ppu_addr(LatchState::SecondWrite(0xff));
    assert_eq!(0x3fff, vram.address.get());

    vram.write_ppu_addr(LatchState::FirstWrite(0x80));
    vram.write_ppu_addr(LatchState::SecondWrite(0x00));
    assert_eq!(0x0, vram.address.get());

    vram.write_ppu_addr(LatchState::FirstWrite(0xff));
    vram.write_ppu_addr(LatchState::SecondWrite(0xff));
    assert_eq!(0x3fff, vram.address.get());

    vram.write_ppu_addr(LatchState::FirstWrite(0xff));
    assert_eq!(0x3fff, vram.address.get());
}

#[test]
fn palette_read_mapping() {
    // Verifying the following:
    // Addresses $3F10/$3F14/$3F18/$3F1C are mirrors of $3F00/$3F04/$3F08/$3F0C for reads and writes

    let mut vram = vram_fixture();

    for i in 0..0x20 {
        vram.palette[i] = (0x20 - i) as _;
    }

    assert_eq!(0x20, vram.read(0x3f10).unwrap());
    assert_eq!(0x1c, vram.read(0x3f14).unwrap());
    assert_eq!(0x18, vram.read(0x3f18).unwrap());
    assert_eq!(0x14, vram.read(0x3f1c).unwrap());
}

#[test]
fn addr_first_write() {
    // Verify correct temporary VRAM address changes during first address register write:
    // t: ..FEDCBA ........ = d: ..FEDCBA
    // t: .X...... ........ = 0

    let vram = vram_fixture();
    vram.t.set(0b0100_0000_0000_0000);
    vram.write_ppu_addr(LatchState::FirstWrite(0b1111_1111));
    assert_eq!(0b0011_1111_0000_0000, vram.t.get());

    vram.t.set(0b1100_0000_1111_1111);
    vram.write_ppu_addr(LatchState::FirstWrite(0b0011_0101));
    assert_eq!(0b1011_0101_1111_1111, vram.t.get());
}

#[test]
fn addr_second_write() {
    // Verify correct temporary VRAM address changes during second address register write:
    // t: ....... HGFEDCBA = d: HGFEDCBA
    // v                   = t

    let vram = vram_fixture();
    vram.t.set(0);
    vram.write_ppu_addr(LatchState::SecondWrite(0b1111_1111));
    assert_eq!(0b0000_0000_1111_1111, vram.t.get());
    assert_eq!(vram.t.get(), vram.address.get());

    vram.t.set(0b1111_1111_0000_0000);
    vram.write_ppu_addr(LatchState::SecondWrite(0b1010_1010));
    assert_eq!(0b0111_1111_1010_1010, vram.t.get());
    assert_eq!(vram.t.get(), vram.address.get());

}

#[test]
fn copy_horizontal_pos_to_addr() {
    // Test scanline cycle 257 behavior
    // v: ....F.. ...EDCBA = t: ....F.. ...EDCBA
    let vram = vram_fixture();
    vram.t.set(0xffff);
    vram.address.set(0);

    vram.copy_horizontal_pos_to_addr();

    assert_eq!(0b0000_0100_0001_1111, vram.address.get())
}

#[test]
fn scroll_first_write() {
    // Verify correct temporary VRAM address changes during first scroll register writes:
    // t: ....... ...HGFED = d: HGFED...

    let vram = vram_fixture();
    vram.t.set(0);
    vram.scroll_write(LatchState::FirstWrite(0b1111_1000));
    assert_eq!(0b0000_0000_0001_1111, vram.t.get());

    vram.t.set(0b0111_1111_1110_0000);
    vram.scroll_write(LatchState::FirstWrite(0b1111_1000));
    assert_eq!(0b0111_1111_1111_1111, vram.t.get());

    vram.t.set(0b0101_1010_1110_0100);
    vram.scroll_write(LatchState::FirstWrite(0b1101_1000));
    assert_eq!(0b0101_1010_1111_1011, vram.t.get());
}

#[test]
fn scroll_second_write() {
    // Verify correct temporary VRAM address changes during second scroll register writes:
    // t: CBA..HG FED..... = d: HGFEDCBA

    let vram = vram_fixture();
    vram.t.set(0);
    vram.scroll_write(LatchState::SecondWrite(0b0000_0111));
    assert_eq!(0b0111_0000_0000_0000, vram.t.get());

    vram.t.set(0);
    vram.scroll_write(LatchState::SecondWrite(0b1111_1000));
    assert_eq!(0b0000_0011_1110_0000, vram.t.get());


    vram.t.set(0b0000_1100_0001_1111);
    vram.scroll_write(LatchState::SecondWrite(0b1111_1111));
    assert_eq!(0b0111_1111_1111_1111, vram.t.get());
}

#[test]
fn control_write() {
    // Verify correct temporary VRAM address changes during control register writes:
    // t: ...BA.. ........ = d: ......BA
    let vram = vram_fixture();

    vram.t.set(0b0000_0000_0000_0000);
    vram.control_write(0b0000_0011);
    assert_eq!(0b0000_1100_0000_0000, vram.t.get());

    vram.t.set(0b0111_0011_1111_1111);
    vram.control_write(0b0000_0010);
    assert_eq!(0b0111_1011_1111_1111, vram.t.get());

    vram.t.set(0b0111_0011_1111_1111);
    vram.control_write(0b0000_0010);
    assert_eq!(0b0111_1011_1111_1111, vram.t.get());

    vram.t.set(0b0111_0011_1111_1111);
    vram.control_write(0b0000_0001);
    assert_eq!(0b0111_0111_1111_1111, vram.t.get());
}

#[test]
fn palette_write_mapping() {
    // Verifying the following:
    // Addresses $3F10/$3F14/$3F18/$3F1C are mirrors of $3F00/$3F04/$3F08/$3F0C for reads and writes

    let mut vram = vram_fixture();
    vram.write_ppu_addr(LatchState::FirstWrite(0x3f));
    vram.write_ppu_addr(LatchState::SecondWrite(0x00));
    for i in 0..0x20 {
        vram.write_ppu_data(i, IncrementAmount::One).unwrap();
    }

    assert_eq!(0x10, vram.palette[0x0]);
    assert_eq!(0x14, vram.palette[0x4]);
    assert_eq!(0x18, vram.palette[0x8]);
    assert_eq!(0x1c, vram.palette[0xc]);
}

fn vram_fixture() -> VramBase {
    let mut rom = NesRom::default();
    rom.chr = vec![0; 0x2000];
    VramBase::new(rom)
}
