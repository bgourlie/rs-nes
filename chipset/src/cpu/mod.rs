#[cfg(test)]
mod adc_spec_tests;

#[cfg(test)]
mod branching_spec_tests;

#[cfg(test)]
mod cmp_spec_tests;

#[cfg(test)]
mod inc_and_dec_spec_tests;

#[cfg(test)]
mod jumps_and_calls_spec_tests;

#[cfg(test)]
mod lda_spec_tests;

#[cfg(test)]
mod sbc_spec_tests;

#[cfg(test)]
mod shifts_spec_tests;

#[cfg(test)]
mod stack_utils_spec_tests;

#[cfg(test)]
mod status_flag_spec_tests;

#[cfg(test)]
mod store_spec_tests;

mod registers;

use constants::*;
use cpu::registers::*;
use memory::Memory;

// Graciously taken from FCEU
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

// TODO: consolidate logic with similar implementation in Register
fn get_page_crossed(val1: u16, val2: u16) -> bool {
  val1 & 0xFF00 != val2 & 0xFF00
}

pub struct Cpu6502 {
  pub cycles: u64,
  pub registers: Registers,
  pub memory: Memory
}

impl Cpu6502 {
  pub fn new() -> Cpu6502 {
    Cpu6502 {
      cycles: 0,
      registers: Registers::new(),
      memory: Memory::new()
    }
  }

  pub fn reset(&mut self) {
    let pc_start = self.memory.load16(RESET_VECTOR);
    self.registers.pc = pc_start;
  }

  pub fn nmi(&mut self) {
    let (pc, stat) = (self.registers.pc, self.registers.stat);
    self.push_stack16(pc);
    self.push_stack(stat);
    self.registers.pc = self.memory.load16(NMI_VECTOR);
  }

  pub fn step(&mut self) {
    let op = self.read_op();
    let cycles = self.do_op(op);
    self.cycles += cycles as u64;
  }

  fn read_op(&mut self) -> u8 {
    let pc = self.registers.pc;
    let operand = self.memory.load(pc);
    self.registers.pc += 1;
    operand
  }

  fn read_op16(&mut self) -> u16 {
    let pc = self.registers.pc;
    let operand = self.memory.load16(pc);
    self.registers.pc += 2;
    operand
  }

  fn get_immed(&mut self) -> u8 {
    self.read_op()
  }

  fn get_zp(&mut self) -> u8 {
    let addr = self.read_op() as u16;
    self.memory.load(addr)
  }

  fn get_zp16(&mut self) -> u16 {
    let addr = self.read_op() as u16;
    self.memory.load16(addr)
  }

  fn get_zpx(&mut self) -> u8 {
    let addr = self.read_op();
    self.memory.load_zp_indexed(addr, self.registers.irx)
  }

  fn get_zpx16(&mut self) -> u16 {
    let addr = self.read_op();
    self.memory.load16_zp_indexed(addr, self.registers.irx)
  }

  fn get_zpy(&mut self) -> u8 {
    let addr = self.read_op();
    self.memory.load_zp_indexed(addr, self.registers.iry)
  }

  fn get_zpy16(&mut self) -> u16 {
    let addr = self.read_op();
    self.memory.load16_zp_indexed(addr, self.registers.iry)
  }

  fn get_abs(&mut self) -> u8 {
    let addr = self.read_op16();
    self.memory.load(addr)
  }

  fn get_abs16(&mut self) -> u16 {
    let addr = self.read_op16();
    self.memory.load16(addr)
  }

  fn get_abs_indexed_base(&mut self, index: u8) -> (u8, bool) {
    let abs = self.read_op16();
    let addr = abs + index as u16;

    // TODO: do we check that there is a page crossed when adding
    // the register to the absolute address?  That's what we're assuming now.
    (self.memory.load(addr), get_page_crossed(abs, addr))
  }

  fn get_abs_indexed_base16(&mut self, index: u8) -> (u16, bool) {
    let abs = self.read_op16();
    let addr = abs + index as u16;

    // TODO: do we check that there is a page crossed when adding
    // the register to the absolute address?  That's what we're assuming now.
    (self.memory.load16(addr), get_page_crossed(abs, addr))
  }

  fn get_absx(&mut self) -> (u8, bool) {
    let x = self.registers.irx;
    self.get_abs_indexed_base(x)
  }

  fn get_absx16(&mut self) -> (u16, bool) {
    let x = self.registers.irx;
    self.get_abs_indexed_base16(x)
  }

  fn get_absy(&mut self) -> (u8, bool) {
    let y = self.registers.iry;
    self.get_abs_indexed_base(y)
  }

  fn get_absy16(&mut self) -> (u16, bool) {
    let y = self.registers.iry;
    self.get_abs_indexed_base16(y)
  }

  fn get_indx(&mut self) -> u8 {
    let val = self.read_op();
    let x = self.registers.irx;
    let addr = self.memory.load16_zp_indexed(val, x);
    self.memory.load(addr)
  }

  fn get_indx16(&mut self) -> u16 {
    let val = self.read_op();
    let x = self.registers.irx;
    let addr = self.memory.load16_zp_indexed(val, x);
    self.memory.load16(addr)
  }

  fn get_indy(&mut self) -> (u8, bool) {
    let val = self.read_op();
    let y = self.registers.iry;
    let addr = self.memory.load16(val as u16) + y as u16;

    // TODO: is this the correct way to determine if page is crossed?
    let page_boundary_crossed = get_page_crossed(val as u16, addr);
    (self.memory.load(addr), page_boundary_crossed)
  }

  fn get_indy16(&mut self) -> (u16, bool) {
    let val = self.read_op();
    let y = self.registers.iry;
    let addr = self.memory.load16(val as u16) + y as u16;

    // TODO: is this the correct way to determine if page is crossed?
    let page_boundary_crossed = get_page_crossed(val as u16, addr);
    (self.memory.load16(addr), page_boundary_crossed)
  }

  // performs an operation, returns number of cycles consumed
  fn do_op(&mut self, opcode: u8) -> u8 {
    let mut cycles = CYCLE_TABLE[opcode as usize];
    match opcode {
      // # Loads
      // lda
      0xa1 => { let val = self.get_indx(); self.lda(val); }
      0xa5 => { let val = self.get_zp(); self.lda(val); }
      0xa9 => { let val = self.get_immed(); self.lda(val); }
      0xad => { let val = self.get_abs(); self.lda(val); }
      0xb1 => {
        let (val, page_crossed) = self.get_indy();
        self.lda(val);
        if page_crossed { cycles += 1; }
      }
      0xb5 => { let val = self.get_zpx(); self.lda(val); }
      0xb9 => {
        let (val, page_crossed) = self.get_absy();
        self.lda(val);
        if page_crossed { cycles += 1; }
      }
      0xbd => {
        let (val, page_crossed) = self.get_absx();
        self.lda(val);
        if page_crossed { cycles += 1; }
      }

      // ldx
      0xa2 => { let val = self.get_immed(); self.ldx(val); }
      0xa6 => { let val = self.get_zp(); self.ldx(val); }
      0xb6 => { let val = self.get_zpy(); self.ldx(val); }
      0xae => { let val = self.get_abs(); self.ldx(val); }
      0xbe => {
        let (val, page_crossed) = self.get_absy();
        self.ldx(val);
        if page_crossed { cycles += 1; }
      }

      // ldy
      0xa0 => { let val = self.get_immed(); self.ldy(val); }
      0xa4 => { let val = self.get_zp(); self.ldy(val); }
      0xb4 => { let val = self.get_zpx(); self.ldy(val); }
      0xac => { let val = self.get_abs(); self.ldy(val); }
      0xbc => { 
        let (val, page_crossed) = self.get_absx();
        self.ldy(val);
        if page_crossed { cycles += 1; }
      }

      // # Stores
      // sta
      0x85 => { let val = self.get_zp16(); self.sta(val); }
      0x95 => { let val = self.get_zpx16(); self.sta(val); }
      0x8d => { let val = self.get_abs16(); self.sta(val); }
      0x9d => { let (val, _) = self.get_absx16(); self.sta(val); }
      0x99 => { let (val, _) = self.get_absy16(); self.sta(val); }
      0x81 => { let val = self.get_indx16(); self.sta(val); }
      0x91 => { let (val, _) = self.get_indy16(); self.sta(val); }

      // stx
      0x86 => { let val = self.get_zp16(); self.stx(val); }
      0x96 => { let val = self.get_zpy16(); self.stx(val); }
      0x8e => { let val = self.get_abs16(); self.stx(val); }

      // sty
      0x84 => { let val = self.get_zp16(); self.sty(val); }
      0x94 => { let val = self.get_zpx16(); self.sty(val); }
      0x8c => { let val = self.get_abs16(); self.sty(val); }

      // # Arithmetic
      // adc
      0x69 => { let val = self.get_immed(); self.adc(val); }
      0x65 => { let val = self.get_zp(); self.adc(val); }
      0x75 => { let val = self.get_zpx(); self.adc(val); }
      0x6d => { let val = self.get_abs(); self.adc(val); }
      0x7d => { 
        let (val, page_crossed) = self.get_absx(); 
        self.adc(val);
        if page_crossed { cycles += 1; }
      }
      0x79 => {
        let (val, page_crossed) = self.get_absy();
        self.adc(val);
        if page_crossed { cycles += 1 }
      }
      0x61 => { let val = self.get_indx(); self.adc(val); }
      0x71 => {
        let (val, page_crossed) = self.get_indy(); 
        self.adc(val);
        if page_crossed { cycles += 1; }
      }

      // sbc
      0xe9 => { let val = self.get_immed(); self.sbc(val); }
      0xe5 => { let val = self.get_zp(); self.sbc(val); }
      0xf5 => { let val = self.get_zpx(); self.sbc(val); }
      0xed => { let val = self.get_abs(); self.sbc(val); }
      0xfd => { 
        let (val, page_crossed) = self.get_absx();
        self.sbc(val);
        if page_crossed { cycles += 1; }
      }
      0xf9 => {
        let (val, page_crossed) = self.get_absy();
        self.sbc(val);
        if page_crossed { cycles += 1; }
      }
      0xe1 => { let val = self.get_indx(); self.sbc(val); }
      0xf1 => { 
        let (val, page_crossed) = self.get_indy(); 
        self.sbc(val); 
        if page_crossed { cycles += 1; }
      }

      // # Comparisons
      // cmp
      0xc9 => { let val = self.get_immed(); self.cmp(val); }
      0xc5 => { let val = self.get_zp(); self.cmp(val); }
      0xd5 => { let val = self.get_zpx(); self.cmp(val); }
      0xcd => { let val = self.get_abs(); self.cmp(val); }
      0xdd => { 
        let (val, page_crossed) = self.get_absx(); 
        self.cmp(val); 
        if page_crossed { cycles += 1; }
      }
      0xd9 => {
        let (val, page_crossed) = self.get_absy();
        self.cmp(val);
        if page_crossed { cycles += 1; }
      }
      0xc1 => { let val = self.get_indx(); self.cmp(val); }
      0xd1 => {
        let (val, page_crossed) = self.get_indy();
        self.cmp(val);
        if page_crossed { cycles += 1; }
      }

      // cpx
      0xe0 => { let val = self.get_immed(); self.cpx(val); }
      0xe4 => { let val = self.get_zp(); self.cpx(val); }
      0xec => { let val = self.get_abs(); self.cpx(val); }

      // cpy
      0xc0 => { let val = self.get_immed(); self.cpy(val); }
      0xc4 => { let val = self.get_zp(); self.cpy(val); }
      0xcc => { let val = self.get_abs(); self.cpy(val); }

      // # Bitwise operations
      // and
      0x29 => { let val = self.get_immed(); self.and(val); }
      0x25 => { let val = self.get_zp(); self.and(val); }
      0x35 => { let val = self.get_zpx(); self.and(val); }
      0x2d => { let val = self.get_abs(); self.and(val); }
      0x3d => { 
        let (val, page_crossed) = self.get_absx();
        self.and(val);
        if page_crossed { cycles += 1; }
      }
      0x39 => {
        let (val, page_crossed) = self.get_absy();
        self.and(val);
        if page_crossed { cycles += 1; }
      }
      0x21 => { let val = self.get_indx(); self.and(val); }
      0x31 => {
        let (val, page_crossed) = self.get_indy();
        self.and(val);
        if page_crossed { cycles += 1; }
      }

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

    cycles
  }

  fn push_stack(&mut self, value: u8) {
    if self.registers.sp == 0 {
      panic!("stack overflow");
    }
    self.memory.store(STACK_LOC + self.registers.sp as u16, value);
    self.registers.sp -= 1;
  }

  fn peek_stack(&mut self) -> u8 {
    self.memory.load(STACK_LOC + self.registers.sp as u16 + 1)
  }

  fn pop_stack(&mut self) -> u8 {
    let val = self.peek_stack();
    self.registers.sp += 1;
    val
  }

  fn push_stack16(&mut self, value: u16) {
    if self.registers.sp < 2 {
      panic!("stack overflow");
    }
    self.memory.store16(STACK_LOC + (self.registers.sp as u16 - 1), value);
    self.registers.sp -= 2;
  }

  fn peek_stack16(&mut self) -> u16 {
    let lowb = self.memory.load(STACK_LOC + self.registers.sp as u16 + 1)
         as u16;
    let highb = self.memory.load(STACK_LOC + self.registers.sp as u16 + 2)
        as u16;
    lowb | (highb << 8)
  }

  fn pop_stack16(&mut self) -> u16 {
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

  fn tax(&mut self) {
    self.registers.irx = self.registers.acc;
    let x = self.registers.irx;
    self.registers.set_sign_and_zero_flag(x);
  }

  fn tay(&mut self) {
    self.registers.iry = self.registers.acc;
    let y = self.registers.iry;
    self.registers.set_sign_and_zero_flag(y);
  }

  fn txa(&mut self) {
    self.registers.acc = self.registers.irx;
    let acc = self.registers.acc;
    self.registers.set_sign_and_zero_flag(acc);
  }

  fn tya(&mut self) {
    self.registers.acc = self.registers.iry;
    let acc = self.registers.acc;
    self.registers.set_sign_and_zero_flag(acc);
  }

  /// ## Stack Operations

  fn tsx(&mut self) {
    self.registers.irx = self.registers.sp;
    let x = self.registers.irx;
    self.registers.set_sign_and_zero_flag(x);
  }

  fn txs(&mut self) {
    self.registers.sp = self.registers.irx;
    let sp = self.registers.sp;
    self.registers.set_sign_and_zero_flag(sp);
  }

  fn pha(&mut self) {
    let acc = self.registers.acc;
    self.push_stack(acc);
  }

  fn php(&mut self) {
    let stat = self.registers.stat;
    self.push_stack(stat);
  }

  fn pla(&mut self) {
    let val = self.pop_stack();
    self.registers.set_acc(val);
  }

  fn plp(&mut self) {
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

  fn adc(&mut self, rop: u8) {
    let carry = if self.registers.get_flag(FL_CARRY) { 1 } else { 0 };
    self.adc_sbc_base(rop, carry);
  }

  fn sbc(&mut self, rop: u8) {
    let rop = !rop;
    let borrow = if self.registers.get_flag(FL_CARRY) { 0 } else { 1 };
    self.adc_sbc_base(rop, borrow);
  }

  fn cmp_base(&mut self, lop: u8, rop: u8) {
    let res = lop as i32 - rop as i32;
    self.registers.set_flag(FL_CARRY, res & 0x100 == 0);
    self.registers.set_sign_and_zero_flag(res as u8);
  }

  fn cmp(&mut self, rop: u8) {
    let lop = self.registers.acc;
    self.cmp_base(lop, rop);
  }

  fn cpx(&mut self, rop: u8) {
    let lop = self.registers.irx;
    self.cmp_base(lop, rop);
  }

  fn cpy(&mut self, rop: u8) {
    let lop = self.registers.iry;
    self.cmp_base(lop, rop);
  }

  /// ## Increments and Decrements

  fn inc(&mut self, addr: u16) {
    let val = self.memory.inc(addr);
    self.registers.set_sign_and_zero_flag(val);
  }

  fn inx(&mut self) {
    self.registers.irx = (self.registers.irx as u16 + 1) as u8;
    let x = self.registers.irx;
    self.registers.set_sign_and_zero_flag(x);
  }

  fn iny(&mut self) {
    self.registers.iry = (self.registers.iry as u16 + 1) as u8;
    let y = self.registers.iry;
    self.registers.set_sign_and_zero_flag(y);
  }

  fn dec(&mut self, addr: u16) {
    let val = self.memory.dec(addr);
    self.registers.set_sign_and_zero_flag(val);
  }

  fn dex(&mut self) {
    self.registers.irx = (self.registers.irx as i16 - 1) as u8;
    let x = self.registers.irx;
    self.registers.set_sign_and_zero_flag(x);
  }

  fn dey(&mut self) {
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

  fn asl(&mut self, val: u8) -> u8 {
    self.shift_left(val, false)
  }

  fn lsr(&mut self, val: u8) -> u8 {
    self.shift_right(val, false)
  }

  fn rol(&mut self, val: u8) -> u8 {
    let carry_set = self.registers.get_flag(FL_CARRY);
    self.shift_left(val, carry_set)
  }

  fn ror(&mut self, val: u8) -> u8 {
    let carry_set = self.registers.get_flag(FL_CARRY);
    self.shift_right(val, carry_set)
  }

  /// ## Jumps and Calls

  fn jmp(&mut self, loc: u16) {
    self.registers.pc = loc;
  }

  fn jsr(&mut self, loc: u16) {
    let pc = self.registers.pc;
    self.push_stack16(pc - 1);
    self.registers.pc = loc;
  }

  fn rts(&mut self) {
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

  fn bcc(&mut self, rel_addr: i8) -> u8 {
    let carry_clear = !self.registers.get_flag(FL_CARRY);
    self.branch(carry_clear, rel_addr)
  }

  fn bcs(&mut self, rel_addr: i8) -> u8 {
    let carry_set = self.registers.get_flag(FL_CARRY);
    self.branch(carry_set, rel_addr)
  }

  fn beq(&mut self, rel_addr: i8) -> u8 {
    let zero_set = self.registers.get_flag(FL_ZERO);
    self.branch(zero_set, rel_addr)
  }

  fn bmi(&mut self, rel_addr: i8) -> u8 {
    let sign_set = self.registers.get_flag(FL_SIGN);
    self.branch(sign_set, rel_addr)
  }

  fn bne(&mut self, rel_addr: i8) -> u8 {
    let zero_clear = !self.registers.get_flag(FL_ZERO);
    self.branch(zero_clear, rel_addr)
  }

  fn bpl(&mut self, rel_addr: i8) -> u8 {
    let sign_clear = !self.registers.get_flag(FL_SIGN);
    self.branch(sign_clear, rel_addr)
  }

  fn bvc(&mut self, rel_addr: i8) -> u8 {
    let overflow_clear = !self.registers.get_flag(FL_OVERFLOW);
    self.branch(overflow_clear, rel_addr)
  }

  fn bvs(&mut self, rel_addr: i8) -> u8 {
    let overflow_set = self.registers.get_flag(FL_OVERFLOW);
    self.branch(overflow_set, rel_addr)
  }

  /// Status Flag Changes

  fn clc(&mut self) {
    self.registers.set_flag(FL_CARRY, false);
  }

  fn cld(&mut self) {
    panic!("Not implemented by Nintendo's 6502");
  }

  fn cli(&mut self) {
    self.registers.set_flag(FL_INTERRUPT_DISABLE, false);
  }

  fn clv(&mut self) {
    self.registers.set_flag(FL_OVERFLOW, false);
  }

  fn sec(&mut self) {
    self.registers.set_flag(FL_CARRY, true);
  }

  fn sed(&mut self) {
    panic!("Not implemented by Nintendo's 6502");
  }

  fn sei(&mut self) {
    self.registers.set_flag(FL_INTERRUPT_DISABLE, true);
  }

  /// ## Load/Store Operations

  fn lda(&mut self, val: u8) {
    self.registers.set_acc(val);
  }

  fn ldx(&mut self, val: u8) {
    self.registers.irx = val;
    self.registers.set_sign_and_zero_flag(val);
  }

  fn ldy(&mut self, val: u8) {
    self.registers.iry = val;
    self.registers.set_sign_and_zero_flag(val);
  }

  fn sta(&mut self, addr: u16) {
    self.memory.store(addr, self.registers.acc);
  }

  fn stx(&mut self, addr: u16) {
    self.memory.store(addr, self.registers.irx);
  }

  fn sty(&mut self, addr: u16) {
    self.memory.store(addr, self.registers.iry);
  }

  /// ## Logical (todo: tests)

  fn and(&mut self, rop: u8) {
    let lop = self.registers.acc;
    let res = lop & rop;
    self.registers.set_acc(res);
  }

  fn eor(&mut self, rop: u8) {
    let lop = self.registers.acc;
    let res = lop ^ rop;
    self.registers.set_acc(res);
  }

  fn ora(&mut self, rop: u8) {
    let lop = self.registers.acc;
    let res = lop | rop;
    self.registers.set_acc(res);
  }

  fn bit(&mut self, rop: u8) {
    let lop = self.registers.acc;
    let res = lop & rop;
    self.registers.set_sign_and_zero_flag(res);
    self.registers.set_flag(FL_OVERFLOW, res & 0x40 != 0);
  }

  /// ## System Functions (todo: tests)

  fn brk(&mut self) {
    let pc = self.registers.pc;
    let status = self.registers.stat;
    self.push_stack16(pc + 1);
    self.push_stack(status);
    let irq_handler = self.memory.load16(BRK_VECTOR);
    self.registers.pc = irq_handler;
    self.registers.set_flag(FL_BRK, true);
  }

  fn nop(&mut self) { }

  fn rti(&mut self) {
    let stat = self.pop_stack();
    let pc = self.pop_stack16();
    self.registers.stat = stat;
    self.registers.pc = pc;
  }
}
