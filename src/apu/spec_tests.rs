use self::mocks::apu_mock;
use apu::ApuContract;

#[test]
fn memory_read_mapping() {
    let mut apu = apu_mock();
    apu.status.reg_4015 = 0xb0;
    let result = apu.read_status();
    assert_eq!(0xb0, result)
}

#[test]
fn memory_write_mapping() {
    let mut apu = apu_mock();
    apu.write(0x4000, 0xde);
    assert_eq!(0xde, apu.pulse_1.reg_4000_4004);

    let mut apu = apu_mock();
    apu.write(0x4001, 0xad);
    assert_eq!(0xad, apu.pulse_1.reg_4001_4005);

    let mut apu = apu_mock();
    apu.write(0x4002, 0xbe);
    assert_eq!(0xbe, apu.pulse_1.reg_4002_4006);

    let mut apu = apu_mock();
    apu.write(0x4003, 0xef);
    assert_eq!(0xef, apu.pulse_1.reg_4003_4007);

    apu.write(0x4004, 0xde);
    assert_eq!(0xde, apu.pulse_2.reg_4000_4004);

    let mut apu = apu_mock();
    apu.write(0x4005, 0xad);
    assert_eq!(0xad, apu.pulse_2.reg_4001_4005);

    let mut apu = apu_mock();
    apu.write(0x4006, 0xbe);
    assert_eq!(0xbe, apu.pulse_2.reg_4002_4006);

    let mut apu = apu_mock();
    apu.write(0x4007, 0xef);
    assert_eq!(0xef, apu.pulse_2.reg_4003_4007);

    let mut apu = apu_mock();
    apu.write(0x4008, 0xde);
    assert_eq!(0xde, apu.triangle.reg_4008);

    let mut apu = apu_mock();
    apu.write(0x400a, 0xad);
    assert_eq!(0xad, apu.triangle.reg_400a);

    let mut apu = apu_mock();
    apu.write(0x400b, 0xbe);
    assert_eq!(0xbe, apu.triangle.reg_400b);

    let mut apu = apu_mock();
    apu.write(0x400c, 0xef);
    assert_eq!(0xef, apu.noise.reg_400c);

    let mut apu = apu_mock();
    apu.write(0x400e, 0xde);
    assert_eq!(0xde, apu.noise.reg_400e);

    let mut apu = apu_mock();
    apu.write(0x400f, 0xad);
    assert_eq!(0xad, apu.noise.reg_400f);
}

mod mocks {
    use apu::ApuImpl;
    use apu::frame_sequencer::FrameSequencer;
    use apu::noise::Noise;
    use apu::pulse::Pulse;
    use apu::status::Status;
    use apu::triangle::Triangle;

    pub type ApuMock = ApuImpl<PulseMock, TriangleMock, NoiseMock, StatusMock, FrameSequencerMock>;

    pub fn apu_mock() -> ApuMock {
        ApuImpl::default()
    }

    #[derive(Default)]
    pub struct FrameSequencerMock {
        pub reg_4017: u8,
    }

    impl FrameSequencer for FrameSequencerMock {
        fn write(&mut self, val: u8) {
            self.reg_4017 = val;
        }
    }

    #[derive(Default)]
    pub struct TriangleMock {
        pub reg_4008: u8,
        pub reg_400a: u8,
        pub reg_400b: u8,
    }

    impl Triangle for TriangleMock {
        fn write_linear_counter_reg(&mut self, val: u8) {
            self.reg_4008 = val;
        }

        fn write_counter_load_timer_high_reg(&mut self, val: u8) {
            self.reg_400b = val;
        }

        fn write_timer_low_reg(&mut self, val: u8) {
            self.reg_400a = val;
        }
    }

    #[derive(Default)]
    pub struct PulseMock {
        pub reg_4000_4004: u8,
        pub reg_4001_4005: u8,
        pub reg_4002_4006: u8,
        pub reg_4003_4007: u8,
    }

    impl Pulse for PulseMock {
        fn write_duty_etc_reg(&mut self, val: u8) {
            self.reg_4000_4004 = val
        }

        fn write_sweep_reg(&mut self, val: u8) {
            self.reg_4001_4005 = val
        }

        fn write_timer_low_reg(&mut self, val: u8) {
            self.reg_4002_4006 = val
        }

        fn write_counter_load_timer_high_reg(&mut self, val: u8) {
            self.reg_4003_4007 = val;
        }
    }

    #[derive(Default)]
    pub struct NoiseMock {
        pub reg_400c: u8,
        pub reg_400e: u8,
        pub reg_400f: u8,
    }

    impl Noise for NoiseMock {
        fn write_counter_halt_etc_reg(&mut self, val: u8) {
            self.reg_400c = val;
        }

        fn write_mode_and_period_reg(&mut self, val: u8) {
            self.reg_400e = val;
        }

        fn write_counter_load_and_envelope_restart(&mut self, val: u8) {
            self.reg_400f = val;
        }
    }

    #[derive(Default)]
    pub struct StatusMock {
        pub reg_4015: u8,
    }

    impl Status for StatusMock {
        fn read(&self) -> u8 {
            self.reg_4015
        }

        fn write(&mut self, val: u8) {
            self.reg_4015 = val
        }
    }
}
