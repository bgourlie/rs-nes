#![allow(dead_code)]

use apu::Sequencer;

pub enum ClockUnits {
    None,
    All(bool),
    EnvelopesAndTrianglesLinearCounter,
}

pub trait FrameCounter: Default {
    fn write(&mut self, val: u8) -> ClockUnits;
    fn half_step(&mut self) -> ClockUnits;
}

#[derive(Default)]
pub struct FrameCounterImpl {
    reg: u8,
    sequencer: FrameTimerSequencer,
}

impl FrameCounter for FrameCounterImpl {
    fn write(&mut self, val: u8) -> ClockUnits {
        // The rest of the bits are used for input
        self.reg = val & 0b_1100_0000;

        // Writing to $4017 with bit 7 set ($80) will immediately clock all of its controlled units
        // at the beginning of the 5-step sequence; with bit 7 clear, only the sequence is reset
        // without clocking any of its units.
        if val & 0b_1000_0000 == 0 {
            self.sequencer.set_mode(SequenceMode::FourStep);
            ClockUnits::None
        } else {
            self.sequencer.set_mode(SequenceMode::FiveStep);
            ClockUnits::All(false)
        }
    }

    fn half_step(&mut self) -> ClockUnits {
        let reg = self.reg;
        self.sequencer
            .half_step(|step, max_steps| match (step, max_steps) {
                           (1, _) => ClockUnits::EnvelopesAndTrianglesLinearCounter,
                           (2, _) => ClockUnits::All(false),
                           (3, _) => ClockUnits::EnvelopesAndTrianglesLinearCounter,
                           (4, 4) => ClockUnits::All(reg & 0b_0100_0000 == 0),
                           (5, 5) => ClockUnits::All(false),
                           _ => ClockUnits::None,
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

impl Sequencer for FrameTimerSequencer {
    type R = ClockUnits;

    fn half_step<F>(&mut self, sequence_handler: F) -> ClockUnits
        where F: FnOnce(u8, u8) -> ClockUnits
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
                        ClockUnits::None
                    }
                    _ => ClockUnits::None,
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
                        ClockUnits::None
                    }
                    _ => ClockUnits::None,
                }
            }
        }
    }
}
