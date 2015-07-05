use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Error;

struct Cpu6502 {
  registers: Registers,
  memory: Memory
}

struct Registers {
  pc: u16, // Program Counter
  sp: u8, // Stack Pointer
  acc: u8, // Accumulator
  irx: u8, // Index Register X
  iry: u8, // Index Register Y
  stat: u8 // Processor Status Flags
}

impl Registers {
  fn new() -> Registers {
    Registers {
      pc: 0,
      sp: 0,
      acc: 0,
      irx: 0,
      iry: 0,
      stat: 0
    }
  }
}

struct Memory {
  addr:[u8; 0xffff]
}

impl Memory {
  fn new() -> Memory {
    Memory {
      addr: [0; 0xffff]
    }
  }

  fn store(&mut self, addr: usize, data: u8) -> Result<(), &'static str> {
    if addr > 0xffff {
      Err("memory address out of bounds")
    } else {
      self.addr[addr] = data;
      Ok(()) // TODO: There must be a cleaner way of doing this
    }
  }
}

impl Cpu6502 {
  fn new() -> Cpu6502 {
    Cpu6502 {
      registers: Registers::new(),
      memory: Memory::new()
    }
  }
}

impl Display for Cpu6502 {
  fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
    write!(formatter, "{}|{}|{}|{}|{}|{}",
        self.registers.pc, self.registers.sp, self.registers.acc,
        self.registers.irx, self.registers.iry, self.registers.stat)
  }
}
