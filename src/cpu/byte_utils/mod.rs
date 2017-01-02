use std::num::Wrapping;

pub fn lo_hi(val: u16) -> (u8, u8) {
    let low_byte = (val & 0xff) as u8;
    let high_byte = ((val >> 8) & 0xff) as u8;
    (low_byte, high_byte)
}

pub fn from_lo_hi(low: u8, high: u8) -> u16 {
    low as u16 | (high as u16) << 8
}

pub fn wrapping_add(lhs: u8, rhs: u8) -> u8 {
    (Wrapping(lhs) + Wrapping(rhs)).0
}

pub fn wrapping_inc(val: u8) -> u8 {
    wrapping_add(val, 1)
}

pub fn wrapping_dec(val: u8) -> u8 {
    wrapping_subtract(val, 1)
}

pub fn wrapping_subtract(lhs: u8, rhs: u8) -> u8 {
    (Wrapping(lhs) - Wrapping(rhs)).0
}
