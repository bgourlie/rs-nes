use self::sequencer::Sequencer;
use apu::envelope::Envelope;
use apu::length_counter::LengthCounter;
use apu::timer::Timer;

pub trait Pulse: Default {
    fn write_4000_4004(&mut self, val: u8);
    fn write_4001_4005(&mut self, val: u8);
    fn write_4002_4006(&mut self, val: u8);
    fn write_4003_4007(&mut self, val: u8);
    fn clock_length_counter(&mut self);
    fn clock_envelope(&mut self);
    fn clock_timer(&mut self);
}

#[derive(Default)]
pub struct PulseImpl {
    sweep_reg: u8,
    duty: u8,
    timer: Timer,
    length_counter: LengthCounter,
    sequencer: Sequencer,
    envelope: Envelope,
}

impl PulseImpl {
    fn set_timer_period_low(&mut self, val: u8) {
        let timer_period = (self.timer.period() & 0b_0111_0000_0000) | val as u16;
        self.timer.set_period(timer_period)
    }

    fn set_timer_period_high(&mut self, val: u8) {
        let timer_period = (self.timer.period() & 0b_1111_1111) | ((val as u16 & 0b111) << 8);
        self.timer.set_period(timer_period);
    }
}

impl Pulse for PulseImpl {
    fn write_4000_4004(&mut self, val: u8) {
        self.envelope.set_flags(val);
        self.length_counter.set_halt_flag(val & 0b_0010_0000 > 0);
        self.duty = (val & 0b_1100_0000) >> 6;

        // TODO: Side-effects
        // The duty cycle is changed (see table on nesdev), but the sequencer's current position
        // isn't affected.
    }

    fn write_4001_4005(&mut self, val: u8) {
        self.sweep_reg = val;
    }

    fn write_4002_4006(&mut self, val: u8) {
        self.set_timer_period_low(val);
    }

    fn write_4003_4007(&mut self, val: u8) {
        self.set_timer_period_high(val);
        self.length_counter.load((val & 0b_1111_1000) >> 3);

        // Side-effects:
        // The sequencer is immediately restarted at the first value of the current sequence. The
        // envelope is also restarted. The period divider is not reset.
        self.sequencer.reset();
        self.envelope.set_start_flag();
    }

    fn clock_timer(&mut self) {
        if self.timer.clock() {
            self.sequencer.clock();
        }
    }

    fn clock_length_counter(&mut self) {
        self.length_counter.clock()
    }

    fn clock_envelope(&mut self) {
        self.envelope.clock()
    }
}

mod sequencer {
    #[derive(Default)]
    pub struct Sequencer {
        step: u8,
    }

    impl Sequencer {
        pub fn reset(&mut self) {
            self.step = 0
        }

        pub fn clock(&mut self) {
            if self.step == 0 {
                self.step = 7;
            } else {
                self.step -= 1;
            }
        }
    }
}
