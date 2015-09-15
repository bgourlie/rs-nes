use constants::*;

pub struct Registers {
  pub pc: u16, // Program Counter
  pub sp: u8, // Stack Pointer
  pub acc: u8, // Accumulator
  pub irx: u8, // Index Register X
  pub iry: u8, // Index Register Y
  pub stat: u8 // Processor Status Flags
}

impl Registers {
  pub fn new() -> Registers {
    Registers {
      pc: 0,
      // http://www.pagetable.com/?p=410 explains why the stack pointer has
      // an initial value of 0xfd.
      sp: 0xfd,
      acc: 0,
      irx: 0,
      iry: 0,
      stat: 0b00010000
    }
  }

  pub fn get_flag(&self, mask: u8) -> bool {
    self.stat & mask != 0
  }

  pub fn set_flag(&mut self, mask: u8, val: bool) {
    if val {
      self.stat |= mask;
    } else {
      self.stat &= !mask;
    }
  }

  pub fn set_sign_and_zero_flag(&mut self, val: u8) {
    self.set_flag(FL_SIGN, val & 0x80 != 0);
    self.set_flag(FL_ZERO, val == 0);
  }

  pub fn set_acc(&mut self, res: u8) {
    self.set_sign_and_zero_flag(res);
    self.acc = res;
  }

  pub fn page_boundary_crossed(&self, old_pc: u16) -> bool {
    old_pc & 0xFF00 != self.pc & 0xFF00
  }
}
