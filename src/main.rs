use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Error;

fn main() {
    let cpu = Cpu6502::new();
    println!("Hello, {}!", cpu);
}

struct Cpu6502 {
    pc: u16, // Program Counter
    sp: u8, // Stack Pointer
    acc: u8, // Accumulator
    irx: u8, // Index Register X
    iry: u8, // Index Register Y
    stat: u8 // Processor Status Flags
}

impl Cpu6502 {
    fn new() -> Cpu6502 {
        Cpu6502 {
            pc: 0,
            sp: 0,
            acc: 0,
            irx: 0,
            iry: 0,
            stat: 0
        }
    }
}

impl Display for Cpu6502 {
  fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
      write!(formatter, "{}|{}|{}|{}|{}|{}", self.pc, self.sp, self.acc, self.irx, self.iry, self.stat)
  }
}
