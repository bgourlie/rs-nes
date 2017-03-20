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
    assert_eq!(0x1000, vram.address.get());

    vram.write_ppu_addr(LatchState::SecondWrite(0x11));
    assert_eq!(0x1011, vram.address.get());

    vram.write_ppu_addr(LatchState::FirstWrite(0x12));
    assert_eq!(0x1211, vram.address.get());

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
