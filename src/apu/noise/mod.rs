pub trait Noise: Default {
    fn write_counter_halt_etc_reg(&mut self, val: u8);
    fn write_mode_and_period_reg(&mut self, val: u8);
    fn write_length_load_and_envelope_restart(&mut self, val: u8);
}

#[derive(Default)]
pub struct NoiseImpl {
    counter_halt_etc_reg: u8,
    mode_and_period_reg: u8,
    length_load_and_envelope_restart: u8,
}

impl Noise for NoiseImpl {
    fn write_counter_halt_etc_reg(&mut self, val: u8) {
        self.counter_halt_etc_reg = val
    }

    fn write_mode_and_period_reg(&mut self, val: u8) {
        self.mode_and_period_reg = val
    }

    fn write_length_load_and_envelope_restart(&mut self, val: u8) {
        self.length_load_and_envelope_restart = val
    }
}
