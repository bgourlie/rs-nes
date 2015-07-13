use memory::Memory;

const FL_CARRY: u8 = 1 << 0;
const FL_STATUS: u8 = 1 << 1;
const FL_INTERRUPT_DISABLE: u8 = 1 << 2;
const FL_BRK: u8 = 1 << 4;
const FL_OVERFLOW: u8 = 1 << 6;
const FL_SIGN: u8 = 1 << 7;

// Graciously taken from https://github.com/pcwalton/sprocketnes
static CYCLE_TABLE: [u8; 256] = [
  /*0x00*/ 7,6,2,8,3,3,5,5,3,2,2,2,4,4,6,6,
  /*0x10*/ 2,5,2,8,4,4,6,6,2,4,2,7,4,4,7,7,
  /*0x20*/ 6,6,2,8,3,3,5,5,4,2,2,2,4,4,6,6,
  /*0x30*/ 2,5,2,8,4,4,6,6,2,4,2,7,4,4,7,7,
  /*0x40*/ 6,6,2,8,3,3,5,5,3,2,2,2,3,4,6,6,
  /*0x50*/ 2,5,2,8,4,4,6,6,2,4,2,7,4,4,7,7,
  /*0x60*/ 6,6,2,8,3,3,5,5,4,2,2,2,5,4,6,6,
  /*0x70*/ 2,5,2,8,4,4,6,6,2,4,2,7,4,4,7,7,
  /*0x80*/ 2,6,2,6,3,3,3,3,2,2,2,2,4,4,4,4,
  /*0x90*/ 2,6,2,6,4,4,4,4,2,5,2,5,5,5,5,5,
  /*0xA0*/ 2,6,2,6,3,3,3,3,2,2,2,2,4,4,4,4,
  /*0xB0*/ 2,5,2,5,4,4,4,4,2,4,2,4,4,4,4,4,
  /*0xC0*/ 2,6,2,8,3,3,5,5,2,2,2,2,4,4,6,6,
  /*0xD0*/ 2,5,2,8,4,4,6,6,2,4,2,7,4,4,7,7,
  /*0xE0*/ 2,6,3,8,3,3,5,5,2,2,2,2,4,4,6,6,
  /*0xF0*/ 2,5,2,8,4,4,6,6,2,4,2,7,4,4,7,7,
];

pub struct Cpu6502 {
  registers: Registers,
  memory: Memory
}

pub struct Registers {
  pc: u16, // Program Counter
  sp: u8, // Stack Pointer
  acc: u8, // Accumulator
  irx: u8, // Index Register X
  iry: u8, // Index Register Y
  stat: u8 // Processor Status Flags
}

pub enum AddressingMode {
  Immediate,
  ZeroPage,
  ZeroPageX,
  Absolute,
  AboluteX,
  AbsoluteY,
  IndirectX,
  IndirectY
}

impl Registers {
  fn new() -> Registers {
    Registers {
      pc: 0,
      sp: 0,
      acc: 0,
      irx: 0,
      iry: 0,
      stat: 0b00010000
    }
  }

  fn get_flag(&self, mask: u8) -> bool {
    self.stat & mask != 0
  }

  fn set_flag(&mut self, mask: u8, val: bool) {
    if val {
      self.stat |= mask;
    } else {
      self.stat &= !mask;
    }
  }
}

impl Cpu6502 {
 pub fn new() -> Cpu6502 {
    Cpu6502 {
      registers: Registers::new(),
      memory: Memory::new()
    }
  }

  pub fn adc(&mut self, arg: u8, mode: AddressingMode) {
    match mode {
      AddressingMode::Immediate => {
        // Cheaply determine carry flag by casting the values to the native
        // word size and checking if the result carries over to the 9th bit.
        let res = if self.registers.get_flag(FL_CARRY) { 1 } else { 0 }
            + self.registers.acc as usize + arg as usize;

        self.registers.set_flag(FL_CARRY, res & 0x100 != 0);

        let res = res as u8;
        let acc = self.registers.acc;

        // Set the overflow flag when both operands have the same sign bit AND
        // the sign bit of the result differs from the two.
        //
        // This is a clever way of determining whether or not the result overflows
        // 8-bits and works with both signed and unsigned values.
        self.registers.set_flag(FL_OVERFLOW, (acc ^ arg) & 0x80 == 0
            && (acc ^ res) & 0x80 == 0x80);

        // TODO: Set remaining flags
      },
      _ => {
        panic!("not implemented");
      }
    }
  }
}
