pub trait Noise: Default {
    fn write_len_counter_etc_reg(&mut self, val: u8);
    fn write_mode_and_period_reg(&mut self, val: u8);
    fn write_counter_load_and_envelope_restart(&mut self, val: u8);
}

#[derive(Default)]
pub struct NoiseImpl {
    len_counter_etc_reg: u8,
    mode_and_period_reg: u8,
    counter_load_and_envelope_restart: u8,
}

impl Noise for NoiseImpl {
    fn write_len_counter_etc_reg(&mut self, val: u8) {
        self.len_counter_etc_reg = val
    }

    fn write_mode_and_period_reg(&mut self, val: u8) {
        self.mode_and_period_reg = val
    }

    fn write_counter_load_and_envelope_restart(&mut self, val: u8) {
        self.counter_load_and_envelope_restart = val
    }
}
