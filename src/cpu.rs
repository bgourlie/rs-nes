use memory::Memory;

pub const FL_CARRY: u8 = 1 << 0;
pub const FL_ZERO: u8 = 1 << 1;
pub const FL_INTERRUPT_DISABLE: u8 = 1 << 2;
pub const FL_BRK: u8 = 1 << 4;
pub const FL_OVERFLOW: u8 = 1 << 6;
pub const FL_SIGN: u8 = 1 << 7;

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
  pub registers: Registers,
  memory: Memory
}

pub struct Registers {
  pub pc: u16, // Program Counter
  pub sp: u8, // Stack Pointer
  pub acc: u8, // Accumulator
  pub irx: u8, // Index Register X
  pub iry: u8, // Index Register Y
  pub stat: u8 // Processor Status Flags
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

  pub fn get_flag(&self, mask: u8) -> bool {
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

  pub fn adc(&mut self, lop: u8, rop: u8) {
    // add using the native word size
    let res = if self.registers.get_flag(FL_CARRY) { 1 } else { 0 }
        + lop as usize + rop as usize;

    // if the operation carries into the 8th bit, carry flag will be 1, 
    // and zero othersize.
    let has_carry = res & 0x100 != 0;

    let res = res as u8;

    // Set the overflow flag when both operands have the same sign bit AND
    // the sign bit of the result differs from the two.
    // See: http://www.righto.com/2012/12/the-6502-overflow-flag-explained.html
    let has_overflow = (lop ^ rop) & 0x80 == 0 && (lop ^ res) & 0x80 != 0;

    let is_negative = if !has_overflow { res & 0x80 != 0 } else { res & 0x80 == 0 };
    self.registers.set_flag(FL_CARRY, has_carry);
    self.registers.set_flag(FL_OVERFLOW, has_overflow);
    self.registers.set_flag(FL_SIGN, is_negative);
    self.registers.set_flag(FL_ZERO, res == 0);
    self.registers.acc = res;
  }
}
