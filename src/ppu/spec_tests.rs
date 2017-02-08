use super::*;
use super::status_register::StatusRegister;

#[test]
fn write() {
    let mut ppu = mocks::TestPpu::default();

    // Writes to 0x2000 write the control register
    ppu.write(0x2000, 0x1).unwrap();
    assert_eq!(0x1, *ppu.control);

    // Writes to 0x2001 write the mask register
    ppu.write(0x2001, 0x2).unwrap();
    assert_eq!(0x2, *ppu.mask);

    // Writes to 0x2003 write the oam addr register
    ppu.write(0x2003, 0x3).unwrap();
    assert_eq!(0x3, ppu.oam.address_value());

    // Writes to 0x2004 write the oam data register
    ppu.write(0x2004, 0x4).unwrap();
    assert_eq!(0x4, ppu.oam.data_value());

    // Writes to 0x2005 write the scroll register
    ppu.write(0x2005, 0x5).unwrap();
    assert_eq!(0x5, ppu.scroll.value());

    // Writes to 0x2006 write the vram addr register
    ppu.write(0x2006, 0x20).unwrap();
    assert_eq!(0x20, ppu.vram.address_value());

    // Writes to 0x2007 write the vram data register
    ppu.write(0x2007, 0x7).unwrap();
    assert_eq!(0x7, ppu.vram.data_value());

    // Test mirroring: 0x2000-0x2007 are mirrored every 8 bytes to 0x3fff

    ppu.write(0x2008, 0x8).unwrap();
    assert_eq!(0x8, *ppu.control);

    ppu.write(0x2009, 0x9).unwrap();
    assert_eq!(0x9, *ppu.mask);

    ppu.write(0x200b, 0xa).unwrap();
    assert_eq!(0xa, ppu.oam.address_value());

    ppu.write(0x200c, 0xb).unwrap();
    assert_eq!(0xb, ppu.oam.data_value());

    ppu.write(0x200d, 0xc).unwrap();
    assert_eq!(0xc, ppu.scroll.value());

    ppu.write(0x200e, 0x01).unwrap();
    assert_eq!(0x01, ppu.vram.address_value());

    ppu.write(0x200f, 0x14).unwrap();
    assert_eq!(0x14, ppu.vram.data_value());

    // Test mirroring on the tail end of the valid address space

    ppu.write(0x3ff8, 0xf).unwrap();
    assert_eq!(0xf, *ppu.control);

    ppu.write(0x3ff9, 0x10).unwrap();
    assert_eq!(0x10, *ppu.mask);

    ppu.write(0x3ffb, 0x11).unwrap();
    assert_eq!(0x11, ppu.oam.address_value());

    ppu.write(0x3ffc, 0x12).unwrap();
    assert_eq!(0x12, ppu.oam.data_value());

    ppu.write(0x3ffd, 0x13).unwrap();
    assert_eq!(0x13, ppu.scroll.value());

    ppu.vram.write_address(0x02);
    assert_eq!(0x02, ppu.vram.address_value());

    ppu.write(0x3fff, 0x15).unwrap();
    assert_eq!(0x15, ppu.vram.data_value());
}

#[test]
fn memory_mapped_register_read() {
    let mut ppu = mocks::TestPpu::default();

    ppu.control.write(0xf0);
    assert_eq!(0xf0, ppu.read(0x2000).unwrap());

    ppu.mask.write(0xf1);
    assert_eq!(0xf1, ppu.read(0x2001).unwrap());

    ppu.status = StatusRegister::new(0xf2);
    assert_eq!(0xf2, ppu.read(0x2002).unwrap());

    ppu.oam.set_address_value(0xf3);
    assert_eq!(0, ppu.read(0x2003).unwrap()); // write-only, should always read 0

    ppu.oam.set_data_value(0xf4);
    assert_eq!(0xf4, ppu.read(0x2004).unwrap());

    ppu.scroll.write(0xf5);
    assert_eq!(0x0, ppu.read(0x2005).unwrap()); // write-only, should always read 0

    ppu.vram.set_address_value(0xf6);
    assert_eq!(0x0, ppu.read(0x2006).unwrap()); // write-only, should always read 0

    ppu.vram.set_data_value(0xfe);
    assert_eq!(0xfe, ppu.read(0x2007).unwrap());

    // Test mirroring: 0x2000-0x2007 are mirrored every 8 bytes to 0x3fff

    ppu.control.write(0xe0);
    assert_eq!(0xe0, ppu.read(0x2008).unwrap());

    ppu.mask.write(0xe1);
    assert_eq!(0xe1, ppu.read(0x2009).unwrap());

    ppu.status = StatusRegister::new(0xe2);
    assert_eq!(0xe2, ppu.read(0x200a).unwrap());

    ppu.oam.set_address_value(0xe3);
    assert_eq!(0, ppu.read(0x200b).unwrap()); // write-only, should always read 0

    ppu.oam.set_data_value(0xe4);
    assert_eq!(0xe4, ppu.read(0x200c).unwrap());

    ppu.scroll.write(0xe5);
    assert_eq!(0x0, ppu.read(0x200d).unwrap()); // write-only, should always read 0

    ppu.vram.set_address_value(0xe6);
    assert_eq!(0x0, ppu.read(0x200e).unwrap()); // write-only, should always read 0

    ppu.vram.set_data_value(0xfb);
    assert_eq!(0xfb, ppu.read(0x200f).unwrap());

    // Test mirroring on the tail end of the valid address space

    ppu.control.write(0xd0);
    assert_eq!(0xd0, ppu.read(0x3ff8).unwrap());

    ppu.mask.write(0xd1);
    assert_eq!(0xd1, ppu.read(0x3ff9).unwrap());

    ppu.status = StatusRegister::new(0xd2);
    assert_eq!(0xd2, ppu.read(0x3ffa).unwrap());

    ppu.oam.set_address_value(0xd3);
    assert_eq!(0, ppu.read(0x3ffb).unwrap()); // write-only, should always read 0

    ppu.oam.set_data_value(0xd4);
    assert_eq!(0xd4, ppu.read(0x3ffc).unwrap());

    ppu.scroll.write(0xd5);
    assert_eq!(0x0, ppu.read(0x3ffd).unwrap()); // write-only, should always read 0

    ppu.vram.set_address_value(0xd6);
    assert_eq!(0x0, ppu.read(0x3ffe).unwrap()); // write-only, should always read 0

    ppu.vram.set_data_value(0xfc);
    assert_eq!(0xfc, ppu.read(0x3fff).unwrap());
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

    let mut ppu = mocks::TestPpu::default();

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
    let ppu = mocks::TestPpu::default();
    ppu.status.set_in_vblank();
    let status = ppu.read(0x2002).unwrap();
    assert_eq!(true, status & 0b10000000 > 0);
    assert_eq!(true, ppu.status.read() & 0b10000000 == 0);
}

#[test]
fn oam_read_non_blanking_increments_addr() {
    let mut ppu = mocks::TestPpu::default();
    ppu.status.clear_in_vblank();
    ppu.mask.write(1);
    ppu.read(0x2004).unwrap();
    assert_eq!(true, ppu.oam.read_data_increment_addr_called());
    assert_eq!(false, ppu.oam.read_data_called());
}

#[test]
fn oam_read_v_blanking_doesnt_increments_addr() {
    let mut ppu = mocks::TestPpu::default();
    ppu.status.set_in_vblank();
    ppu.mask.write(1);
    ppu.read(0x2004).unwrap();
    assert_eq!(false, ppu.oam.read_data_increment_addr_called());
    assert_eq!(true, ppu.oam.read_data_called());
}

#[test]
fn oam_read_forced_blanking_doesnt_increments_addr() {
    let mut ppu = mocks::TestPpu::default();
    ppu.status.clear_in_vblank();
    ppu.mask.write(0);
    ppu.read(0x2004).unwrap();
    assert_eq!(false, ppu.oam.read_data_increment_addr_called());
    assert_eq!(true, ppu.oam.read_data_called());
}

mod mocks {
    use super::object_attribute_memory::SpriteAttributes;
    use errors::*;
    use ppu::PpuBase;
    use ppu::object_attribute_memory::ObjectAttributeMemory;
    use ppu::scroll_register::ScrollRegister;
    use ppu::vram::Vram;
    use std::cell::Cell;

    pub type TestPpu = PpuBase<MockVram, MockScrollRegister, MockOam>;

    #[derive(Clone, Default)]
    pub struct MockScrollRegister {
        mock_value: u8,
    }

    impl MockScrollRegister {
        pub fn value(&self) -> u8 {
            self.mock_value
        }

        pub fn set_value(&mut self, val: u8) {
            self.mock_value = val
        }
    }

    impl ScrollRegister for MockScrollRegister {
        fn write(&mut self, val: u8) {
            self.set_value(val)
        }

        fn clear_latch(&self) {}
    }

    #[derive(Clone, Default)]
    pub struct MockOam {
        read_data_called: Cell<bool>,
        read_data_increment_addr_called: Cell<bool>,
        mock_addr: u8,
        mock_data: u8,
    }

    impl MockOam {
        pub fn address_value(&self) -> u8 {
            self.mock_addr
        }

        pub fn set_address_value(&mut self, addr: u8) {
            self.mock_addr = addr;
        }

        pub fn data_value(&self) -> u8 {
            self.mock_data
        }

        pub fn set_data_value(&mut self, data: u8) {
            self.mock_data = data;
        }

        pub fn read_data_called(&self) -> bool {
            self.read_data_called.get()
        }

        pub fn read_data_increment_addr_called(&self) -> bool {
            self.read_data_increment_addr_called.get()
        }
    }

    impl ObjectAttributeMemory for MockOam {
        fn read_data(&self) -> u8 {
            self.read_data_called.set(true);
            self.data_value()
        }

        fn read_data_increment_addr(&self) -> u8 {
            self.read_data_increment_addr_called.set(true);
            self.data_value()
        }

        fn write_address(&mut self, addr: u8) {
            self.set_address_value(addr)
        }

        fn write_data(&mut self, val: u8) {
            self.set_data_value(val)
        }

        fn sprite_attributes(&self, _: u8) -> SpriteAttributes {
            unimplemented!()
        }
    }

    #[derive(Clone, Default)]
    pub struct MockVram {
        mock_addr: Cell<u8>,
        mock_data: u8,
    }

    impl MockVram {
        pub fn address_value(&self) -> u8 {
            self.mock_addr.get()
        }

        pub fn data_value(&self) -> u8 {
            self.mock_data
        }

        pub fn set_address_value(&self, addr: u8) {
            self.mock_addr.set(addr)
        }

        pub fn set_data_value(&mut self, data: u8) {
            self.mock_data = data;
        }
    }

    impl Vram for MockVram {
        fn write_address(&self, val: u8) {
            self.set_address_value(val)
        }

        fn read_data_increment_address(&self) -> Result<u8> {
            Ok(self.data_value())
        }

        fn read_data(&self) -> Result<u8> {
            Ok(self.data_value())
        }

        fn write_data_increment_address(&mut self, val: u8) -> Result<()> {
            self.set_data_value(val);
            Ok(())
        }

        fn clear_latch(&self) {}
    }
}
