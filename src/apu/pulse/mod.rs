use apu::envelope::Envelope;
use apu::length_counter::LengthCounter;
use apu::timer::Timer;

pub trait Pulse: Default {
    fn write_duty_and_envelope_reg(&mut self, val: u8);
    fn write_sweep_reg(&mut self, val: u8);
    fn write_timer_low_reg(&mut self, val: u8);
    fn write_length_load_timer_high_reg(&mut self, val: u8);
    fn clock_length_counter(&mut self);
    fn clock_envelope(&mut self);
    fn clock_timer(&mut self);
}

#[derive(Default)]
pub struct PulseImpl {
    sweep_reg: u8,
    timer_low_reg: u8,
    length_load_timer_high_reg: u8,
    duty: u8,
    timer: Timer,
    length_counter: LengthCounter,
    sequencer: PulseSequencer,
    envelope: Envelope,
}

impl Pulse for PulseImpl {
    fn write_duty_and_envelope_reg(&mut self, val: u8) {
        self.envelope.set_flags(val);
        self.length_counter.set_halt_flag(val & 0b_0010_0000 > 0);
        self.duty = (val & 0b_1100_0000) >> 6;
    }

    fn write_sweep_reg(&mut self, val: u8) {
        self.sweep_reg = val;
    }

    fn write_timer_low_reg(&mut self, val: u8) {
        self.timer_low_reg = val;
        let timer_period = self.timer_period();
        self.timer.set_period(timer_period)
    }

    fn write_length_load_timer_high_reg(&mut self, val: u8) {
        self.sequencer.reset();
        self.envelope.set_start_flag();
        self.length_load_timer_high_reg = val;
        let timer_period = self.timer_period();
        self.timer.set_period(timer_period)
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

impl PulseImpl {
    fn timer_period(&self) -> u16 {
        ((self.length_load_timer_high_reg as u16 & 0b111) << 8) | self.timer_low_reg as u16
    }

    fn length_counter_load(&self) -> u8 {
        (self.length_load_timer_high_reg & 0b_1111_1000) >> 3
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
