#![allow(dead_code)]

pub trait FrameCounter: Default {
    fn write(&mut self, val: u8);
    fn half_step(&mut self);
}

#[derive(Default)]
pub struct FrameCounterImpl {
    reg: u8,
}

impl FrameCounter for FrameCounterImpl {
    fn write(&mut self, val: u8) {
        // The rest of the bits are used for input
        self.reg = val & 0b_1100_0000;
    }

    fn half_step(&mut self) {
        unimplemented!()
    }
}
