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
        let val = fixture.read(addr).unwrap();
        assert_eq!(expect, val);
    }
}

#[test]
fn ram_memory_mapped_write() {
    let mut fixture = new_fixture();
    for addr in 0..0x2000_u16 {
        let ram_index = (addr & 0x7ff) as usize;
        fixture.write(addr, 0xff).unwrap();
        assert_eq!(0xff, fixture.ram[ram_index]);
        fixture.ram[ram_index] = 0; // reset it after asserting it was written correctly
    }
}

#[test]
fn ppu_memory_mapped_read() {
    let mut fixture = new_fixture();
    fixture.ppu.set_value(0xff);
    for addr in 0x2000..0x2008_u16 {
        let val = fixture.read(addr).unwrap();
        assert_eq!(0xff, val);
    }
}

#[test]
fn ppu_memory_mapped_write() {
    let mut fixture = new_fixture();
    for addr in 0x2000..0x2008_u16 {
        fixture.write(addr, 0xff).unwrap();
        assert_eq!(addr, fixture.ppu.addr());
        assert_eq!(0xff, fixture.ppu.value());
    }
}

#[test]
fn apu_memory_mapped_read() {
    let mut fixture = new_fixture();
    fixture.apu.set_control(0xff);
    // Only a single APU address is readable
    let val = fixture.read(0x4015).unwrap();
    assert_eq!(0xff, val);
}

#[test]
fn apu_memory_mapped_write() {
    let mut fixture = new_fixture();

    for addr in 0x4000..0x4016_u16 {
        fixture.write(addr, 0xff).unwrap();
        assert_eq!(addr, fixture.apu.write_addr());
        assert_eq!(0xff, fixture.apu.write_value());
    }

    for addr in 0x4017..0x4018_u16 {
        fixture.write(addr, 0xff).unwrap();
        assert_eq!(addr, fixture.apu.write_addr());
        assert_eq!(0xff, fixture.apu.write_value());
    }
}

#[test]
fn input_memory_mapped_read() {
    let mut fixture = new_fixture();
    fixture.input.set_joy1(0xfe);
    fixture.input.set_joy2(0xff);
    let joy1_val = fixture.read(0x4016).unwrap();
    let joy2_val = fixture.read(0x4017).unwrap();
    assert_eq!(0xfe, joy1_val);
    assert_eq!(0xff, joy2_val);
}

#[test]
fn input_memory_mapped_write() {
    let mut fixture = new_fixture();
    fixture.write(0x4016, 0xff).unwrap();
    assert_eq!(0xff, fixture.input.probe());
}

mod mocks {
    use apu::Apu;
    use errors::*;
    use input::Input;
    use memory::nes_memory::{NesMemoryBase, StepAction};
    use ppu::Ppu;
    use rom::*;
    use std::io::Write;

    #[derive(Copy, Clone, Default)]
    pub struct InputMock {
        probe: u8,
        joy1: u8,
        joy2: u8,
    }

    impl InputMock {
        pub fn probe(&self) -> u8 {
            self.probe
        }

        pub fn set_joy1(&mut self, val: u8) {
            self.joy1 = val;
        }

        pub fn set_joy2(&mut self, val: u8) {
            self.joy2 = val;
        }
    }

    impl Input for InputMock {
        fn write_probe(&mut self, val: u8) {
            self.probe = val;
        }

        fn read_joy_1(&self) -> u8 {
            self.joy1
        }

        fn read_joy_2(&self) -> u8 {
            self.joy2
        }
    }

    #[derive(Copy, Clone, Default)]
    pub struct ApuMock {
        write_addr: u16,
        write_value: u8,
        control: u8,
    }

    impl ApuMock {
        pub fn write_addr(&self) -> u16 {
            self.write_addr
        }

        pub fn write_value(&self) -> u8 {
            self.write_value
        }

        pub fn set_control(&mut self, val: u8) {
            self.control = val;
        }
    }

    impl Apu for ApuMock {
        fn write(&mut self, addr: u16, value: u8) {
            self.write_addr = addr;
            self.write_value = value;
        }

        fn read_control(&self) -> u8 {
            self.control
        }
    }

    #[derive(Copy, Clone, Default)]
    pub struct PpuMock {
        addr: u16,
        value: u8,
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
        fn write(&mut self, addr: u16, val: u8) -> Result<()> {
            self.addr = addr;
            self.value = val;
            Ok(())
        }

        fn read(&self, _: u16) -> Result<u8> {
            Ok(self.value)
        }

        fn step(&mut self) -> StepAction {
            unimplemented!()
        }

        fn dump_registers<T: Write>(&self, _: &mut T) {
            unimplemented!()
        }
    }

    pub type NesMemoryFixture = NesMemoryBase<PpuMock, ApuMock, InputMock>;

    pub fn new_fixture() -> NesMemoryFixture {
        let rom = NesRom {
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
        };

        NesMemoryBase {
            ram: [0_u8; 0x800],
            rom: rom,
            ppu: PpuMock::default(),
            apu: ApuMock::default(),
            input: InputMock::default(),
        }
    }
}
