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
    timer_period: u16,
    duty: u8,
    timer: Timer,
    length_counter: LengthCounter,
    sequencer: PulseSequencer,
    envelope: Envelope,
}

impl Pulse for PulseImpl {
    fn write_4000_4004(&mut self, val: u8) {
        self.envelope.set_flags(val);
        self.length_counter.set_halt_flag(val & 0b_0010_0000 > 0);
        self.duty = (val & 0b_1100_0000) >> 6;
    }

    fn write_4001_4005(&mut self, val: u8) {
        self.sweep_reg = val;
    }

    fn write_4002_4006(&mut self, val: u8) {
        self.timer_period = (self.timer_period & 0b_0111_0000_0000) | val as u16;
        self.timer.set_period(self.timer_period)
    }

    fn write_4003_4007(&mut self, val: u8) {
        self.sequencer.reset();
        self.envelope.set_start_flag();
        self.timer_period = (self.timer_period & 0b_1111_1111) | ((val as u16 & 0b111) << 8);
        self.timer.set_period(self.timer_period);
        self.length_counter.load((val & 0b_1111_1000) >> 3);
    }

    fn clock_timer(&mut self) {
        let mut clock_sequencer = false;
        self.timer.clock(|| { clock_sequencer = true; });

        if clock_sequencer {
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

#[derive(Default)]
struct PulseSequencer {
    step: u8,
}

impl PulseSequencer {
    fn reset(&mut self) {
        self.step = 0
    }

    fn clock(&mut self) {
        if self.step == 0 {
            self.step = 7;
        } else {
            self.step -= 1;
        }
    }
}
