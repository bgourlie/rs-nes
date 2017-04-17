use super::divider::DownCounterDivider;
use cpu::Interrupt;

struct FrameCounter {
    reg: u8,
    divider: DownCounterDivider,
}

impl FrameCounter {
    pub fn tick(&mut self) -> Interrupt {
        Interrupt::None
    }

    pub fn write(&mut self, val: u8) {
        // The rest of the bits are used for input
        self.reg = val & 0b_1100_0000;
    }
}
