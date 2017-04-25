#[derive(Default)]
pub struct Timer {
    period: u16,
    counter: u16,
}

impl Timer {
    /// The return value indicates whether or not an output clock occurs
    pub fn clock(&mut self) -> bool {
        if self.counter == 0 {
            self.counter = self.period;
            true
        } else {
            self.counter -= 1;
            false
        }
    }

    pub fn period(&self) -> u16 {
        self.period
    }

    pub fn set_period(&mut self, period: u16) {
        self.period = period;
    }

    pub fn reload(&mut self) {
        self.counter = self.period
    }

    pub fn counter(&self) -> u16 {
        self.counter
    }

    pub fn is_zero(&self) -> bool {
        self.counter == 0
    }
}
