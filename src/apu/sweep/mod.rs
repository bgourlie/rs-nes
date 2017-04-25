use apu::timer::Timer;

#[derive(Default)]
pub struct Sweep {
    period: u8,
    enabled_flag: bool,
    negate_flag: bool,
    shift_count: u8,
    reload_flag: bool,
    timer: Timer,
}

impl Sweep {
    pub fn shift_count(&self) -> u8 {
        self.shift_count
    }

    pub fn negate_flag(&self) -> bool {
        self.negate_flag
    }

    pub fn write_flags(&mut self, flags: u8) {
        // bit 7        E--- ----   Enabled flag
        // bits 6-4	    -PPP ----   The divider's period is set to P + 1
        // bit 3        ---- N---   Negate flag
        //   0: add to period, sweeping toward lower frequencies
        //   1: subtract from period, sweeping toward higher frequencies
        // bits 2-0     ---- -SSS   Shift count (number of bits)
        //
        // Side effects: Sets the reload flag
        self.enabled_flag = flags & 0b_1000_0000 > 0;
        self.period = (flags & 0b_0111_0000) >> 4;
        self.negate_flag = flags & 0b_0000_1000 > 0;
        self.shift_count = flags & 0b_0111;
        self.reload_flag = true;
    }

    /// Clock the sweet unit and return true if the pulse's period should be adjusted
    pub fn clock(&mut self) -> bool {
        // When the frame counter sends a half-frame clock (at 120 or 96 Hz), one of three things
        // happens:
        //
        //   - If the reload flag is set, the divider's counter is set to the period P. If the
        //     divider's counter was zero before the reload and the sweep is enabled, the pulse's
        //     period is also adjusted (if the target period is in range; see below). The reload
        //     flag is then cleared.
        //
        //   - If the reload flag is clear and the divider's counter is non-zero, it is decremented.
        //
        //   - If the reload flag is clear and the divider's counter is zero and the sweep is
        //     enabled, the counter is set to P and the pulse's period is adjusted (if the target
        //     period is in range)
        if self.reload_flag {
            let divider_counter_was_zero = self.timer.is_zero();
            self.timer.set_period(self.period as u16);
            self.timer.reload();
            self.reload_flag = false;
            divider_counter_was_zero && self.enabled_flag
        } else if self.timer.clock() && self.enabled_flag {
            self.timer.set_period(self.period as u16);
            self.timer.reload();
            true
        } else {
            false
        }
    }
}
