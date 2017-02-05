use super::Ppu;
use super::StatusRegister;

// TODO: Tests that require interaction with OAM registers have subtle non-obvious interactions.
// This doesn't make them suitable for a spec test, is there any way to make this nicer?

#[test]
fn memory_mapped_register_write() {
    let mut ppu = Ppu::new();

    // Writes to 0x2000 write the control register
    ppu.memory_mapped_register_write(0x2000, 0x1);
    assert_eq!(0x1, *ppu.control);

    // Writes to 0x2001 write the mask register
    ppu.memory_mapped_register_write(0x2001, 0x2);
    assert_eq!(0x2, *ppu.mask);

    // Writes to 0x2003 write the oam addr register
    ppu.memory_mapped_register_write(0x2003, 0x3);
    assert_eq!(0x3, ppu.oam.get_address());

    // Writes to 0x2004 write the oam data register
    // We need to reset the oam address before writing and reading since it increments
    ppu.oam.set_address(0x0);
    ppu.memory_mapped_register_write(0x2004, 0x4);
    ppu.oam.set_address(0x0);
    assert_eq!(0x4, ppu.oam.read_data());

    // Writes to 0x2005 write the scroll register
    ppu.scroll.clear_latch();
    ppu.memory_mapped_register_write(0x2005, 0x5);
    assert_eq!(0x5, ppu.scroll.x_pos);
    ppu.memory_mapped_register_write(0x2005, 0x6);
    assert_eq!(0x6, ppu.scroll.y_pos);

    // Writes to 0x2006 write the vram addr register
    ppu.vram.clear_latch();
    ppu.memory_mapped_register_write(0x2006, 0x20);
    ppu.memory_mapped_register_write(0x2006, 0x03);
    assert_eq!(0x2003, ppu.vram.address());

    // Writes to 0x2007 write the vram data register
    // HERE
    ppu.vram.clear_latch();
    ppu.vram.write_address(0x20);
    ppu.vram.write_address(0x01);
    ppu.memory_mapped_register_write(0x2007, 0x7);
    ppu.vram.write_address(0x20);
    ppu.vram.write_address(0x01);
    assert_eq!(0x7, ppu.vram.read_data());

    // Test mirroring: 0x2000-0x2007 are mirrored every 8 bytes to 0x3fff

    ppu.memory_mapped_register_write(0x2008, 0x8);
    assert_eq!(0x8, *ppu.control);

    ppu.memory_mapped_register_write(0x2009, 0x9);
    assert_eq!(0x9, *ppu.mask);

    ppu.memory_mapped_register_write(0x200b, 0xa);
    assert_eq!(0xa, ppu.oam.get_address());

    // We need to reset the oam address before writing and reading since it increments
    ppu.oam.set_address(0x0);
    ppu.memory_mapped_register_write(0x200c, 0xb);
    ppu.oam.set_address(0x0);
    assert_eq!(0xb, ppu.oam.read_data());

    ppu.scroll.clear_latch();
    ppu.memory_mapped_register_write(0x200d, 0xc);
    assert_eq!(0xc, ppu.scroll.x_pos);
    ppu.memory_mapped_register_write(0x200d, 0xd);
    assert_eq!(0xd, ppu.scroll.y_pos);

    ppu.vram.clear_latch();
    ppu.memory_mapped_register_write(0x200e, 0x20);
    ppu.memory_mapped_register_write(0x200e, 0x01);
    assert_eq!(0x2001, ppu.vram.address());

    ppu.vram.clear_latch();
    ppu.vram.write_address(0x20);
    ppu.vram.write_address(0x01);
    ppu.memory_mapped_register_write(0x200f, 0x14);
    ppu.vram.write_address(0x20);
    ppu.vram.write_address(0x01);
    assert_eq!(0x14, ppu.vram.read_data());

    // Test mirroring on the tail end of the valid address space

    ppu.memory_mapped_register_write(0x3ff8, 0xf);
    assert_eq!(0xf, *ppu.control);

    ppu.memory_mapped_register_write(0x3ff9, 0x10);
    assert_eq!(0x10, *ppu.mask);

    ppu.memory_mapped_register_write(0x3ffb, 0x11);
    assert_eq!(0x11, ppu.oam.get_address());

    // We need to reset the oam address before writing and reading since it increments
    ppu.oam.set_address(0x0);
    ppu.memory_mapped_register_write(0x3ffc, 0x12);
    ppu.oam.set_address(0x0);
    assert_eq!(0x12, ppu.oam.read_data());

    ppu.scroll.clear_latch();
    ppu.memory_mapped_register_write(0x3ffd, 0x13);
    assert_eq!(0x13, ppu.scroll.x_pos);
    ppu.memory_mapped_register_write(0x3ffd, 0x14);
    assert_eq!(0x14, ppu.scroll.y_pos);

    ppu.vram.clear_latch();
    ppu.vram.write_address(0x20);
    ppu.vram.write_address(0x01);
    assert_eq!(0x2001, ppu.vram.address());

    ppu.vram.clear_latch();
    ppu.vram.write_address(0x20);
    ppu.vram.write_address(0x01);
    ppu.memory_mapped_register_write(0x3fff, 0x15);
    ppu.vram.write_address(0x20);
    ppu.vram.write_address(0x01);
    assert_eq!(0x15, ppu.vram.read_data());
}

#[test]
fn memory_mapped_register_read() {
    let mut ppu = Ppu::new();

    ppu.control.set(0xf0);
    assert_eq!(0xf0, ppu.memory_mapped_register_read(0x2000));

    ppu.mask.set(0xf1);
    assert_eq!(0xf1, ppu.memory_mapped_register_read(0x2001));

    ppu.status = StatusRegister::new(0xf2);
    assert_eq!(0xf2, ppu.memory_mapped_register_read(0x2002));

    ppu.oam.write_data(0xf3);
    assert_eq!(0, ppu.memory_mapped_register_read(0x2003)); // write-only, should always read 0

    // We need to reset the oam address before writing and reading since it increments
    ppu.oam.set_address(0x0);
    ppu.oam.write_data(0xf4);
    ppu.oam.set_address(0x0);
    assert_eq!(0xf4, ppu.memory_mapped_register_read(0x2004));

    ppu.scroll.write(0xf5);
    assert_eq!(0x0, ppu.memory_mapped_register_read(0x2005)); // write-only, should always read 0

    ppu.vram.write_address(0xf6);
    assert_eq!(0x0, ppu.memory_mapped_register_read(0x2006)); // write-only, should always read 0

    ppu.vram.clear_latch();
    ppu.vram.write_address(0x20);
    ppu.vram.write_address(0x07);
    ppu.vram.write_data(0xf7);
    ppu.vram.write_address(0x20);
    ppu.vram.write_address(0x07);
    assert_eq!(0xf7, ppu.memory_mapped_register_read(0x2007));

    // Test mirroring: 0x2000-0x2007 are mirrored every 8 bytes to 0x3fff

    ppu.control.set(0xe0);
    assert_eq!(0xe0, ppu.memory_mapped_register_read(0x2008));

    ppu.mask.set(0xe1);
    assert_eq!(0xe1, ppu.memory_mapped_register_read(0x2009));

    ppu.status = StatusRegister::new(0xe2);
    assert_eq!(0xe2, ppu.memory_mapped_register_read(0x200a));

    ppu.oam.set_address(0xe3);
    assert_eq!(0, ppu.memory_mapped_register_read(0x200b)); // write-only, should always read 0

    // We need to reset the oam address before writing and reading since it increments
    ppu.oam.set_address(0x0);
    ppu.oam.write_data(0xe4);
    ppu.oam.set_address(0x0);
    assert_eq!(0xe4, ppu.memory_mapped_register_read(0x200c));

    ppu.scroll.write(0xe5);
    assert_eq!(0x0, ppu.memory_mapped_register_read(0x200d)); // write-only, should always read 0

    ppu.vram.write_address(0xe6);
    assert_eq!(0x0, ppu.memory_mapped_register_read(0x200e)); // write-only, should always read 0

    ppu.vram.clear_latch();
    ppu.vram.write_address(0x20);
    ppu.vram.write_address(0x0f);
    ppu.vram.write_data(0xe7);
    ppu.vram.write_address(0x20);
    ppu.vram.write_address(0x0f);
    assert_eq!(0xe7, ppu.memory_mapped_register_read(0x200f));

    // Test mirroring on the tail end of the valid address space

    ppu.control.set(0xd0);
    assert_eq!(0xd0, ppu.memory_mapped_register_read(0x3ff8));

    ppu.mask.set(0xd1);
    assert_eq!(0xd1, ppu.memory_mapped_register_read(0x3ff9));

    ppu.status = StatusRegister::new(0xd2);
    assert_eq!(0xd2, ppu.memory_mapped_register_read(0x3ffa));

    ppu.oam.set_address(0xd3);
    assert_eq!(0, ppu.memory_mapped_register_read(0x3ffb)); // write-only, should always read 0

    // We need to reset the oam address before writing and reading since it increments
    ppu.oam.set_address(0x0);
    ppu.oam.write_data(0xd4);
    ppu.oam.set_address(0x0);
    assert_eq!(0xd4, ppu.memory_mapped_register_read(0x3ffc));

    ppu.scroll.write(0xd5);
    assert_eq!(0x0, ppu.memory_mapped_register_read(0x3ffd)); // write-only, should always read 0

    ppu.vram.write_address(0xd6);
    assert_eq!(0x0, ppu.memory_mapped_register_read(0x3ffe)); // write-only, should always read 0

    ppu.vram.clear_latch();
    ppu.vram.write_address(0x20);
    ppu.vram.write_address(0x01);
    ppu.vram.write_data(0xd7);
    ppu.vram.write_address(0x20);
    ppu.vram.write_address(0x01);
    assert_eq!(0xd7, ppu.memory_mapped_register_read(0x3fff));
}

#[test]
fn vblank_set_and_clear_cycles() {
    // VBLANK is clear until cycle 1 of the 241st scanline (which is the second cycle, as there is a
    // cycle 0). Therefore:
    // - Cycle 0 of scanline 241 = vblank clear
    // - Cycle 1 of scanline 241 = VBLANK set during this cycle, but still considered pre-vblank
    // - Cycle 2 of scanline 241 = post-vblank
    const VBLANK_OFF: u64 = super::CYCLES_PER_SCANLINE * super::VBLANK_SCANLINE + 1;
    const VBLANK_ON: u64 = VBLANK_OFF + 1;

    // VBLANK is set until cycle 1 of the 261st scanline, similar to above. Therefore:
    // - Cycle 0 of scanline 261 = in-vblank
    // - Cycle 1 of scanline 241 = VBLANK cleared during this cycle, but still considered in-vblank
    // - Cycle 2 of scanline 241 = vblank clear
    const CLEAR_VBLANK_CYCLE: u64 = super::CYCLES_PER_SCANLINE * super::LAST_SCANLINE + 1;
    const VBLANK_OFF_AGAIN: u64 = CLEAR_VBLANK_CYCLE + 1;

    let mut ppu = Ppu::new();

    // Render 100 frames and assert expected VBLANK behavior
    while ppu.cycles < super::CYCLES_PER_FRAME * 100 {
        match ppu.cycles % super::CYCLES_PER_FRAME {
            0...VBLANK_OFF => assert_eq!(false, ppu.status.in_vblank()),
            VBLANK_ON...CLEAR_VBLANK_CYCLE => assert_eq!(true, ppu.status.in_vblank()),
            VBLANK_OFF_AGAIN...super::CYCLES_PER_FRAME => assert_eq!(false, ppu.status.in_vblank()),
            _ => panic!("We should never get here"),
        }
        ppu.step();
    }
}

#[test]
fn vblank_clear_after_status_read() {
    let ppu = Ppu::new();
    ppu.status.set_in_vblank();
    let status = ppu.memory_mapped_register_read(0x2002);
    assert_eq!(true, status & 0b10000000 > 0);
    assert_eq!(true, ppu.status.value() & 0b10000000 == 0);
}

#[test]
fn oam_read_non_blanking_increments_addr() {
    let mut ppu = Ppu::new();
    ppu.status.clear_in_vblank();
    ppu.mask.set(1);
    ppu.oam.set_address(0x0);
    ppu.memory_mapped_register_read(0x2004);
    assert_eq!(0x1, ppu.oam.get_address());
}

#[test]
fn oam_read_v_blanking_doesnt_increments_addr() {
    let mut ppu = Ppu::new();
    ppu.status.set_in_vblank();
    ppu.mask.set(1);
    ppu.oam.set_address(0x0);
    ppu.memory_mapped_register_read(0x2004);
    assert_eq!(0x0, ppu.oam.get_address());
}

#[test]
fn oam_read_forced_blanking_doesnt_increments_addr() {
    let mut ppu = Ppu::new();
    ppu.status.clear_in_vblank();
    ppu.mask.set(0);
    ppu.oam.set_address(0x0);
    ppu.memory_mapped_register_read(0x2004);
    assert_eq!(0x0, ppu.oam.get_address());
}
