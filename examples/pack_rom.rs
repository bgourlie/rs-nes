extern crate byteorder;
extern crate rs_nes;

use std::fs::File;
use std::io::{Cursor, Read};

use byteorder::{LittleEndian, ReadBytesExt};

fn main() {
    let mut f = File::open("test_roms/6502_functional_test.bin").unwrap();
    let mut rom = Vec::<u8>::new();
    let bytes_read = f.read_to_end(&mut rom).unwrap();
    assert!(bytes_read == 65536);
    let mut packed = Vec::<i32>::new();
    for i in 0..(65536 / 4) {
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
        packed.push(val)
    }

    println!("{:?}", packed)
}
