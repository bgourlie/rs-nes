use apu::length_counter::LengthCounter;
use apu::timer::Timer;

const SEQUENCER_VALUE_TABLE: [u8; 32] = [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 0,
                                         1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

pub trait Triangle: Default {
    fn write_4008(&mut self, val: u8);
    fn write_400a(&mut self, val: u8);
    fn write_400b(&mut self, val: u8);
    fn clock_timer(&mut self);
    fn clock_length_counter(&mut self);
    fn clock_linear_counter(&mut self);
}

#[derive(Default)]
pub struct TriangleImpl {
    length_counter: LengthCounter,
    timer: Timer,
    sequencer: TriangleSequencer,
}

impl Triangle for TriangleImpl {
    fn write_4008(&mut self, val: u8) {}

    fn write_400a(&mut self, val: u8) {}

    fn write_400b(&mut self, val: u8) {}

    fn clock_timer(&mut self) {
        self.timer.clock(|| { /* TODO */ })
    }

    fn clock_length_counter(&mut self) {
        self.length_counter.clock()
    }

    fn clock_linear_counter(&mut self) {
        // TODO
    }
}

#[derive(Default)]
struct TriangleSequencer {
    step: u8,
}

impl TriangleSequencer {
    fn clock(&mut self) -> u8 {
        let step = self.step;
        if step == 0 {
            self.step = 31;
        } else {
            self.step -= 1;
        }
        SEQUENCER_VALUE_TABLE[31 - step as usize]
    }
}
