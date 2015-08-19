use memory::Memory;

pub const STACK_LOC: u16 = 0x100;

pub const FL_CARRY: u8 = 1 << 0;
pub const FL_ZERO: u8 = 1 << 1;
pub const FL_INTERRUPT_DISABLE: u8 = 1 << 2;
pub const FL_BRK: u8 = 1 << 4;
pub const FL_OVERFLOW: u8 = 1 << 6;
pub const FL_SIGN: u8 = 1 << 7;

//// Graciously taken from https://github.com/pcwalton/sprocketnes
// const CYCLE_TABLE: [u8; 256] = [
//   /*0x00*/ 7,6,2,8,3,3,5,5,3,2,2,2,4,4,6,6,
//   /*0x10*/ 2,5,2,8,4,4,6,6,2,4,2,7,4,4,7,7,
//   /*0x20*/ 6,6,2,8,3,3,5,5,4,2,2,2,4,4,6,6,
//   /*0x30*/ 2,5,2,8,4,4,6,6,2,4,2,7,4,4,7,7,
//   /*0x40*/ 6,6,2,8,3,3,5,5,3,2,2,2,3,4,6,6,
//   /*0x50*/ 2,5,2,8,4,4,6,6,2,4,2,7,4,4,7,7,
//   /*0x60*/ 6,6,2,8,3,3,5,5,4,2,2,2,5,4,6,6,
//   /*0x70*/ 2,5,2,8,4,4,6,6,2,4,2,7,4,4,7,7,
//   /*0x80*/ 2,6,2,6,3,3,3,3,2,2,2,2,4,4,4,4,
//   /*0x90*/ 2,6,2,6,4,4,4,4,2,5,2,5,5,5,5,5,
//   /*0xA0*/ 2,6,2,6,3,3,3,3,2,2,2,2,4,4,4,4,
//   /*0xB0*/ 2,5,2,5,4,4,4,4,2,4,2,4,4,4,4,4,
//   /*0xC0*/ 2,6,2,8,3,3,5,5,2,2,2,2,4,4,6,6,
//   /*0xD0*/ 2,5,2,8,4,4,6,6,2,4,2,7,4,4,7,7,
//   /*0xE0*/ 2,6,3,8,3,3,5,5,2,2,2,2,4,4,6,6,
//   /*0xF0*/ 2,5,2,8,4,4,6,6,2,4,2,7,4,4,7,7,
// ];

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

  fn set_sign_and_zero_flag(&mut self, val: u8) {
    self.set_flag(FL_SIGN, val & 0x80 != 0);
    self.set_flag(FL_ZERO, val == 0);
  }

  fn set_acc(&mut self, res: u8) {
    self.set_sign_and_zero_flag(res);
    self.acc = res;
  }

  fn page_boundary_crossed(&self, old_pc: u16) -> bool {
    old_pc & 0xFF00 != self.pc & 0xFF00
  }
}

impl Cpu6502 {
  pub fn new() -> Cpu6502 {
    Cpu6502 {
      registers: Registers::new(),
      memory: Memory::new()
    }
  }


  fn do_op(&mut self, opcode: u8) {
    match opcode {
      // # Loads
      // lda
      0xa1 => { panic!("unimplemented"); }
      0xa5 => { panic!("unimplemented"); }
      0xa9 => { panic!("unimplemented"); }
      0xad => { panic!("unimplemented"); }
      0xb1 => { panic!("unimplemented"); }
      0xb5 => { panic!("unimplemented"); }
      0xb9 => { panic!("unimplemented"); }
      0xbd => { panic!("unimplemented"); }

      // ldx
      0xa2 => { panic!("unimplemented"); }
      0xa6 => { panic!("unimplemented"); }
      0xb6 => { panic!("unimplemented"); }
      0xae => { panic!("unimplemented"); }
      0xbe => { panic!("unimplemented"); }

      // ldy
      0xa0 => { panic!("unimplemented"); }
      0xa4 => { panic!("unimplemented"); }
      0xb4 => { panic!("unimplemented"); }
      0xac => { panic!("unimplemented"); }
      0xbc => { panic!("unimplemented"); }

      // # Stores
      // sta
      0x85 => { panic!("unimplemented"); }
      0x95 => { panic!("unimplemented"); }
      0x8d => { panic!("unimplemented"); }
      0x9d => { panic!("unimplemented"); }
      0x99 => { panic!("unimplemented"); }
      0x81 => { panic!("unimplemented"); }
      0x91 => { panic!("unimplemented"); }

      // stx
      0x86 => { panic!("unimplemented"); }
      0x96 => { panic!("unimplemented"); }
      0x8e => { panic!("unimplemented"); }

      // sty
      0x84 => { panic!("unimplemented"); }
      0x94 => { panic!("unimplemented"); }
      0x8c => { panic!("unimplemented"); }

      // # Arithmetic
      // adc
      0x69 => { panic!("unimplemented"); }
      0x65 => { panic!("unimplemented"); }
      0x75 => { panic!("unimplemented"); }
      0x6d => { panic!("unimplemented"); }
      0x7d => { panic!("unimplemented"); }
      0x79 => { panic!("unimplemented"); }
      0x61 => { panic!("unimplemented"); }
      0x71 => { panic!("unimplemented"); }

      // sbc
      0xe9 => { panic!("unimplemented"); }
      0xe5 => { panic!("unimplemented"); }
      0xf5 => { panic!("unimplemented"); }
      0xed => { panic!("unimplemented"); }
      0xfd => { panic!("unimplemented"); }
      0xf9 => { panic!("unimplemented"); }
      0xe1 => { panic!("unimplemented"); }
      0xf1 => { panic!("unimplemented"); }

      // # Comparisons
      // cmp
      0xc9 => { panic!("unimplemented"); }
      0xc5 => { panic!("unimplemented"); }
      0xd5 => { panic!("unimplemented"); }
      0xcd => { panic!("unimplemented"); }
      0xdd => { panic!("unimplemented"); }
      0xd9 => { panic!("unimplemented"); }
      0xc1 => { panic!("unimplemented"); }
      0xd1 => { panic!("unimplemented"); }

      // cpx
      0xe0 => { panic!("unimplemented"); }
      0xe4 => { panic!("unimplemented"); }
      0xec => { panic!("unimplemented"); }

      // cpy
      0xc0 => { panic!("unimplemented"); }
      0xc4 => { panic!("unimplemented"); }
      0xcc => { panic!("unimplemented"); }

      // # Bitwise operations
      // and
      0x29 => { panic!("unimplemented"); }
      0x25 => { panic!("unimplemented"); }
      0x35 => { panic!("unimplemented"); }
      0x2d => { panic!("unimplemented"); }
      0x3d => { panic!("unimplemented"); }
      0x39 => { panic!("unimplemented"); }
      0x21 => { panic!("unimplemented"); }
      0x31 => { panic!("unimplemented"); }

      // ora
      0x09 => { panic!("unimplemented"); }
      0x05 => { panic!("unimplemented"); }
      0x15 => { panic!("unimplemented"); }
      0x0d => { panic!("unimplemented"); }
      0x1d => { panic!("unimplemented"); }
      0x19 => { panic!("unimplemented"); }
      0x01 => { panic!("unimplemented"); }
      0x11 => { panic!("unimplemented"); }

      // eor
      0x49 => { panic!("unimplemented"); }
      0x45 => { panic!("unimplemented"); }
      0x55 => { panic!("unimplemented"); }
      0x4d => { panic!("unimplemented"); }
      0x5d => { panic!("unimplemented"); }
      0x59 => { panic!("unimplemented"); }
      0x41 => { panic!("unimplemented"); }
      0x51 => { panic!("unimplemented"); }

      // bit
      0x24 => { panic!("unimplemented"); }
      0x2c => { panic!("unimplemented"); }

      // # Shifts and rotates
      // rol
      0x2a => { panic!("unimplemented"); }
      0x26 => { panic!("unimplemented"); }
      0x36 => { panic!("unimplemented"); }
      0x2e => { panic!("unimplemented"); }
      0x3e => { panic!("unimplemented"); }

      // ror
      0x6a => { panic!("unimplemented"); }
      0x66 => { panic!("unimplemented"); }
      0x76 => { panic!("unimplemented"); }
      0x6e => { panic!("unimplemented"); }
      0x7e => { panic!("unimplemented"); }

      // asl
      0x0a => { panic!("unimplemented"); }
      0x06 => { panic!("unimplemented"); }
      0x16 => { panic!("unimplemented"); }
      0x0e => { panic!("unimplemented"); }
      0x1e => { panic!("unimplemented"); }

      // lsr
      0x4a => { panic!("unimplemented"); }
      0x46 => { panic!("unimplemented"); }
      0x56 => { panic!("unimplemented"); }
      0x4e => { panic!("unimplemented"); }
      0x5e => { panic!("unimplemented"); }

      // # Increments and decrements
      // inc
      0xe6 => { panic!("unimplemented"); }
      0xf6 => { panic!("unimplemented"); }
      0xee => { panic!("unimplemented"); }
      0xfe => { panic!("unimplemented"); }

      // dec
      0xc6 => { panic!("unimplemented"); }
      0xd6 => { panic!("unimplemented"); }
      0xce => { panic!("unimplemented"); }
      0xde => { panic!("unimplemented"); }

      // inx
      0xe8 => { panic!("unimplemented"); }

      // dex
      0xca => { panic!("unimplemented"); }

      // iny
      0xc8 => { panic!("unimplemented"); }

      // dey
      0x88 => { panic!("unimplemented"); }

      // # Register moves
      // tax
      0xaa => { panic!("unimplemented"); }

      // tay
      0xa8 => { panic!("unimplemented"); }

      // txa
      0x8a => { panic!("unimplemented"); }

      // tya
      0x98 => { panic!("unimplemented"); }

      // txs
      0x9a => { panic!("unimplemented"); }

      // tsx
      0xba => { panic!("unimplemented"); }

      // # Flag operations
      // clc
      0x18 => { panic!("unimplemented"); }

      // sec
      0x38 => { panic!("unimplemented"); }

      // cli
      0x58 => { panic!("unimplemented"); }

      // sei
      0x78 => { panic!("unimplemented"); }

      // clv
      0xb8 => { panic!("unimplemented"); }

      // cld
      0xd8 => { panic!("unimplemented"); }

      // sed
      0xf8 => { panic!("unimplemented"); }

      // # Branches
      // bpl
      0x10 => { panic!("unimplemented"); }

      // bmi
      0x30 => { panic!("unimplemented"); }

      // bvc
      0x50 => { panic!("unimplemented"); }

      // bvs
      0x70 => { panic!("unimplemented"); }

      // bcc
      0x90 => { panic!("unimplemented"); }

      // bcs
      0xb0 => { panic!("unimplemented"); }

      // bne
      0xd0 => { panic!("unimplemented"); }

      // beq
      0xf0 => { panic!("unimplemented"); }

      // # Jumps
      // jmp
      0x4c => { panic!("unimplemented"); }
      0x6c => { panic!("unimplemented"); },

      // # Procedure calls
      // jsr
      0x20 => { panic!("unimplemented"); }

      // rts
      0x60 => { panic!("unimplemented"); }

      // brk
      0x00 => { panic!("unimplemented"); }

      // rti
      0x40 => { panic!("unimplemented"); }

      // # Stack operations
      // pha
      0x48 => { panic!("unimplemented"); }

      // pla
      0x68 => { panic!("unimplemented"); }

      // php
      0x08 => { panic!("unimplemented"); }

      // plp
      0x28 => { panic!("unimplemented"); }

      // No operation
      // nop
      0xea => { panic!("unimplemented"); }

      _ => { panic!("unexpected opcode encountered"); }
    }
  }

  pub fn push_stack(&mut self, value: u8) {
    if self.registers.sp == 0 {
      panic!("stack overflow");
    }
    self.memory.store(STACK_LOC + self.registers.sp as u16, value);
    self.registers.sp -= 1;
  }

  pub fn peek_stack8(&mut self) -> u8 {
    self.memory.load(STACK_LOC + self.registers.sp as u16 + 1)
  }

  pub fn pop_stack(&mut self) -> u8 {
    let val = self.peek_stack8();
    self.registers.sp += 1;
    val
  }

  pub fn push_stack16(&mut self, value: u16) {
    self.memory.store16(STACK_LOC + (self.registers.sp as u16 - 1), value);
    self.registers.sp -= 2;
  }

  pub fn peek_stack16(&mut self) -> u16 {
    let lowb = self.memory.load(STACK_LOC + self.registers.sp as u16 + 1)
         as u16;
    let highb = self.memory.load(STACK_LOC + self.registers.sp as u16 + 2)
        as u16;
    lowb | (highb << 8)
  }

  pub fn pop_stack16(&mut self) -> u16 {
    let val = self.peek_stack16();
    self.registers.sp += 2;
    val
  }

  /// ## Implementation of the 6502 instruction set
  ///
  /// Any instruction that consumes additional cycles under certain conditions
  /// will return the number of conditional cycles.  This will not include
  /// cycles that can be determined simply by decoding the instruction.

  /// ## Register Transfers (TODO: tests)

  pub fn tax(&mut self) {
    self.registers.irx = self.registers.acc;
    let x = self.registers.irx;
    self.registers.set_sign_and_zero_flag(x);
  }

  pub fn tay(&mut self) {
    self.registers.iry = self.registers.acc;
    let y = self.registers.iry;
    self.registers.set_sign_and_zero_flag(y);
  }

  pub fn txa(&mut self) {
    self.registers.acc = self.registers.irx;
    let acc = self.registers.acc;
    self.registers.set_sign_and_zero_flag(acc);
  }

  pub fn tya(&mut self) {
    self.registers.acc = self.registers.iry;
    let acc = self.registers.acc;
    self.registers.set_sign_and_zero_flag(acc);
  }

  /// ## Stack Operations

  pub fn tsx(&mut self) {
    self.registers.irx = self.registers.sp;
    let x = self.registers.irx;
    self.registers.set_sign_and_zero_flag(x);
  }

  pub fn txs(&mut self) {
    self.registers.sp = self.registers.irx;
    let sp = self.registers.sp;
    self.registers.set_sign_and_zero_flag(sp);
  }

  pub fn pha(&mut self) {
    let acc = self.registers.acc;
    self.push_stack(acc);
  }

  pub fn php(&mut self) {
    let stat = self.registers.stat;
    self.push_stack(stat);
  }

  pub fn pla(&mut self) {
    let val = self.pop_stack();
    self.registers.set_acc(val);
  }

  pub fn plp(&mut self) {
    let val = self.pop_stack();
    self.registers.stat = val;
  }

  /// ## Arithmetic

  fn adc_sbc_base(&mut self, rop: u8, carry_or_borrow: isize) {
    // See http://www.righto.com/2012/12/the-6502-overflow-flag-explained.html
    let lop = self.registers.acc;

    // add using the native word size
    let res = carry_or_borrow + lop as isize + rop as isize;

    // if the operation carries into the 8th bit, carry flag will be 1,
    // and zero othersize.
    let has_carry = res & 0x100 != 0;

    let res = res as u8;

    // Set the overflow flag when both operands have the same sign bit AND
    // the sign bit of the result differs from the two.
    let has_overflow = (lop ^ rop) & 0x80 == 0 && (lop ^ res) & 0x80 != 0;

    self.registers.set_flag(FL_CARRY, has_carry);
    self.registers.set_flag(FL_OVERFLOW, has_overflow);
    self.registers.set_acc(res);
  }

  pub fn adc(&mut self, rop: u8) {
    let carry = if self.registers.get_flag(FL_CARRY) { 1 } else { 0 };
    self.adc_sbc_base(rop, carry);
  }

  pub fn sbc(&mut self, rop: u8) {
    let rop = !rop;
    let borrow = if self.registers.get_flag(FL_CARRY) { 0 } else { 1 };
    self.adc_sbc_base(rop, borrow);
  }

  fn cmp_base(&mut self, lop: u8, rop: u8) {
    let res = lop as i32 - rop as i32;
    self.registers.set_flag(FL_CARRY, res & 0x100 == 0);
    self.registers.set_sign_and_zero_flag(res as u8);
  }

  pub fn cmp(&mut self, rop: u8) {
    let lop = self.registers.acc;
    self.cmp_base(lop, rop);
  }

  pub fn cpx(&mut self, rop: u8) {
    let lop = self.registers.irx;
    self.cmp_base(lop, rop);
  }

  pub fn cpy(&mut self, rop: u8) {
    let lop = self.registers.iry;
    self.cmp_base(lop, rop);
  }

  /// ## Increments and Decrements

  pub fn inc(&mut self, addr: u16) {
    let val = self.memory.inc(addr);
    self.registers.set_sign_and_zero_flag(val);
  }

  pub fn inx(&mut self) {
    self.registers.irx = (self.registers.irx as u16 + 1) as u8;
    let x = self.registers.irx;
    self.registers.set_sign_and_zero_flag(x);
  }

  pub fn iny(&mut self) {
    self.registers.iry = (self.registers.iry as u16 + 1) as u8;
    let y = self.registers.iry;
    self.registers.set_sign_and_zero_flag(y);
  }

  pub fn dec(&mut self, addr: u16) {
    let val = self.memory.dec(addr);
    self.registers.set_sign_and_zero_flag(val);
  }

  pub fn dex(&mut self) {
    self.registers.irx = (self.registers.irx as i16 - 1) as u8;
    let x = self.registers.irx;
    self.registers.set_sign_and_zero_flag(x);
  }

  pub fn dey(&mut self) {
    self.registers.iry = (self.registers.iry as i16 - 1) as u8;
    let y = self.registers.iry;
    self.registers.set_sign_and_zero_flag(y);
  }

  /// ## Shifts
  ///
  /// All shift operations return the shifted value.  It will be up to the
  /// instruction decoder to apply the value to the accumulator or memory
  /// location.

  fn shift_left(&mut self, val: u8, lsb: bool) -> u8 {
    let carry = (val & 0x80) != 0;
    let res = if lsb { (val << 1) | 0x1 } else { val << 1 };
    self.registers.set_flag(FL_CARRY, carry);
    self.registers.set_sign_and_zero_flag(res);
    res
  }

  fn shift_right(&mut self, val: u8, msb: bool) -> u8 {
    let carry = (val & 0x1) != 0;
    let res = if msb { (val >> 1) | 0x80 } else { val >> 1 };
    self.registers.set_flag(FL_CARRY, carry);
    self.registers.set_sign_and_zero_flag(res);
    res
  }

  pub fn asl(&mut self, val: u8) -> u8 {
    self.shift_left(val, false)
  }

  pub fn lsr(&mut self, val: u8) -> u8 {
    self.shift_right(val, false)
  }

  pub fn rol(&mut self, val: u8) -> u8 {
    let carry_set = self.registers.get_flag(FL_CARRY);
    self.shift_left(val, carry_set)
  }

  pub fn ror(&mut self, val: u8) -> u8 {
    let carry_set = self.registers.get_flag(FL_CARRY);
    self.shift_right(val, carry_set)
  }

  /// ## Jumps and Calls

  pub fn jmp(&mut self, loc: u16) {
    self.registers.pc = loc;
  }

  pub fn jsr(&mut self, loc: u16) {
    let pc = self.registers.pc;
    self.push_stack16(pc - 1);
    self.registers.pc = loc;
  }

  pub fn rts(&mut self) {
    self.registers.pc = self.pop_stack16() + 1;
  }

  /// ##  Branches

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

  /// Status Flag Changes

  pub fn clc(&mut self) {
    self.registers.set_flag(FL_CARRY, false);
  }

  pub fn cld(&mut self) {
    panic!("Not implemented by Nintendo's 6502");
  }

  pub fn cli(&mut self) {
    self.registers.set_flag(FL_INTERRUPT_DISABLE, false);
  }

  pub fn clv(&mut self) {
    self.registers.set_flag(FL_OVERFLOW, false);
  }

  pub fn sec(&mut self) {
    self.registers.set_flag(FL_CARRY, true);
  }

  pub fn sed(&mut self) {
    panic!("Not implemented by Nintendo's 6502");
  }

  pub fn sei(&mut self) {
    self.registers.set_flag(FL_INTERRUPT_DISABLE, true);
  }

  /// ## Load/Store Operations

  pub fn lda(&mut self, val: u8) {
    self.registers.set_acc(val);
  }

  pub fn ldx(&mut self, val: u8) {
    self.registers.irx = val;
    self.registers.set_sign_and_zero_flag(val);
  }

  pub fn ldy(&mut self, val: u8) {
    self.registers.iry = val;
    self.registers.set_sign_and_zero_flag(val);
  }

  pub fn sta(&mut self, addr: u16) {
    self.memory.store(addr, self.registers.acc);
  }

  pub fn stx(&mut self, addr: u16) {
    self.memory.store(addr, self.registers.irx);
  }

  pub fn sty(&mut self, addr: u16) {
    self.memory.store(addr, self.registers.iry);
  }

  /// ## Logical (todo: tests)

  pub fn and(&mut self, rop: u8) {
    let lop = self.registers.acc;
    let res = lop & rop;
    self.registers.set_acc(res);
  }

  pub fn eor(&mut self, rop: u8) {
    let lop = self.registers.acc;
    let res = lop ^ rop;
    self.registers.set_acc(res);
  }

  pub fn ora(&mut self, rop: u8) {
    let lop = self.registers.acc;
    let res = lop | rop;
    self.registers.set_acc(res);
  }

  pub fn bit(&mut self, rop: u8) {
    let lop = self.registers.acc;
    let res = lop & rop;
    self.registers.set_sign_and_zero_flag(res);
    self.registers.set_flag(FL_OVERFLOW, res & 0x40 != 0);
  }

  /// ## System Functions

  pub fn brk(&mut self) {
    panic!("unimplemented");
  }

  pub fn nop(&mut self) {
    panic!("unimplemented");
  }

  pub fn rti(&mut self) {
    panic!("unimplemented");
  }
}
