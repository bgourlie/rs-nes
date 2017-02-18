#![allow(unused_imports)]

extern crate log;
extern crate env_logger;

extern crate rs_nes;

use rs_nes::cpu::*;
use rs_nes::memory::nes_memory::NesMemoryImpl;
use rs_nes::rom::NesRom;

#[cfg(feature = "debugger")]
fn main() {
    env_logger::init().unwrap();
    let rom = NesRom::read("test_roms/mario.nes").unwrap();
    println!("ROM Mapper: {}", rom.mapper);
    let mem = NesMemoryImpl::new(rom);
    let mut cpu = Cpu::new(mem);
    cpu.reset().unwrap();
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
