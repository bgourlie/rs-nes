use super::Ppu;

#[test]
fn memory_mapped_register_write_test() {
    let mut ppu = Ppu::new();

    // Writes to 0x2000 write the control register
    ppu.memory_mapped_register_write(0x2000, 0x1);
    assert_eq!(0x1, *ppu.ctrl_reg);

    // Writes to 0x2001 write the mask register
    ppu.memory_mapped_register_write(0x2001, 0x2);
    assert_eq!(0x2, *ppu.mask_reg);

    // Writes to 0x2003 write the oam addr register
    ppu.memory_mapped_register_write(0x2003, 0x3);
    assert_eq!(0x3, ppu.oam_addr);

    // Writes to 0x2004 write the oam data register
    ppu.memory_mapped_register_write(0x2004, 0x4);
    assert_eq!(0x4, ppu.oam_data);

    // Writes to 0x2005 write the scroll register
    ppu.memory_mapped_register_write(0x2005, 0x5);
    assert_eq!(0x5, ppu.scroll);

    // Writes to 0x2006 write the vram addr register
    ppu.memory_mapped_register_write(0x2006, 0x6);
    assert_eq!(0x6, ppu.vram_addr);

    // Writes to 0x2007 write the vram data register
    ppu.memory_mapped_register_write(0x2007, 0x7);
    assert_eq!(0x7, ppu.vram_data);

    // Test mirroring: 0x2000-0x2007 are mirrored every 8 bytes to 0x3fff

    ppu.memory_mapped_register_write(0x2008, 0x8);
    assert_eq!(0x8, *ppu.ctrl_reg);

    ppu.memory_mapped_register_write(0x2009, 0x9);
    assert_eq!(0x9, *ppu.mask_reg);

    ppu.memory_mapped_register_write(0x200b, 0xa);
    assert_eq!(0xa, ppu.oam_addr);

    ppu.memory_mapped_register_write(0x200c, 0xb);
    assert_eq!(0xb, ppu.oam_data);

    ppu.memory_mapped_register_write(0x200d, 0xc);
    assert_eq!(0xc, ppu.scroll);

    ppu.memory_mapped_register_write(0x200e, 0xd);
    assert_eq!(0xd, ppu.vram_addr);

    ppu.memory_mapped_register_write(0x200f, 0xe);
    assert_eq!(0xe, ppu.vram_data);

    // Test mirroring on the tail end of the valid address space

    ppu.memory_mapped_register_write(0x3ff8, 0xf);
    assert_eq!(0xf, *ppu.ctrl_reg);

    ppu.memory_mapped_register_write(0x3ff9, 0x10);
    assert_eq!(0x10, *ppu.mask_reg);

    ppu.memory_mapped_register_write(0x3ffb, 0x11);
    assert_eq!(0x11, ppu.oam_addr);

    ppu.memory_mapped_register_write(0x3ffc, 0x12);
    assert_eq!(0x12, ppu.oam_data);

    ppu.memory_mapped_register_write(0x3ffd, 0x13);
    assert_eq!(0x13, ppu.scroll);

    ppu.memory_mapped_register_write(0x3ffe, 0x14);
    assert_eq!(0x14, ppu.vram_addr);

    ppu.memory_mapped_register_write(0x3fff, 0x15);
    assert_eq!(0x15, ppu.vram_data);
}
