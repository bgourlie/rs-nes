use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

mod constants;
mod memory;
mod cpu;
mod rom_loader;

use rom_loader::NesRom;
use cpu::Cpu6502;

fn main() {
  let mut f = File::open("/Users/brian/Desktop/roms/6502_functional_test.bin")
      .unwrap();

  let mut vec = Vec::<u8>::new();
  let bytes_read = f.read_to_end(&mut vec).unwrap();
  if bytes_read < 65536 {
    panic!("expected 16kb of data");
  }

  let mut cpu = Cpu6502::new();
  println!("Press enter to step");
  cpu.load(0, &vec, 0x400);
  let mut last_pc: u16 = 0x400;;

  loop {
    match cpu.step() {
      Ok(instr) => {
          if last_pc == cpu.registers.pc {
            println!("Detected Trap");
            memdump(cpu.memory.get_bytes());
            return;
          }
          last_pc = cpu.registers.pc;
          println!("{} {} cycles: {}", instr.to_string(), cpu.registers.to_string(),
        cpu.cycles);
      },
      Err(msg) => {
        println!("{}", msg);
        memdump(&cpu.memory.get_bytes());
        return;
      }
    }
  }
}

fn memdump(bytes: &[u8]) {
  println!("writing memory dump");
  let mut f = File::create("/Users/brian/Desktop/6502dump.bin").unwrap();
  f.write_all(bytes).unwrap();
}
