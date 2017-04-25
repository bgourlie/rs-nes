use self::sequencer::Sequencer;
use apu::envelope::Envelope;
use apu::length_counter::LengthCounter;
use apu::sweep::Sweep;
use apu::timer::Timer;
use std::marker::PhantomData;

const CPU_FREQ: f32 = 1.789773;

pub type Pulse1 = PulseImpl<Pulse1Negater>;
pub type Pulse2 = PulseImpl<Pulse1Negater>;

pub trait Pulse: Default {
    fn write_4000_4004(&mut self, val: u8);
    fn write_4001_4005(&mut self, val: u8);
    fn write_4002_4006(&mut self, val: u8);
    fn write_4003_4007(&mut self, val: u8);
    fn clock_envelope(&mut self);
    fn clock_timer(&mut self);
    fn clock_length_counter(&mut self);
    fn clock_sweep(&mut self);
    fn zero_length_counter(&mut self);
    fn length_is_nonzero(&self) -> bool;
    fn output(&self) -> (f32, u8);
}

/// Trait for implementing negation logic used during sweep adjustment, which differs between pulse
/// units.
///
/// The two pulse channels have their adders' carry inputs wired differently, which produces
/// different results when each channel's change amount is made negative:
///
///   - Pulse 1 adds the ones' complement (−c − 1). Making 20 negative produces a change
///     amount of −21.
///
///   - Pulse 2 adds the two's complement (−c). Making 20 negative produces a change amount
///     of -20.
pub trait Negater: Default {
    fn negate_amount(val: u16) -> u16;
}

#[derive(Default)]
pub struct Pulse1Negater;

#[derive(Default)]
pub struct Pulse2Negater;

impl Negater for Pulse1Negater {
    fn negate_amount(val: u16) -> u16 {
        val + 1
    }
}

impl Negater for Pulse2Negater {
    fn negate_amount(val: u16) -> u16 {
        val
    }
}

#[derive(Default)]
pub struct PulseImpl<N: Negater> {
    sweep: Sweep,
    duty_cycle: bool,
    timer: Timer,
    length_counter: LengthCounter,
    sequencer: Sequencer,
    envelope: Envelope,
    raw_timer_period: u16,
    phantom: PhantomData<N>,
}

impl<N: Negater> PulseImpl<N> {
    fn set_raw_timer_period_low(&mut self, val: u8) {
        self.raw_timer_period = (self.raw_timer_period & 0b_0111_0000_0000) | val as u16;
        self.update_timer_period()
    }

    fn set_raw_timer_period_high(&mut self, val: u8) {
        self.raw_timer_period = (self.raw_timer_period & 0b_1111_1111) |
                                ((val as u16 & 0b111) << 8);
        self.update_timer_period()
    }

    fn update_timer_period(&mut self) {
        // The sweep unit continuously calculates each channel's target period in this way:
        //
        //     1. A barrel shifter shifts the channel's 11-bit raw timer period right by the shift
        //        count, producing the change amount.
        //     2. If the negate flag is true, the change amount is made negative.
        //     3. The target period is the sum of the current period and the change amount.
        //
        // For example, if the negate flag is false and the shift amount is zero, the change
        // amount equals the current period, making the target period equal to twice the current
        // period.
        //
        // Whenever the current period changes for any reason, whether by $400x writes or by sweep,
        // the target period also changes.
        let target_period = {
            let change_amount = self.raw_timer_period >> self.sweep.shift_count();
            if self.sweep.negate_flag() {
                // TODO differences between pulse1 and pulse2
                self.raw_timer_period - N::negate_amount(change_amount)
            } else {
                self.raw_timer_period + change_amount
            }
        };
        self.timer.set_period(target_period)
    }
}

impl<N: Negater> Pulse for PulseImpl<N> {
    fn write_4000_4004(&mut self, val: u8) {
        self.envelope.set_flags(val);
        self.length_counter.set_halt_flag(val & 0b_0010_0000 > 0);
        self.sequencer
            .set_duty_sequence((val & 0b_1100_0000) >> 6);
    }

    fn write_4001_4005(&mut self, val: u8) {
        self.sweep.write_flags(val);
    }

    fn write_4002_4006(&mut self, val: u8) {
        self.set_raw_timer_period_low(val);
    }

    fn write_4003_4007(&mut self, val: u8) {
        self.set_raw_timer_period_high(val);
        self.length_counter.load((val & 0b_1111_1000) >> 3);

        // Side-effects:
        // The sequencer is immediately restarted at the first value of the current sequence. The
        // envelope is also restarted. The period divider is not reset.
        self.sequencer.reset();
        self.envelope.set_start_flag();
    }

    fn clock_timer(&mut self) {
        if self.timer.clock() {
            self.duty_cycle = self.sequencer.clock();
        }
    }

    fn clock_envelope(&mut self) {
        self.envelope.clock()
    }

    fn clock_length_counter(&mut self) {
        self.length_counter.clock();
    }

    fn zero_length_counter(&mut self) {
        self.length_counter.zero();
    }

    fn length_is_nonzero(&self) -> bool {
        self.length_counter.is_nonzero()
    }

    fn clock_sweep(&mut self) {
        if self.sweep.clock() {
            self.update_timer_period()
        }
    }

    fn output(&self) -> (f32, u8) {
        (CPU_FREQ / (16.0 * (self.timer.period() as f32 + 1.0)), self.envelope.output())
    }
}

mod sequencer {
    const WAVEFORM_TABLE: [[u8; 8]; 4] = [[0, 1, 0, 0, 0, 0, 0, 0],
                                          [0, 1, 1, 0, 0, 0, 0, 0],
                                          [0, 1, 1, 1, 1, 0, 0, 0],
                                          [1, 0, 0, 1, 1, 1, 1, 1]];

    #[derive(Default)]
    pub struct Sequencer {
        step: u8,
        duty_sequence: u8,
    }

    impl Sequencer {
        pub fn set_duty_sequence(&mut self, duty_sequence: u8) {
            debug_assert!(duty_sequence < 4);
            self.duty_sequence = duty_sequence;
        }

        pub fn reset(&mut self) {
            self.step = 0
        }

        pub fn clock(&mut self) -> bool {
            if self.step == 0 {
                self.step = 7;
            } else {
                self.step -= 1;
            }
            WAVEFORM_TABLE[self.duty_sequence as usize][self.step as usize] > 0
        }
    }
}
