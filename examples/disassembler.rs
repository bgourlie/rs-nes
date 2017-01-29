extern crate log;
extern crate env_logger;
extern crate rs_nes;

use rs_nes::disassembler::InstructionDecoder;
use std::fs::File;
use std::io::Read;

const PC_START: usize = 0x400;

fn main() {
    env_logger::init().unwrap();
    let mut f = File::open("test_roms/6502_functional_test.bin").unwrap();
    let mut rom = Vec::<u8>::new();
    let bytes_read = f.read_to_end(&mut rom).unwrap();
    assert!(bytes_read == 65536);
    let decoder = InstructionDecoder::new(&rom, PC_START);

    for instr in decoder.take(100) {
        println!("{}", instr.mnemonic.to_string());
    }
}
