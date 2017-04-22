#[derive(Default)]
pub struct PulseGenerator {
    duty_etc_reg: u8,
    sweep_reg: u8,
    timer_low_reg: u8,
    len_low_timer_high_reg: u8,
}

impl PulseGenerator {
    pub fn write_duty_etc_reg(&mut self, val: u8) {
        self.duty_etc_reg = val;
    }

    pub fn write_sweep_reg(&mut self, val: u8) {
        self.sweep_reg = val;
    }

    pub fn write_timer_low_reg(&mut self, val: u8) {
        self.timer_low_reg = val;
    }

    pub fn write_len_low_timer_high_reg(&mut self, val: u8) {
        self.len_low_timer_high_reg = val;
    }
}
