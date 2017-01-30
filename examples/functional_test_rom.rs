extern crate log;
extern crate env_logger;
extern crate rs_nes;

use rs_nes::cpu::*;
use rs_nes::cpu::debugger::HttpDebugger;
use rs_nes::memory::*;
use std::fs::File;
use std::io::Read;

const PC_START: u16 = 0x400;

fn main() {
    env_logger::init().unwrap();
    let mut f = File::open("test_roms/6502_functional_test.bin").unwrap();
    let mut buf = Vec::new();
    f.read_to_end(&mut buf).unwrap();
    let mut mem = SimpleMemory::new();
    mem.store_many(0, &buf);
    let cpu = Cpu::new(mem, PC_START);
    let mut debugger = HttpDebugger::new(cpu);
    debugger.start().unwrap();

    loop {
        debugger.step();
    }
}
