#[cfg(feature = "debugger")]
use byteorder::{LittleEndian, ReadBytesExt};
#[cfg(feature = "debugger")]
use std::io::Cursor;
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

fn wrapping_subtract(lhs: u8, rhs: u8) -> u8 {
    (Wrapping(lhs) - Wrapping(rhs)).0
}

// Convert an array of bytes into an array 32-bit signed integers.
//
// This is done to reduce the json payload when serializing memory. Once elm supports binary data,
// this shouldn't be necessary.
#[cfg(feature = "debugger")]
pub fn pack_memory(rom: &[u8]) -> Vec<i32> {
    let mut packed = Vec::<i32>::new();
    for i in 0..(rom.len() / 4) {
        let bytes = {
            let index = i * 4;
            let b1 = rom[index];
            let b2 = rom[index + 1];
            let b3 = rom[index + 2];
            let b4 = rom[index + 3];
            [b1, b2, b3, b4]
        };

        let mut buffer = Cursor::new(&bytes[..]);
        let val = buffer.read_i32::<LittleEndian>().unwrap();
        packed.push(val);
    }
    packed
}
