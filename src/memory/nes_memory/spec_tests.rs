use self::mocks::new_fixture;
use memory::Memory;

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
        fixture.write(addr, 0xff, 0);
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
        fixture.write(addr, 0xff, 0);
        assert_eq!(addr, fixture.ppu.addr());
        assert_eq!(0xff, fixture.ppu.value());
    }
}

#[test]
fn apu_memory_mapped_read() {
    let mut fixture = new_fixture();
    fixture.apu.set_status(0xff);
    // Only a single APU address is readable
    let val = fixture.read(0x4015);
    assert_eq!(0xff, val);
}

#[test]
fn apu_memory_mapped_write() {
    let mut fixture = new_fixture();

    for addr in 0x4000..0x4014_u16 {
        fixture.write(addr, 0xff, 0);
        assert_eq!(addr, fixture.apu.write_addr());
        assert_eq!(0xff, fixture.apu.write_value());
    }
    // Skip 0x4014, since it's ppu DMA address

    fixture.write(0x4015, 0xff, 0);
    assert_eq!(0x4015, fixture.apu.write_addr());
    assert_eq!(0xff, fixture.apu.write_value());

    // Skip 0x4016 since it's input probe register

    for addr in 0x4017..0x4018_u16 {
        fixture.write(addr, 0xff, 0);
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
fn oam_dma_timing() {
    let mut fixture = new_fixture();
    let addl_cycles = fixture.write(0x4014, 0x02, 0);
    assert_eq!(513, addl_cycles);

    let addl_cycles = fixture.write(0x4014, 0x02, 1);
    assert_eq!(514, addl_cycles);
}

mod mocks {
    use apu::Apu;
    use cpu::Interrupt;

    use input::{Button, Input};
    use memory::nes_memory::NesMemoryBase;
    use ppu::Ppu;
    use rom::*;
    use screen::NesScreen;
    use std::io::Write;
    use std::rc::Rc;

    #[derive(Default)]
    pub struct InputMock;

    impl Input for InputMock {
        fn write(&mut self, _: u16, _: u8) {}

        fn read(&self, _: u16) -> u8 {
            0
        }

        fn player1_press(&self, _: Button) {
            unimplemented!()
        }

        fn player1_release(&self, _: Button) {
            unimplemented!()
        }
    }

    #[derive(Default)]
    pub struct ApuMock {
        write_addr: u16,
        write_value: u8,
        status: u8,
    }

    impl ApuMock {
        pub fn write_addr(&self) -> u16 {
            self.write_addr
        }

        pub fn write_value(&self) -> u8 {
            self.write_value
        }

        pub fn set_status(&mut self, val: u8) {
            self.status = val;
        }
    }

    impl Apu for ApuMock {
        fn write(&mut self, addr: u16, value: u8) {
            self.write_addr = addr;
            self.write_value = value;
        }

        fn read_status(&self) -> u8 {
            self.status
        }
        fn step(&mut self) -> Interrupt {
            Interrupt::None
        }
    }

    #[derive(Default)]
    pub struct PpuMock {
        addr: u16,
        value: u8,
        screen: NesScreen,
    }

    impl PpuMock {
        pub fn addr(&self) -> u16 {
            self.addr
        }

        pub fn value(&self) -> u8 {
            self.value
        }

        pub fn set_value(&mut self, value: u8) {
            self.value = value;
        }
    }

    impl Ppu for PpuMock {
        type Scr = NesScreen;

        fn write(&mut self, addr: u16, val: u8) {
            self.addr = addr;
            self.value = val;

        }

        fn read(&self, _: u16) -> u8 {
            self.value
        }

        fn step(&mut self) -> Interrupt {
            Interrupt::None
        }

        fn dump_registers<T: Write>(&self, _: &mut T) {
            unimplemented!()
        }

        fn new(_: Rc<Box<NesRom>>) -> Self {
            unimplemented!()
        }

        fn screen(&self) -> &NesScreen {
            &self.screen
        }
    }

    pub type NesMemoryFixture = NesMemoryBase<PpuMock, ApuMock, InputMock>;

    pub fn new_fixture() -> NesMemoryFixture {
        let rom = Rc::new(Box::new(NesRom {
                                       format: RomFormat::INes,
                                       video_standard: VideoStandard::Ntsc,
                                       mapper: 0,
                                       mirroring: Mirroring::Horizontal,
                                       prg_rom_banks: 1,
                                       prg_ram_banks: 1,
                                       chr_rom_banks: 1,
                                       has_sram: false,
                                       has_trainer: false,
                                       is_pc10: false,
                                       is_vs_unisystem: false,
                                       trainer: Vec::new(),
                                       chr: Vec::new(),
                                       prg: Vec::new(),
                                   }));

        NesMemoryBase {
            ram: [0_u8; 0x800],
            rom: rom,
            ppu: PpuMock::default(),
            apu: ApuMock::default(),
            input: InputMock::default(),
        }
    }
}
