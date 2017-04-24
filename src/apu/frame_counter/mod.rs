#![allow(dead_code)]

pub enum Clock {
    None,
    All(bool),
    EnvelopeAndTriangleLinearCounter,
}

pub trait FrameCounter: Default {
    fn write_4017(&mut self, val: u8) -> Clock;
    fn half_step(&mut self) -> Clock;
}

#[derive(Default)]
pub struct FrameCounterImpl {
    sequencer: FrameTimerSequencer,
}

impl FrameCounter for FrameCounterImpl {
    fn write_4017(&mut self, val: u8) -> Clock {
        // Bit 7    M--- ----   Sequencer mode: 0 selects 4-step sequence, 1 selects 5-step sequence
        // Bit 6    -I-- ----   Interrupt inhibit flag. If set, the frame interrupt flag is cleared,
        //                      otherwise it is unaffected.
        //
        // TODO:
        // Side effects: After 3 or 4 CPU clock cycles*, the timer is reset. If the mode flag is
        // set, then both "quarter frame" and "half frame" signals are also generated.
        //
        // * If the write occurs during an APU cycle, the effects occur 3 CPU cycles after the $4017
        //   write cycle, and if the write occurs between APU cycles, the effects occurs 4 CPU
        //   cycles after the write cycle.
        self.sequencer.set_flags(val)
    }

    fn half_step(&mut self) -> Clock {
        self.sequencer.clock()
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
    interrupt_inhibit: bool,
    mode: SequenceMode,
}

impl FrameTimerSequencer {
    fn set_flags(&mut self, val: u8) -> Clock {
        self.interrupt_inhibit = val & 0b_0100_0000 > 0;

        // Writing to $4017 with bit 7 set ($80) will immediately clock all of its controlled units
        // at the beginning of the 5-step sequence; with bit 7 clear, only the sequence is reset
        // without clocking any of its units.
        if val & 0b_1000_0000 == 0 {
            self.mode = SequenceMode::FourStep;
            Clock::None
        } else {
            self.mode = SequenceMode::FiveStep;
            Clock::All(false)
        }
    }

    fn clock(&mut self) -> Clock {
        self.half_steps += 1;
        match (self.half_steps, self.mode) {
            (7547, _) => Clock::EnvelopeAndTriangleLinearCounter,
            (14913, _) => Clock::All(false),
            (22371, _) => Clock::EnvelopeAndTriangleLinearCounter,
            (29829, SequenceMode::FourStep) => Clock::All(!self.interrupt_inhibit),
            (29830, SequenceMode::FourStep) |
            (37282, SequenceMode::FiveStep) => {
                self.half_steps = 0;
                Clock::None
            }
            (37281, SequenceMode::FiveStep) => Clock::All(false),
            _ => Clock::None,
        }
    }
}
