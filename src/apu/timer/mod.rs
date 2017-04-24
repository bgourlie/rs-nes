#[derive(Default)]
pub struct Timer {
    period: u16,
    current: u16,
}

impl Timer {
    pub fn clock<F>(&mut self, output_clock_handler: F)
        where F: FnOnce()
    {
        if self.current == 0 {
            self.current = self.period;
            output_clock_handler();
        } else {
            self.current -= 1;
        }
    }

    pub fn set_period(&mut self, period: u16) {
        self.period = period;
    }
}
