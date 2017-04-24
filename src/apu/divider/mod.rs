use std::marker::PhantomData;

pub trait Divider {
    type T: Default;
    fn set_period(&mut self, period: u8);
    fn reload_period(&mut self);
    fn clock<F>(&mut self, clock_handler: F) -> Self::T where F: FnOnce() -> Self::T;
}

#[derive(Default)]
pub struct DownCountDivider<T: Default> {
    period: u8,
    counter: u8,
    phantom: PhantomData<T>,
}

impl<T: Default> Divider for DownCountDivider<T> {
    type T = T;

    fn set_period(&mut self, period: u8) {
        self.period = period
    }

    fn clock<F>(&mut self, clock_handler: F) -> Self::T
        where F: FnOnce() -> Self::T
    {
        self.counter += 1;
        if self.counter > self.period {
            self.counter = 0;
            clock_handler()
        } else {
            T::default()
        }
    }

    fn reload_period(&mut self) {
        self.counter = self.period
    }
}
