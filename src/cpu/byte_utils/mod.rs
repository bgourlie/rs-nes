pub fn lo_hi(val: u16) -> (u8, u8) {
    let low_byte = (val & 0xff) as u8;
    let high_byte = ((val >> 8) & 0xff) as u8;
    (low_byte, high_byte)
}

pub fn from_lo_hi(low: u8, high: u8) -> u16 {
    low as u16 | (high as u16) << 8
}
