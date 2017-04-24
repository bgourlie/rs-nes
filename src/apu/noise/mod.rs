use apu::envelope::Envelope;
use apu::length_counter::LengthCounter;
use apu::timer::Timer;

const TIMER_PERIOD_TABLE: [u16; 32] = [4, 8, 16, 32, 64, 96, 128, 160, 202, 254, 380, 508, 762,
                                       1016, 2034, 4068, 4, 8, 14, 30, 60, 88, 118, 148, 188, 236,
                                       354, 472, 708, 944, 1890, 3778];

pub trait Noise: Default {
    fn write_envelope_reg(&mut self, val: u8);
    fn write_mode_and_period_reg(&mut self, val: u8);
    fn write_length_load_envelope_restart_reg(&mut self, val: u8);
    fn clock_envelope(&mut self);
    fn clock_length_counter(&mut self);
    fn clock_timer(&mut self);
}

pub struct NoiseImpl {
    mode_flag: bool,
    envelope: Envelope,
    length_counter: LengthCounter,
    timer: Timer,
    shift_register: u16,
}

impl Default for NoiseImpl {
    fn default() -> Self {
        NoiseImpl {
            mode_flag: false,
            envelope: Envelope::default(),
            length_counter: LengthCounter::default(),
            timer: Timer::default(),
            shift_register: 1,
        }
    }
}

impl NoiseImpl {
    fn clock_shift_register(&mut self) {
        // Feedback is calculated as the exclusive-OR of bit 0 and one other bit: bit 6 if Mode flag
        // is set, otherwise bit 1
        let feedback = {
            let feedback_bit_1 = self.shift_register & 1;
            let feedback_bit_2 = if self.mode_flag {
                (self.shift_register & 0b_0100_0000) >> 6
            } else {
                (self.shift_register & 0b_0010) >> 1
            };
            feedback_bit_1 ^ feedback_bit_2
        };

        // The shift register is shifted right by one bit.
        self.shift_register >>= 1;

        // Bit 14, the leftmost bit, is set to the feedback calculated earlier.
        self.shift_register = (self.shift_register & 0b0011_1111_1111_1111) | (feedback << 14);
    }
}

impl Noise for NoiseImpl {
    fn write_envelope_reg(&mut self, val: u8) {
        self.length_counter.set_halt_flag(val & 0b_0010_0000 > 0);
        self.envelope.set_flags(val);
    }

    fn write_mode_and_period_reg(&mut self, val: u8) {
        self.timer
            .set_period(TIMER_PERIOD_TABLE[val as usize & 0b_1111]);
        self.mode_flag = val & 0b_1000_0000 > 0
    }

    fn write_length_load_envelope_restart_reg(&mut self, val: u8) {
        self.length_counter.load((val & 0b_1111_1000) >> 3);
        self.envelope.set_start_flag();
    }

    fn clock_envelope(&mut self) {
        self.envelope.clock()
    }

    fn clock_timer(&mut self) {
        let mut clock_shift_register = false;
        self.timer.clock(|| { clock_shift_register = true; });

        if clock_shift_register {
            self.clock_shift_register()
        }
    }

    fn clock_length_counter(&mut self) {
        self.length_counter.clock()
    }
}
