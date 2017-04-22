#![allow(dead_code)]

#[derive(Default)]
pub struct FrameSequencer {
    reg: u8,
}

impl FrameSequencer {
    pub fn write(&mut self, val: u8) {
        // The rest of the bits are used for input
        self.reg = val & 0b_1100_0000;
    }
}
