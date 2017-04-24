use self::sequencer::Sequencer;
use apu::length_counter::LengthCounter;
use apu::timer::Timer;

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
    sequencer: Sequencer,
}

impl Triangle for TriangleImpl {
    fn write_4008(&mut self, _: u8) {}

    fn write_400a(&mut self, _: u8) {}

    fn write_400b(&mut self, _: u8) {
        // Side effects: Sets the linear counter reload flag
    }

    fn clock_timer(&mut self) {
        let mut clock_sequencer = false;
        self.timer.clock(|| clock_sequencer = true);
        if clock_sequencer {
            self.sequencer.clock()
        }
    }

    fn clock_length_counter(&mut self) {
        self.length_counter.clock()
    }

    fn clock_linear_counter(&mut self) {
        // TODO
    }
}
mod sequencer {

    const SEQUENCER_VALUE_TABLE: [u8; 32] = [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0,
                                             0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

    #[derive(Default)]
    pub struct Sequencer {
        step: u8,
        current_output: u8,
    }

    impl Sequencer {
        pub fn clock(&mut self) {
            let step = self.step;
            if step == 0 {
                self.step = 31;
            } else {
                self.step -= 1;
            }
            self.current_output = SEQUENCER_VALUE_TABLE[31 - step as usize];
        }

        pub fn current_output(&self) -> u8 {
            self.current_output
        }
    }
}
