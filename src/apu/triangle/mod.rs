#![allow(dead_code)]

pub trait Triangle: Default {
    fn write_4008(&mut self, val: u8);
    fn write_400a(&mut self, val: u8);
    fn write_400b(&mut self, val: u8);
}

#[derive(Default)]
pub struct TriangleImpl {
    linear_counter_reg: u8,
    timer_low_reg: u8,
    length_load_timer_high_reg: u8,
}

impl TriangleImpl {
    fn control_flag(&self) -> bool {
        self.linear_counter_reg & 0b_1000_0000 > 0
    }

    fn counter_reload_value(&self) -> u8 {
        self.linear_counter_reg & 0b_0111_1111
    }

    fn timer_period(&self) -> u16 {
        self.timer_low_reg as u16 | (self.length_load_timer_high_reg as u16 & 0b111) << 8
    }
}

impl Triangle for TriangleImpl {
    fn write_4008(&mut self, val: u8) {
        self.linear_counter_reg = val
    }

    fn write_400a(&mut self, val: u8) {
        self.timer_low_reg = val
    }

    fn write_400b(&mut self, val: u8) {
        self.length_load_timer_high_reg = val
    }
}
