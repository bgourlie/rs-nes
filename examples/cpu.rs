#[macro_use]
extern crate log;
extern crate env_logger;

extern crate rs_nes;

use std::fs::File;
use std::io::Read;

use rs_nes::cpu::*;
use rs_nes::memory::*;

const PC_START: u16 = 0x400;

fn main() {
    env_logger::init().unwrap();
    let mut f = File::open("test_roms/6502_functional_test.bin").unwrap();
    let mut rom = Vec::<u8>::new();
    f.read_to_end(&mut rom).unwrap();
    let mut mem = SimpleMemory::new();
    mem.store_many(0, &rom);
    let mut cpu = Cpu::new(mem);
    cpu.registers.pc = PC_START;
    loop {
        cpu.step(tick_handler);
    }
}

fn tick_handler(cpu: &Cpu<SimpleMemory>) {
    println!("tick!");
    print_status(cpu);
}

fn print_status(cpu: &Cpu<SimpleMemory>) {
    println!("status: {}", cpu.cycles)
}
