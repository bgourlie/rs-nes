#[derive(Default)]
pub struct NoiseGenerator {
    len_counter_etc_reg: u8,
    mode_and_period_reg: u8,
    counter_load_and_envelope_restart: u8,
}

impl NoiseGenerator {
    pub fn write_len_counter_etc_reg(&mut self, val: u8) {
        self.len_counter_etc_reg = val
    }

    pub fn write_mode_and_period_reg(&mut self, val: u8) {
        self.mode_and_period_reg = val
    }

    pub fn write_counter_load_and_envelope_restart(&mut self, val: u8) {
        self.counter_load_and_envelope_restart = val
    }
}
