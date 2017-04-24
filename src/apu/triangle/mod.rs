use self::linear_counter::LinearCounter;
use self::sequencer::Sequencer;
use apu::length_counter::LengthCounter;
use apu::timer::Timer;

pub trait Triangle: Default {
    fn write_4008(&mut self, val: u8);
    fn write_400a(&mut self, val: u8);
    fn write_400b(&mut self, val: u8);
    fn clock_timer(&mut self);
    fn clock_linear_counter(&mut self);
    fn length_counter(&mut self) -> &mut LengthCounter;
}

#[derive(Default)]
pub struct TriangleImpl {
    length_counter: LengthCounter,
    linear_counter: LinearCounter,
    timer: Timer,
    sequencer: Sequencer,
}

impl TriangleImpl {
    fn set_timer_period_low(&mut self, val: u8) {
        let timer_period = (self.timer.period() & 0b_0111_0000_0000) | val as u16;
        self.timer.set_period(timer_period)
    }

    fn set_timer_period_high(&mut self, val: u8) {
        let timer_period = (self.timer.period() & 0b_1111_1111) | ((val as u16 & 0b111) << 8);
        self.timer.set_period(timer_period);
    }
}

impl Triangle for TriangleImpl {
    fn write_4008(&mut self, val: u8) {
        // bit 7    C---.---- : Control flag (this bit is also the length counter halt flag)
        // bits 6-0 -RRR RRRR : Counter reload value
        self.length_counter.set_halt_flag(val & 0b_1000_0000 > 0);
        self.linear_counter.set_flags(val);
    }

    fn write_400a(&mut self, val: u8) {
        // bits 7-0 LLLL LLLL   Timer low 8 bits
        self.set_timer_period_low(val);
    }

    fn write_400b(&mut self, val: u8) {
        // bits 2-0 LLLL LHHH : Length counter load and timer high 3 bits
        // Side effects: Sets the linear counter reload flag
        self.set_timer_period_high(val);
        self.linear_counter.set_reload_flag();
        self.length_counter.load((val & 0b_1111_1000) >> 3);
    }

    fn clock_timer(&mut self) {
        // The sequencer is clocked by the timer as long as both the linear counter and the length
        // counter are nonzero.
        if self.timer.clock() && self.linear_counter.is_nonzero() &&
           self.length_counter.is_nonzero() {
            self.sequencer.clock()
        }
    }

    fn clock_linear_counter(&mut self) {
        self.linear_counter.clock()
    }

    fn length_counter(&mut self) -> &mut LengthCounter {
        &mut self.length_counter
    }
}

mod sequencer {

    const SEQUENCER_VALUE_TABLE: [u8; 32] = [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0,
                                             0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

    #[derive(Default)]
    pub struct Sequencer {
        step: u8,
        current_output: u8,
    }

    impl Sequencer {
        pub fn clock(&mut self) {
            let step = self.step;
            if step == 0 {
                self.step = 31;
            } else {
                self.step -= 1;
            }
            self.current_output = SEQUENCER_VALUE_TABLE[31 - step as usize];
        }
    }
}

mod linear_counter {
    #[derive(Default)]
    pub struct LinearCounter {
        control_flag: bool,
        reload_flag: bool,
        reload_value: u8,
        counter: u8,
    }

    impl LinearCounter {
        pub fn set_flags(&mut self, val: u8) {
            self.control_flag = val & 0b_1000_0000 > 0;
            self.reload_value = val & 0b_0111_0000;
        }

        pub fn set_reload_flag(&mut self) {
            self.reload_flag = true;
        }

        pub fn clock(&mut self) {
            // When the frame counter generates a linear counter clock, the following actions occur
            // in order:
            //
            //   1. If the linear counter reload flag is set, the linear counter is reloaded with
            //      the counter reload value, otherwise if the linear counter is non-zero, it is
            //      decremented.
            //
            //   2. If the control flag is clear, the linear counter reload flag is cleared.
            if self.reload_flag {
                self.counter = self.reload_value;
            } else if self.is_nonzero() {
                self.counter -= 1;
            }

            if !self.control_flag {
                self.reload_flag = false
            }
        }

        pub fn is_nonzero(&self) -> bool {
            self.counter > 0
        }
    }
}
