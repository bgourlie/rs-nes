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
      stat: 0b00100100
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

impl ToString for Registers {
  fn to_string(&self) -> String {
    let c = if self.stat & 0x1 > 0 { 1 } else { 0 };
    let z = if self.stat & 0x2 > 0 { 1 } else { 0 };
    let i = if self.stat & 0x4 > 0 { 1 } else { 0 };
    let d = if self.stat & 0x8 > 0 { 1 } else { 0 };
    let b = if self.stat & 0x10 > 0 { 1 } else { 0 };
    let unused = if self.stat & 0x20 > 0 { 1 } else { 0 };
    let v = if self.stat & 0x40 > 0 { 1 } else { 0 };
    let s = if self.stat & 0x80 > 0 { 1 } else { 0 };
    format!("PC:{:0>4X} SP:{:0>2X} A:{:0>2X} X:{:0>2X} Y:{:0>2X} Status: {:0>2X} s={} v={} _={} b={} d={} i={} z={} c={}",
        self.pc, self.sp, self.acc, self.irx, self.iry, self.stat, s, v, unused, b, d, i, z, c)
  }
}
