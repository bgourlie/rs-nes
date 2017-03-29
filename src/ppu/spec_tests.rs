use super::*;
use super::status_register::StatusRegister;
use super::write_latch::LatchState;

#[test]
fn write() {
    let mut ppu = mocks::mock_ppu();

    // Writes to 0x2000 write the control register
    ppu.write(0x2000, 0x1).unwrap();
    assert_eq!(0x1, *ppu.control);

    // Writes to 0x2001 write the mask register
    ppu.write(0x2001, 0x2).unwrap();
    assert_eq!(0x2, *ppu.mask);

    // Writes to 0x2003 write the oam addr register
    ppu.write(0x2003, 0x3).unwrap();
    assert_eq!(0x3, ppu.oam.mock_addr.get());

    // Writes to 0x2004 write the oam data register
    ppu.write(0x2004, 0x4).unwrap();
    assert_eq!(0x4, ppu.oam.mock_data.get());

    // Writes to 0x2005 write the scroll register
    ppu.write(0x2005, 0x5).unwrap();
    assert_eq!(true, ppu.vram.scroll_write_called.get());
    ppu.vram.reset_mock();

    // Writes to 0x2006 write the vram addr register
    ppu.write(0x2006, 0x20).unwrap();
    assert_eq!(0x20, ppu.vram.mock_addr.get());

    // Writes to 0x2007 write the vram data register
    ppu.write(0x2007, 0x7).unwrap();
    assert_eq!(0x7, ppu.vram.mock_data.get());

    // Test mirroring: 0x2000-0x2007 are mirrored every 8 bytes to 0x3fff

    ppu.write(0x2008, 0x8).unwrap();
    assert_eq!(0x8, *ppu.control);

    ppu.write(0x2009, 0x9).unwrap();
    assert_eq!(0x9, *ppu.mask);

    ppu.write(0x200b, 0xa).unwrap();
    assert_eq!(0xa, ppu.oam.mock_addr.get());

    ppu.write(0x200c, 0xb).unwrap();
    assert_eq!(0xb, ppu.oam.mock_data.get());

    ppu.write(0x200d, 0xc).unwrap();
    assert_eq!(true, ppu.vram.scroll_write_called.get());
    ppu.vram.reset_mock();

    ppu.write(0x200e, 0x01).unwrap();
    assert_eq!(0x01, ppu.vram.mock_addr.get());

    ppu.write(0x200f, 0x14).unwrap();
    assert_eq!(0x14, ppu.vram.mock_data.get());

    // Test mirroring on the tail end of the valid address space

    ppu.write(0x3ff8, 0xf).unwrap();
    assert_eq!(0xf, *ppu.control);

    ppu.write(0x3ff9, 0x10).unwrap();
    assert_eq!(0x10, *ppu.mask);

    ppu.write(0x3ffb, 0x11).unwrap();
    assert_eq!(0x11, ppu.oam.mock_addr.get());

    ppu.write(0x3ffc, 0x12).unwrap();
    assert_eq!(0x12, ppu.oam.mock_data.get());

    ppu.write(0x3ffd, 0x13).unwrap();
    assert_eq!(true, ppu.vram.scroll_write_called.get());
    ppu.vram.reset_mock();

    ppu.write(0x3ffe, 0x02).unwrap();
    assert_eq!(0x02, ppu.vram.mock_addr.get());

    ppu.write(0x3fff, 0x15).unwrap();
    assert_eq!(0x15, ppu.vram.mock_data.get());
}

#[test]
fn memory_mapped_register_read() {
    let mut ppu = mocks::mock_ppu();

    ppu.control.write(0xf0);
    assert_eq!(0xf0, ppu.read(0x2000).unwrap());

    ppu.mask.write(0xf1);
    assert_eq!(0xf1, ppu.read(0x2001).unwrap());

    ppu.status = StatusRegister::new(0xf2);
    assert_eq!(0xf2, ppu.read(0x2002).unwrap());

    ppu.oam.mock_addr.set(0xf3);
    assert_eq!(0, ppu.read(0x2003).unwrap()); // write-only, should always read 0

    ppu.oam.mock_data.set(0xf4);
    assert_eq!(0xf4, ppu.read(0x2004).unwrap());

    assert_eq!(0x0, ppu.read(0x2005).unwrap()); // write-only, should always read 0

    ppu.vram.mock_addr.set(0xf6);
    assert_eq!(0x0, ppu.read(0x2006).unwrap()); // write-only, should always read 0

    ppu.vram.mock_data.set(0xfe);
    assert_eq!(0xfe, ppu.read(0x2007).unwrap());

    // Test mirroring: 0x2000-0x2007 are mirrored every 8 bytes to 0x3fff

    ppu.control.write(0xe0);
    assert_eq!(0xe0, ppu.read(0x2008).unwrap());

    ppu.mask.write(0xe1);
    assert_eq!(0xe1, ppu.read(0x2009).unwrap());

    ppu.status = StatusRegister::new(0xe2);
    assert_eq!(0xe2, ppu.read(0x200a).unwrap());

    ppu.oam.mock_addr.set(0xe3);
    assert_eq!(0, ppu.read(0x200b).unwrap()); // write-only, should always read 0

    ppu.oam.mock_data.set(0xe4);
    assert_eq!(0xe4, ppu.read(0x200c).unwrap());

    assert_eq!(0x0, ppu.read(0x200d).unwrap()); // write-only, should always read 0

    ppu.vram.mock_addr.set(0xe6);
    assert_eq!(0x0, ppu.read(0x200e).unwrap()); // write-only, should always read 0

    ppu.vram.mock_data.set(0xfb);
    assert_eq!(0xfb, ppu.read(0x200f).unwrap());

    // Test mirroring on the tail end of the valid address space

    ppu.control.write(0xd0);
    assert_eq!(0xd0, ppu.read(0x3ff8).unwrap());

    ppu.mask.write(0xd1);
    assert_eq!(0xd1, ppu.read(0x3ff9).unwrap());

    ppu.status = StatusRegister::new(0xd2);
    assert_eq!(0xd2, ppu.read(0x3ffa).unwrap());

    ppu.oam.mock_addr.set(0xd3);
    assert_eq!(0, ppu.read(0x3ffb).unwrap()); // write-only, should always read 0

    ppu.oam.mock_data.set(0xd4);
    assert_eq!(0xd4, ppu.read(0x3ffc).unwrap());

    assert_eq!(0x0, ppu.read(0x3ffd).unwrap()); // write-only, should always read 0

    ppu.vram.mock_addr.set(0xd6);
    assert_eq!(0x0, ppu.read(0x3ffe).unwrap()); // write-only, should always read 0

    ppu.vram.mock_data.set(0xfc);
    assert_eq!(0xfc, ppu.read(0x3fff).unwrap());
}

#[test]
fn increment_coarse_x_called() {
    // Between dot 328 of a scanline, and 256 of the next scanline, if rendering is enabled, the PPU
    // increments the horizontal position in v many times across the scanline, it begins at dots 328
    // and 336, and will continue through the next scanline at 8, 16, 24... 240, 248, 256 (every 8
    // dots across the scanline until 256). The effective X scroll coordinate is incremented, which
    // will wrap to the next nametable appropriately. See Wrapping around below.

    let mut ppu = mocks::mock_ppu();
    ppu.mask.write(0b00011000); // Enable rendering
    // Render 5 frames and assert that the VRAM coarse x increment function is called
    while ppu.cycles < super::CYCLES_PER_FRAME * 5 {
        let frame_cycle = ppu.cycles % super::CYCLES_PER_FRAME;
        let scanline = frame_cycle / CYCLES_PER_SCANLINE;
        let x = frame_cycle % super::CYCLES_PER_SCANLINE;
        ppu.step().unwrap();
        if (scanline < 240 || scanline == 261) && ((x > 0 && x < 256) || x >= 328) && x % 8 == 0 {
            assert_eq!(true, ppu.vram.coarse_x_increment_called.get())
        } else {
            assert_eq!(false, ppu.vram.coarse_x_increment_called.get())
        }
        ppu.vram.reset_mock();
    }

    // Verify not called if rendering is disabled

    let mut ppu = mocks::mock_ppu();
    ppu.mask.write(0b00000000); // Disable rendering
    while ppu.cycles < super::CYCLES_PER_FRAME * 5 {
        ppu.step().unwrap();
        assert_eq!(false, ppu.vram.coarse_x_increment_called.get())
    }
}

#[test]
fn copy_horizontal_pos_to_addr_called() {
    // At dot 257 of each scanline, if rendering is enabled, VRAM copy_horizontal_pos_to_addr()
    // should be called
    let mut ppu = mocks::mock_ppu();
    ppu.mask.write(0b00011000); // Enable rendering
    while ppu.cycles < super::CYCLES_PER_FRAME * 5 {
        let frame_cycle = ppu.cycles % super::CYCLES_PER_FRAME;
        let scanline = frame_cycle / CYCLES_PER_SCANLINE;
        let x = frame_cycle % super::CYCLES_PER_SCANLINE;
        ppu.step().unwrap();
        if (scanline < 240 || scanline == 261) && x == 257 {
            assert_eq!(true, ppu.vram.copy_horizontal_pos_to_addr_called.get())
        } else {
            assert_eq!(false, ppu.vram.copy_horizontal_pos_to_addr_called.get())
        }
        ppu.vram.reset_mock();
    }

    // Verify not called if rendering is disabled

    let mut ppu = mocks::mock_ppu();
    ppu.mask.write(0b00000000); //
    while ppu.cycles < super::CYCLES_PER_FRAME * 5 {
        ppu.step().unwrap();
        assert_eq!(false, ppu.vram.copy_horizontal_pos_to_addr_called.get())
    }
}

#[test]
fn copy_vertical_pos_to_addr_called() {
    // During dots 280 to 304 of the pre-render scanline (end of vblank), if rendering is enabled,
    // vram copy_vertical_pos_addr should be called
    let mut ppu = mocks::mock_ppu();
    ppu.mask.write(0b00011000); // Enable rendering
    while ppu.cycles < super::CYCLES_PER_FRAME * 5 {
        let frame_cycle = ppu.cycles % super::CYCLES_PER_FRAME;
        let scanline = frame_cycle / CYCLES_PER_SCANLINE;
        let x = frame_cycle % super::CYCLES_PER_SCANLINE;
        ppu.step().unwrap();
        if scanline == 261 && x >= 280 && x <= 304 {
            assert_eq!(true, ppu.vram.copy_vertical_pos_to_addr_called.get())
        } else {
            assert_eq!(false, ppu.vram.copy_vertical_pos_to_addr_called.get())
        }
        ppu.vram.reset_mock();
    }

    // Verify not called if rendering is disabled

    let mut ppu = mocks::mock_ppu();
    ppu.mask.write(0b00000000); //
    while ppu.cycles < super::CYCLES_PER_FRAME * 5 {
        ppu.step().unwrap();
        assert_eq!(false, ppu.vram.copy_vertical_pos_to_addr_called.get())
    }
}

#[test]
fn increment_fine_y_called() {
    // If rendering is enabled, VRAM increment_find_y should be called at dot 256 of each scanline
    let mut ppu = mocks::mock_ppu();
    ppu.mask.write(0b00011000); // Enable rendering
    while ppu.cycles < super::CYCLES_PER_FRAME * 5 {
        let frame_cycle = ppu.cycles % super::CYCLES_PER_FRAME;
        let scanline = frame_cycle / CYCLES_PER_SCANLINE;
        let x = frame_cycle % super::CYCLES_PER_SCANLINE;
        ppu.step().unwrap();
        if (scanline < 240 || scanline == 261) && x == 256 {
            assert_eq!(true, ppu.vram.fine_y_increment_called.get())
        } else {
            assert_eq!(false, ppu.vram.fine_y_increment_called.get())
        }
        ppu.vram.reset_mock();
    }

    // Verify not called if rendering is disabled

    let mut ppu = mocks::mock_ppu();
    ppu.mask.write(0b00000000); // Disable rendering
    while ppu.cycles < super::CYCLES_PER_FRAME * 5 {
        ppu.step().unwrap();
        assert_eq!(false, ppu.vram.fine_y_increment_called.get())
    }
}

#[test]
#[cfg(feature = "slow_tests")]
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

    let mut ppu = mocks::mock_ppu();

    while ppu.cycles < super::CYCLES_PER_FRAME * 5 {
        match ppu.cycles % super::CYCLES_PER_FRAME {
            0...VBLANK_OFF => assert_eq!(false, ppu.status.in_vblank()),
            VBLANK_ON...CLEAR_VBLANK_CYCLE => assert_eq!(true, ppu.status.in_vblank()),
            VBLANK_OFF_AGAIN...super::CYCLES_PER_FRAME => assert_eq!(false, ppu.status.in_vblank()),
            _ => panic!("We should never get here"),
        }
        ppu.step().unwrap();
    }
}

#[test]
fn vblank_clear_after_status_read() {
    let ppu = mocks::mock_ppu();
    ppu.status.set_in_vblank();
    let status = ppu.read(0x2002).unwrap();
    assert_eq!(true, status & 0b10000000 > 0);
    assert_eq!(true, ppu.status.read() & 0b10000000 == 0);
}

#[test]
fn oam_read_non_blanking_increments_addr() {
    let mut ppu = mocks::mock_ppu();
    ppu.status.clear_in_vblank();
    ppu.mask.write(0xff); // Enable rendering
    ppu.read(0x2004).unwrap();
    assert_eq!(true, ppu.oam.read_data_increment_addr_called.get());
    assert_eq!(false, ppu.oam.read_data_called.get());
}

#[test]
fn oam_read_v_blanking_doesnt_increments_addr() {
    let mut ppu = mocks::mock_ppu();
    ppu.status.set_in_vblank();
    ppu.mask.write(0xff); // Enable rendering
    ppu.read(0x2004).unwrap();
    assert_eq!(false, ppu.oam.read_data_increment_addr_called.get());
    assert_eq!(true, ppu.oam.read_data_called.get());
}

#[test]
fn oam_read_forced_blanking_doesnt_increments_addr() {
    let mut ppu = mocks::mock_ppu();
    ppu.status.clear_in_vblank();
    ppu.mask.write(0);
    ppu.read(0x2004).unwrap();
    assert_eq!(false, ppu.oam.read_data_increment_addr_called.get());
    assert_eq!(true, ppu.oam.read_data_called.get());
}

#[test]
fn odd_frame_cycle_skip() {
    let mut ppu = mocks::mock_ppu();
    ppu.mask.write(0b00001000); // Enable background rendering
    while ppu.cycles < super::CYCLES_PER_FRAME * 10 {
        let frame_cycle = ppu.cycles % super::CYCLES_PER_FRAME;
        let scanline = frame_cycle / CYCLES_PER_SCANLINE;
        let x = frame_cycle % super::CYCLES_PER_SCANLINE;
        let frame_number = ppu.cycles / super::CYCLES_PER_FRAME;
        let was_odd_frame = frame_number % 2 == 1;
        ppu.step().unwrap();

        if scanline == 261 && x == 339 {
            let frame_cycle = ppu.cycles % super::CYCLES_PER_FRAME;
            let new_scanline = frame_cycle / CYCLES_PER_SCANLINE;
            let new_x = frame_cycle % super::CYCLES_PER_SCANLINE;
            if was_odd_frame {
                assert_eq!(0, new_scanline);
                assert_eq!(0, new_x);
            } else {
                assert_eq!(261, new_scanline);
                assert_eq!(340, new_x);
            }
        }
    }

    // Verify no skipped frame if background rendering is disabled
    // TODO: Verify that this is the correct behavior
    let mut ppu = mocks::mock_ppu();
    ppu.mask.write(0b00000000); // Disable rendering
    while ppu.cycles < super::CYCLES_PER_FRAME * 10 {
        let frame_cycle = ppu.cycles % super::CYCLES_PER_FRAME;
        let scanline = frame_cycle / CYCLES_PER_SCANLINE;
        let x = frame_cycle % super::CYCLES_PER_SCANLINE;
        ppu.step().unwrap();

        if scanline == 261 && x == 339 {
            let frame_cycle = ppu.cycles % super::CYCLES_PER_FRAME;
            let new_scanline = frame_cycle / CYCLES_PER_SCANLINE;
            let new_x = frame_cycle % super::CYCLES_PER_SCANLINE;
            assert_eq!(261, new_scanline);
            assert_eq!(340, new_x);
        }
    }
}

mod mocks {
    use super::object_attribute_memory::SpriteAttributes;
    use errors::*;
    use ppu::PpuBase;
    use ppu::background_renderer::BackgroundRenderer;
    use ppu::control_register::{ControlRegister, IncrementAmount};
    use ppu::mask_register::MaskRegister;
    use ppu::object_attribute_memory::ObjectAttributeMemory;
    use ppu::status_register::StatusRegister;
    use ppu::vram::Vram;
    use ppu::write_latch::{LatchState, WriteLatch};
    use rom::NesRom;
    use screen::{Color, NesScreen};
    use std::cell::Cell;
    use std::cell::RefCell;
    use std::rc::Rc;

    pub type TestPpu = PpuBase<MockVram, MockOam>;

    pub fn mock_ppu() -> TestPpu {
        let empty: [Color; 16] = [Color(0x00, 0x00, 0x00),
                                  Color(0x00, 0x00, 0x00),
                                  Color(0x00, 0x00, 0x00),
                                  Color(0x00, 0x00, 0x00),
                                  Color(0x00, 0x00, 0x00),
                                  Color(0x00, 0x00, 0x00),
                                  Color(0x00, 0x00, 0x00),
                                  Color(0x00, 0x00, 0x00),
                                  Color(0x00, 0x00, 0x00),
                                  Color(0x00, 0x00, 0x00),
                                  Color(0x00, 0x00, 0x00),
                                  Color(0x00, 0x00, 0x00),
                                  Color(0x00, 0x00, 0x00),
                                  Color(0x00, 0x00, 0x00),
                                  Color(0x00, 0x00, 0x00),
                                  Color(0x00, 0x00, 0x00)];

        PpuBase {
            cycles: 0,
            control: ControlRegister::default(),
            mask: MaskRegister::default(),
            status: StatusRegister::default(),
            vram: MockVram::new(NesRom::default()),
            oam: MockOam::default(),
            screen: Rc::new(RefCell::new(NesScreen::default())),
            sprite_palettes: empty,
            bg_palettes: empty,
            write_latch: WriteLatch::default(),
            sprite_buffer: [None, None, None, None, None, None, None, None],
            background_renderer: BackgroundRenderer::default(),
        }
    }

    #[derive(Default)]
    pub struct MockOam {
        pub read_data_called: Cell<bool>,
        pub read_data_increment_addr_called: Cell<bool>,
        pub mock_addr: Cell<u8>,
        pub mock_data: Cell<u8>,
    }

    impl ObjectAttributeMemory for MockOam {
        fn read_data(&self) -> u8 {
            self.read_data_called.set(true);
            self.mock_data.get()
        }

        fn read_data_increment_addr(&self) -> u8 {
            self.read_data_increment_addr_called.set(true);
            self.mock_data.get()
        }

        fn write_address(&mut self, addr: u8) {
            self.mock_addr.set(addr)
        }

        fn write_data(&mut self, val: u8) {
            self.mock_data.set(val)
        }

        fn sprite_attributes(&self, _: u8) -> SpriteAttributes {
            SpriteAttributes::default()
        }
    }

    #[derive(Default)]
    pub struct MockVram {
        pub mock_addr: Cell<u8>,
        pub mock_data: Cell<u8>,
        pub scroll_write_called: Cell<bool>,
        pub control_write_called: Cell<bool>,
        pub coarse_x_increment_called: Cell<bool>,
        pub fine_y_increment_called: Cell<bool>,
        pub copy_horizontal_pos_to_addr_called: Cell<bool>,
        pub copy_vertical_pos_to_addr_called: Cell<bool>,
    }

    impl MockVram {
        pub fn reset_mock(&self) {
            self.mock_addr.set(0);
            self.mock_data.set(0);
            self.scroll_write_called.set(false);
            self.control_write_called.set(false);
            self.coarse_x_increment_called.set(false);
            self.fine_y_increment_called.set(false);
            self.copy_horizontal_pos_to_addr_called.set(false);
            self.copy_vertical_pos_to_addr_called.set(false);
        }
    }

    impl Vram for MockVram {
        fn write_ppu_addr(&self, latch_state: LatchState) {
            let val = match latch_state {
                LatchState::FirstWrite(val) => val,
                LatchState::SecondWrite(val) => val,
            };

            self.mock_addr.set(val)
        }

        fn read_ppu_data(&self, _: IncrementAmount) -> Result<u8> {
            Ok(self.mock_data.get())
        }

        fn ppu_data(&self) -> Result<u8> {
            Ok(self.mock_data.get())
        }

        fn write_ppu_data(&mut self, val: u8, _: IncrementAmount) -> Result<()> {
            self.mock_data.set(val);;
            Ok(())
        }

        fn read(&self, _: u16) -> Result<u8> {
            Ok(0)
        }

        fn new(_: NesRom) -> Self {
            Self::default()
        }

        fn addr(&self) -> u16 {
            0
        }

        fn scroll_write(&self, _: LatchState) {
            self.scroll_write_called.set(true)
        }

        fn control_write(&self, _: u8) {
            self.control_write_called.set(true)
        }

        fn coarse_x_increment(&self) {
            self.coarse_x_increment_called.set(true)
        }

        fn fine_y_increment(&self) {
            self.fine_y_increment_called.set(true)
        }

        fn copy_horizontal_pos_to_addr(&self) {
            self.copy_horizontal_pos_to_addr_called.set(true)

        }
        fn copy_vertical_pos_to_addr(&self) {
            self.copy_vertical_pos_to_addr_called.set(true)
        }
        fn fine_x(&self) -> u8 {
            0
        }
    }
}
