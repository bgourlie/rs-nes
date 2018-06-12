use cpu6502::cpu::Interconnect;
use interconnect::NesInterconnect;
use mocks::{ApuMock, CartMock, InputMock, PpuMock};
use std::rc::Rc;

#[test]
fn ram_memory_mapped_read() {
    let mut fixture = new_fixture();

    for addr in 0..0x800 {
        fixture.ram[addr] = (addr & 0xfe) as u8;
    }

    for addr in 0..0x2000_u16 {
        let expect = (addr & 0xfe) as u8;
        let val = fixture.read(addr);
        assert_eq!(expect, val);
    }
}

#[test]
fn ram_memory_mapped_write() {
    let mut fixture = new_fixture();
    for addr in 0..0x2000_u16 {
        let ram_index = (addr & 0x7ff) as usize;
        fixture.write(addr, 0xff);
        assert_eq!(0xff, fixture.ram[ram_index]);
        fixture.ram[ram_index] = 0; // reset it after asserting it was written correctly
    }
}

#[test]
fn ppu_memory_mapped_read() {
    let mut fixture = new_fixture();
    fixture.ppu.set_value(0xff);
    for addr in 0x2000..0x2008_u16 {
        let val = fixture.read(addr);
        assert_eq!(0xff, val);
    }
}

#[test]
fn ppu_memory_mapped_write() {
    let mut fixture = new_fixture();
    for addr in 0x2000..0x2008_u16 {
        fixture.write(addr, 0xff);
        assert_eq!(addr, fixture.ppu.addr());
        assert_eq!(0xff, fixture.ppu.value());
    }
}

#[test]
fn apu_memory_mapped_read() {
    let mut fixture = new_fixture();
    fixture.apu.set_control(0xff);
    // Only a single APU address is readable
    let val = fixture.read(0x4015);
    assert_eq!(0xff, val);
}

#[test]
fn apu_memory_mapped_write() {
    let mut fixture = new_fixture();

    for addr in 0x4000..0x4014_u16 {
        fixture.write(addr, 0xff);
        assert_eq!(addr, fixture.apu.write_addr());
        assert_eq!(0xff, fixture.apu.write_value());
    }
    // Skip 0x4014, since it's ppu DMA address

    fixture.write(0x4015, 0xff);
    assert_eq!(0x4015, fixture.apu.write_addr());
    assert_eq!(0xff, fixture.apu.write_value());

    // Skip 0x4016 since it's input probe register

    for addr in 0x4017..0x4018_u16 {
        fixture.write(addr, 0xff);
        assert_eq!(addr, fixture.apu.write_addr());
        assert_eq!(0xff, fixture.apu.write_value());
    }
}

#[test]
#[ignore]
fn input_memory_mapped_read() {
    // TODO: reimplement
}

#[test]
#[ignore]
fn input_memory_mapped_write() {
    // TODO: reimplement
}

#[test]
fn oam_dma_timing_even_cycle() {
    let mut fixture = new_fixture();
    fixture.write(0x4014, 0x02);
    assert_eq!(513, fixture.elapsed_cycles());
}

#[test]
fn oam_dma_timing_odd_cycle() {
    let mut fixture = new_fixture();
    fixture.tick();
    fixture.write(0x4014, 0x02);
    assert_eq!(514, fixture.elapsed_cycles() - 1);
}

fn new_fixture() -> NesInterconnect<PpuMock, ApuMock, InputMock, CartMock> {
    NesInterconnect {
        ram: [0_u8; 0x800],
        rom: Rc::new(Box::new(CartMock::default())),
        ppu: PpuMock::default(),
        apu: ApuMock::default(),
        input: InputMock::default(),
        elapsed_cycles: 0,
    }
}
