#[derive(Default)]
pub struct LengthCounter {
    counter: u8,
}

impl LengthCounter {
    pub fn clock(&mut self, counter_halt: bool) {
        if self.counter > 0 && !counter_halt {
            self.counter -= 1;
        }
    }

    pub fn set(&mut self, load: u8) {
        self.counter = load
    }
}
