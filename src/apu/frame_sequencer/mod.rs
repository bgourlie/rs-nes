#![allow(dead_code)]

pub trait FrameSequencer: Default {
    fn write(&mut self, val: u8);
}

#[derive(Default)]
pub struct FrameSequencerImpl {
    reg: u8,
}

impl FrameSequencer for FrameSequencerImpl {
    fn write(&mut self, val: u8) {
        // The rest of the bits are used for input
        self.reg = val & 0b_1100_0000;
    }
}
