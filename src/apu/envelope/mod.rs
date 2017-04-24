use apu::timer::Timer;

#[derive(Default)]
pub struct Envelope {
    start_flag: bool,
    decay_counter: u8,
    flags: u8,
    timer: Timer,
}

impl Envelope {
    pub fn set_start_flag(&mut self) {
        self.start_flag = true
    }

    pub fn set_flags(&mut self, flags: u8) {
        // Flags: ---CVVVV
        // Where C is the constant volume flag and V is either:
        // a) The constant volume value (occurs when C is set), or
        // b) The period for the divider that clocks the decay counter (occurs when C is not set)
        self.flags = flags;
        self.timer.set_period(flags as u16 & 0b_0000_1111);
    }

    pub fn clock(&mut self) {
        // When clocked by the frame counter, one of two actions occurs: if the start flag is clear,
        // the divider is clocked, otherwise the start flag is cleared, the decay level counter is
        // loaded with 15, and the divider's period is immediately reloaded.
        if !self.start_flag && self.timer.clock() {
            // When the divider emits a clock one of two actions occurs: If the counter is
            // non-zero, it is decremented, otherwise if the loop flag is set, the decay level
            // counter is loaded with 15.
            if self.decay_counter > 0 {
                self.decay_counter -= 1;
            } else if self.flags & 0b_0010_0000 > 0 {
                self.reload_decay_counter();
            }
        } else {
            self.start_flag = false;
            self.reload_decay_counter();
            self.timer.reload_period();
        }
    }

    fn reload_decay_counter(&mut self) {
        self.decay_counter = 15
    }
}
