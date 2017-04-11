#![allow(unused_imports)]

extern crate log;
extern crate env_logger;

extern crate rs_nes;

use rs_nes::cpu::*;
use rs_nes::memory::nes_memory::NesMemoryImpl;
use rs_nes::ppu::{Ppu, PpuImpl};
use rs_nes::rom::NesRom;
use rs_nes::screen::NesScreen;
use std::cell::RefCell;
use std::env;
use std::rc::Rc;

#[cfg(feature = "debugger")]
fn main() {
    env_logger::init().unwrap();
    let file = env::args().last().unwrap();
    let rom = NesRom::read(format!("{}", file)).expect("Couldn't find rom file");
    println!("ROM Mapper: {} CHR banks: {} CHR size: {}",
             rom.mapper,
             rom.chr_rom_banks,
             rom.chr.len());

    let screen = Rc::new(RefCell::new(NesScreen::default()));
    let ppu = PpuImpl::new(rom.clone(), screen.clone());
    let mem = NesMemoryImpl::new(rom, ppu);
    let mut cpu = Cpu::new(mem);
    cpu.reset();
    let mut debugger = rs_nes::cpu::debugger::HttpDebugger::new(cpu, screen);
    debugger.start();
    loop {
        debugger.step();
    }
}

#[cfg(not(feature = "debugger"))]
fn main() {
    panic!("You must run this example with the debugger feature enabled.")
}
