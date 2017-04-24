#[derive(Default)]
pub struct Timer {
    period: u16,
    current: u16,
}

impl Timer {
    /// The return value indicates whether or not an output clock occurs
    pub fn clock(&mut self) -> bool {
        if self.current == 0 {
            self.current = self.period;
            true
        } else {
            self.current -= 1;
            false
        }
    }

    pub fn period(&self) -> u16 {
        self.period
    }

    pub fn set_period(&mut self, period: u16) {
        self.period = period;
    }
}
