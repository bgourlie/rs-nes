pub trait Divider {
    fn set_period(&mut self, period: u8);
    fn reload_period(&mut self);
    fn clock<F>(&mut self, clock_handler: F) where F: FnOnce();
}

#[derive(Default)]
pub struct DownCountDivider {
    period: u8,
    counter: u8,
}

impl Divider for DownCountDivider {
    fn set_period(&mut self, period: u8) {
        self.period = period
    }

    fn clock<F>(&mut self, clock_handler: F)
        where F: FnOnce()
    {
        if self.counter == 0 {
            self.reload_period();
            clock_handler()
        }
        self.counter -= 1;
    }

    fn reload_period(&mut self) {
        self.counter = self.period
    }
}
