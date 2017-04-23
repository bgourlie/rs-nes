use apu::envelope::Envelope;

pub trait Noise: Default {
    fn write_envelope_reg(&mut self, val: u8);
    fn write_mode_and_period_reg(&mut self, val: u8);
    fn write_length_load_envelope_restart_reg(&mut self, val: u8);
    fn clock_envelope(&mut self);
}

#[derive(Default)]
pub struct NoiseImpl {
    mode_and_period_reg: u8,
    length_load_envelope_restart_reg: u8,
    envelope: Envelope,
}

impl Noise for NoiseImpl {
    fn write_envelope_reg(&mut self, val: u8) {
        self.envelope.set_flags(val);
    }

    fn write_mode_and_period_reg(&mut self, val: u8) {
        self.mode_and_period_reg = val
    }

    fn write_length_load_envelope_restart_reg(&mut self, val: u8) {
        self.length_load_envelope_restart_reg = val;
        self.envelope.set_start_flag();
    }

    fn clock_envelope(&mut self) {
        self.envelope.clock()
    }
}
