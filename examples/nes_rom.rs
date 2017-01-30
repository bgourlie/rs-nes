extern crate log;
extern crate env_logger;

extern crate rs_nes;

use rs_nes::cpu::*;
use rs_nes::cpu::debugger::HttpDebugger;
use rs_nes::memory::nes_memory::NesMemory;
use rs_nes::rom::NesRom;

fn main() {
    env_logger::init().unwrap();
    let rom = NesRom::read("test_roms/mario.nes").unwrap();
    let mem = NesMemory::new(rom);
    let cpu = Cpu::new(mem, 0x8000);
    let mut debugger = HttpDebugger::new(cpu);
    debugger.start().unwrap();
    loop {
        debugger.step();
    }
}
