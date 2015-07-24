use memory::Memory;

pub const FL_CARRY: u8 = 1 << 0;
pub const FL_ZERO: u8 = 1 << 1;
pub const FL_INTERRUPT_DISABLE: u8 = 1 << 2;
pub const FL_BRK: u8 = 1 << 4;
pub const FL_OVERFLOW: u8 = 1 << 6;
pub const FL_SIGN: u8 = 1 << 7;

// Graciously taken from https://github.com/pcwalton/sprocketnes
const CYCLE_TABLE: [u8; 256] = [
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
  pub memory: Memory
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

  pub fn set_flag(&mut self, mask: u8, val: bool) {
    if val {
      self.stat |= mask;
    } else {
      self.stat &= !mask;
    }
  }

  fn set_acc(&mut self, res: u8) {
    self.set_flag(FL_SIGN, res & 0x80 != 0);
    self.set_flag(FL_ZERO, res == 0);
    self.acc = res;
  }

  fn page_boundary_crossed(&self, old_pc: u16) -> bool {
    old_pc & 0xFF00 != self.pc & 0xFF00
  }
}

/// ## Implementation of the 6502 instruction set
///
/// Any instruction that consumes additional cycles under certain conditions
/// will return the number of conditional cycles.  To reiterate, this will 
/// not include cycles that can be determined simply by decoding the instruction.

impl Cpu6502 {
  pub fn new() -> Cpu6502 {
    Cpu6502 {
      registers: Registers::new(),
      memory: Memory::new()
    }
  }

  pub fn adc(&mut self, rop: u8) {
    let lop = self.registers.acc;

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

    self.registers.set_flag(FL_CARRY, has_carry);
    self.registers.set_flag(FL_OVERFLOW, has_overflow);
    self.registers.set_acc(res);
  }

  pub fn and(&mut self, rop: u8) {
    let lop = self.registers.acc;
    let res = lop & rop;
    self.registers.set_acc(res);
  }

  pub fn asl(&mut self) {
    let acc = self.registers.acc;
    self.registers.set_flag(FL_CARRY, acc & 0x80 != 0);
    self.registers.set_acc(acc << 1);
  }

  pub fn lda(&mut self, val: u8) {
    self.registers.set_acc(val);
  }

  /// ##  Branching instructions

  fn branch(&mut self, condition: bool, rel_addr: i8) -> u8 {
    if condition {
      let old_pc = self.registers.pc;
      self.registers.pc = (self.registers.pc as i32 + rel_addr as i32) as u16;
      if self.registers.page_boundary_crossed(old_pc) { 2 } else { 1 }
    } else { 0 }
  }

  pub fn bcc(&mut self, rel_addr: i8) -> u8 {
    let carry_clear = !self.registers.get_flag(FL_CARRY);
    self.branch(carry_clear, rel_addr)
  }

  pub fn bcs(&mut self, rel_addr: i8) -> u8 {
    let carry_set = self.registers.get_flag(FL_CARRY);
    self.branch(carry_set, rel_addr)
  }

  pub fn beq(&mut self, rel_addr: i8) -> u8 {
    let zero_set = self.registers.get_flag(FL_ZERO);
    self.branch(zero_set, rel_addr)
  }

  pub fn bmi(&mut self, rel_addr: i8) -> u8 {
    let sign_set = self.registers.get_flag(FL_SIGN);
    self.branch(sign_set, rel_addr)
  }

  pub fn bne(&mut self, rel_addr: i8) -> u8 {
    let zero_clear = !self.registers.get_flag(FL_ZERO);
    self.branch(zero_clear, rel_addr)
  }

  pub fn bpl(&mut self, rel_addr: i8) -> u8 {
    let sign_clear = !self.registers.get_flag(FL_SIGN);
    self.branch(sign_clear, rel_addr)
  }

  pub fn bvc(&mut self, rel_addr: i8) -> u8 {
    let overflow_clear = !self.registers.get_flag(FL_OVERFLOW);
    self.branch(overflow_clear, rel_addr)
  }

  pub fn bvs(&mut self, rel_addr: i8) -> u8 {
    let overflow_set = self.registers.get_flag(FL_OVERFLOW);
    self.branch(overflow_set, rel_addr)
  }

  /// Status setters

  pub fn sec(&mut self) {
    self.registers.set_flag(FL_CARRY, true);
  }

  pub fn sei(&mut self) {
    self.registers.set_flag(FL_INTERRUPT_DISABLE, true);
  }

  /// ## Stores
  
  pub fn sta(&mut self, addr: u16) {
    self.memory.store(addr, self.registers.acc);
  }

  pub fn stx(&mut self, addr: u16) {
    self.memory.store(addr, self.registers.irx);
  }

  pub fn sty(&mut self, addr: u16) {
    self.memory.store(addr, self.registers.iry);
  }
}
