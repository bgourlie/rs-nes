const LENGTH_COUNTER_TABLE: [u8; 32] = [10, 254, 20, 2, 40, 4, 80, 6, 160, 8, 60, 10, 14, 12, 26,
                                        14, 12, 16, 24, 18, 48, 20, 96, 22, 192, 24, 72, 26, 16,
                                        28, 32, 30];

#[derive(Default)]
pub struct LengthCounter {
    halt_flag: bool,
    counter: u8,
}

impl LengthCounter {
    pub fn clock(&mut self) {
        if self.counter > 0 && !self.halt_flag {
            self.counter -= 1;
        }
    }

    pub fn is_nonzero(&self) -> bool {
        self.counter > 0
    }

    pub fn zero(&mut self) {
        self.counter = 0;
    }

    pub fn set_halt_flag(&mut self, halt: bool) {
        self.halt_flag = halt;
    }

    pub fn load(&mut self, val: u8) {
        debug_assert!(val < 32);
        self.counter = LENGTH_COUNTER_TABLE[val as usize];
    }
}
