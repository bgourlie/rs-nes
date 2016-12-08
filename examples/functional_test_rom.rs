#[macro_use]
extern crate log;
extern crate env_logger;

extern crate rs_nes;

use std::fs::File;
use std::io::Read;

use rs_nes::cpu::*;
use rs_nes::cpu::debugger::*;
use rs_nes::memory::*;
use rs_nes::disassembler::*;

const PC_START: u16 = 0x400;

fn main() {
    env_logger::init().unwrap();
    let mut f = File::open("test_roms/6502_functional_test.bin").unwrap();
    let mut rom = Vec::<u8>::new();
    f.read_to_end(&mut rom).unwrap();
    let instructions = InstructionDecoder::new(&rom, PC_START as usize).collect::<Vec<Instruction>>();
    let mut mem = SimpleMemory::new();
    mem.store_many(0, &rom);
    let mut debugger = http_debugger::HttpDebugger::new(instructions);
    debugger.start().unwrap();
    let mut cpu = Cpu::new(mem, debugger);
    cpu.registers.pc = PC_START;
    loop {
        cpu.step();
    }
}
