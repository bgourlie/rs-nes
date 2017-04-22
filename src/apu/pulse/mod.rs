pub trait Pulse: Default {
    fn write_duty_etc_reg(&mut self, val: u8);
    fn write_sweep_reg(&mut self, val: u8);
    fn write_timer_low_reg(&mut self, val: u8);
    fn write_counter_load_timer_high_reg(&mut self, val: u8);
}

#[derive(Default)]
pub struct PulseImpl {
    duty_etc_reg: u8,
    sweep_reg: u8,
    timer_low_reg: u8,
    counter_load_timer_high_reg: u8,
}

impl Pulse for PulseImpl {
    fn write_duty_etc_reg(&mut self, val: u8) {
        self.duty_etc_reg = val;
    }

    fn write_sweep_reg(&mut self, val: u8) {
        self.sweep_reg = val;
    }

    fn write_timer_low_reg(&mut self, val: u8) {
        self.timer_low_reg = val;
    }

    fn write_counter_load_timer_high_reg(&mut self, val: u8) {
        self.counter_load_timer_high_reg = val;
    }
}
