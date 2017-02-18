#![allow(unused_imports, dead_code)]

extern crate log;
extern crate env_logger;
extern crate rs_nes;

use rs_nes::cpu::*;
use rs_nes::memory::*;
use std::fs::File;
use std::io::Read;

const PC_START: u16 = 0x400;

#[cfg(feature = "debugger")]
fn main() {
    env_logger::init().unwrap();
    let mut f = File::open("test_roms/6502_functional_test.bin").unwrap();
    let mut buf = Vec::new();
    f.read_to_end(&mut buf).unwrap();
    let mut mem = SimpleMemory::new();
    mem.store_many(PC_START, &buf);
    let cpu = Cpu::new_init_pc(mem, PC_START);
    let mut debugger = rs_nes::cpu::debugger::HttpDebugger::new(cpu);
    debugger.start().unwrap();

    loop {
        debugger.step().unwrap();
    }
}

#[cfg(not(feature = "debugger"))]
fn main() {
    panic!("You must run this example with the debugger feature enabled.")
}
