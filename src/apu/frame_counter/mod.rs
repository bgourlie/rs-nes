#![allow(dead_code)]

pub enum Clock {
    None,
    All(bool),
    EnvelopesAndTrianglesLinearCounter,
}

pub trait FrameCounter: Default {
    fn write_4017(&mut self, val: u8, on_full_cycle: bool) -> Clock;
    fn half_step(&mut self) -> Clock;
}

#[derive(Default)]
pub struct FrameCounterImpl {
    reg: u8,
    sequencer: FrameTimerSequencer,
    reset_cycles: Option<u8>,
}

impl FrameCounter for FrameCounterImpl {
    fn write_4017(&mut self, val: u8, on_full_cycle: bool) -> Clock {
        // The rest of the bits are used for input
        self.reg = val & 0b_1100_0000;
        if on_full_cycle {
            self.reset_cycles = Some(3)
        } else {
            self.reset_cycles = Some(4)
        }

        // Writing to $4017 with bit 7 set ($80) will immediately clock all of its controlled units
        // at the beginning of the 5-step sequence; with bit 7 clear, only the sequence is reset
        // without clocking any of its units.
        if val & 0b_1000_0000 == 0 {
            self.sequencer.set_mode(SequenceMode::FourStep);
            Clock::None
        } else {
            self.sequencer.set_mode(SequenceMode::FiveStep);
            Clock::All(false)
        }
    }

    fn half_step(&mut self) -> Clock {
        self.reset_cycles = match self.reset_cycles {
            Some(0) => {
                self.sequencer.reset();
                None
            }
            Some(cycles) => Some(cycles - 1),
            None => None,
        };

        let reg = self.reg;
        self.sequencer
            .half_step(|step, max_steps| match (step, max_steps) {
                           (1, _) => Clock::EnvelopesAndTrianglesLinearCounter,
                           (2, _) => Clock::All(false),
                           (3, _) => Clock::EnvelopesAndTrianglesLinearCounter,
                           (4, 4) => Clock::All(reg & 0b_0100_0000 == 0),
                           (5, 5) => Clock::All(false),
                           _ => Clock::None,
                       })
    }
}

#[derive(Copy, Clone)]
enum SequenceMode {
    FourStep,
    FiveStep,
}

impl Default for SequenceMode {
    fn default() -> Self {
        SequenceMode::FourStep
    }
}

#[derive(Default)]
pub struct FrameTimerSequencer {
    half_steps: u16,
    mode: SequenceMode,
}

impl FrameTimerSequencer {
    fn set_mode(&mut self, mode: SequenceMode) {
        self.mode = mode;
        self.half_steps = 0;
    }
}

impl FrameTimerSequencer {
    fn reset(&mut self) {
        self.half_steps = 0;
    }

    fn half_step<F>(&mut self, sequence_handler: F) -> Clock
        where F: FnOnce(u8, u8) -> Clock
    {
        self.half_steps += 1;
        match self.mode {
            SequenceMode::FourStep => {
                match self.half_steps {
                    7457 => sequence_handler(1, 4),
                    14913 => sequence_handler(2, 4),
                    22371 => sequence_handler(3, 4),
                    29829 => sequence_handler(4, 4),
                    29830 => {
                        self.half_steps = 0;
                        Clock::None
                    }
                    _ => Clock::None,
                }
            }
            SequenceMode::FiveStep => {
                match self.half_steps {
                    7457 => sequence_handler(1, 5),
                    14913 => sequence_handler(2, 5),
                    22371 => sequence_handler(3, 5),
                    29829 => sequence_handler(4, 5),
                    37281 => sequence_handler(5, 5),
                    37282 => {
                        self.half_steps = 0;
                        Clock::None
                    }
                    _ => Clock::None,
                }
            }
        }
    }
}
