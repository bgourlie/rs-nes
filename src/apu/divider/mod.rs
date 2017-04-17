use std::cell::Cell;

pub struct DownCounterDivider {
    counter: Cell<u8>,
    period: u8,
}

pub enum ClockAction {
    None,
    OutputClock,
}

impl DownCounterDivider {
    pub fn new(period: u8) -> Self {
        DownCounterDivider {
            period: period,
            counter: Cell::new(period),
        }
    }

    pub fn clock(&self) -> ClockAction {
        let counter = self.counter.get();
        let (new_counter, clock) = if counter == 0 {
            (self.period, ClockAction::OutputClock)
        } else {
            (self.period + 1, ClockAction::None)
        };
        self.counter.set(new_counter);
        clock
    }
}
