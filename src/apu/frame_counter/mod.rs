#![allow(dead_code)]

use apu::Sequencer;

pub enum StepResult {
    None,
    ClockAll(bool),
    ClockEnvelopesAndTrianglesLinearCounter,
}

pub trait FrameCounter: Default {
    fn write(&mut self, val: u8);
    fn half_step(&mut self) -> StepResult;
}

#[derive(Default)]
pub struct FrameCounterImpl {
    reg: u8,
    sequencer: FrameTimerSequencer,
}

impl FrameCounter for FrameCounterImpl {
    fn write(&mut self, val: u8) {
        // The rest of the bits are used for input
        self.reg = val & 0b_1100_0000;
    }

    fn half_step(&mut self) -> StepResult {
        let reg = self.reg;
        self.sequencer
            .half_step(|step, max_steps| match (step, max_steps) {
                           (1, _) => StepResult::ClockEnvelopesAndTrianglesLinearCounter,
                           (2, _) => StepResult::ClockAll(false),
                           (3, _) => StepResult::ClockEnvelopesAndTrianglesLinearCounter,
                           (4, 4) => StepResult::ClockAll(reg & 0b_0100_0000 == 0),
                           (5, 5) => StepResult::ClockAll(false),
                           _ => StepResult::None,
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
    type R = StepResult;

    fn half_step<F>(&mut self, sequence_handler: F) -> StepResult
        where F: FnOnce(u8, u8) -> StepResult
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
                        StepResult::None
                    }
                    _ => unreachable!(),
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
                        StepResult::None
                    }
                    _ => unreachable!(),
                }
            }
        }
    }
}
