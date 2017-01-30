// TODO: Remove, eventually...
 #![allow(dead_code)]

#[cfg(test)]
mod spec_tests;

use std::fmt;

const FL_CARRY: u8 = 0b00000001;
const FL_ZERO: u8 = 0b00000010;
const FL_INTERRUPT_DISABLE: u8 = 0b00000100;
const FL_DECIMAL: u8 = 0b00001000;
const FL_BREAK: u8 = 0b00010000;
const FL_UNUSED: u8 = 0b00100000;
const FL_OVERFLOW: u8 = 0b01000000;
const FL_SIGN: u8 = 0b10000000;

#[derive(Clone, Serialize)]
pub struct Registers {
    pub pc: u16, // Program Counter
    pub sp: u8, // Stack Pointer
    pub acc: u8, // Accumulator
    pub x: u8, // Index Register X
    pub y: u8, // Index Register Y
    pub status: u8, // Processor Status Flags
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            pc: 0,
            // http://www.pagetable.com/?p=410 explains why the stack pointer has
            // an initial value of 0xfd.
            sp: 0xfd,
            acc: 0,
            x: 0,
            y: 0,
            status: 0b00100100,
        }
    }

    pub fn status_for_stack(&self) -> u8 {
        self.status | FL_BREAK | FL_UNUSED
    }

    pub fn set_status_from_stack(&mut self, val: u8) {
        let old_status = self.status;
        self.status = val | (old_status & (FL_BREAK | FL_UNUSED));
    }

    pub fn carry_flag(&self) -> bool {
        self.status & FL_CARRY != 0
    }

    pub fn set_carry_flag(&mut self, val: bool) {
        if val {
            self.status |= FL_CARRY;
        } else {
            self.status &= !FL_CARRY;
        }
    }

    pub fn zero_flag(&self) -> bool {
        self.status & FL_ZERO != 0
    }

    pub fn set_zero_flag(&mut self, val: bool) {
        if val {
            self.status |= FL_ZERO;
        } else {
            self.status &= !FL_ZERO;
        }
    }

    pub fn interrupt_disable_flag(&self) -> bool {
        self.status & FL_INTERRUPT_DISABLE != 0
    }

    pub fn set_interrupt_disable_flag(&mut self, val: bool) {
        if val {
            self.status |= FL_INTERRUPT_DISABLE;
        } else {
            self.status &= !FL_INTERRUPT_DISABLE;
        }
    }

    pub fn decimal_flag(&self) -> bool {
        self.status & FL_DECIMAL != 0
    }

    pub fn set_decimal_flag(&mut self, val: bool) {
        if val {
            self.status |= FL_DECIMAL;
        } else {
            self.status &= !FL_DECIMAL;
        }
    }

    pub fn break_flag(&self) -> bool {
        self.status & FL_BREAK != 0
    }

    pub fn overflow_flag(&self) -> bool {
        self.status & FL_OVERFLOW != 0
    }

    pub fn set_overflow_flag(&mut self, val: bool) {
        if val {
            self.status |= FL_OVERFLOW;
        } else {
            self.status &= !FL_OVERFLOW;
        }
    }

    pub fn sign_flag(&self) -> bool {
        self.status & FL_SIGN != 0
    }

    pub fn set_sign_flag(&mut self, val: bool) {
        if val {
            self.status |= FL_SIGN;
        } else {
            self.status &= !FL_SIGN;
        }
    }

    pub fn set_sign_and_zero_flag(&mut self, val: u8) {
        self.set_sign_flag(val & 0x80 != 0);
        self.set_zero_flag(val == 0);
    }

    pub fn set_acc(&mut self, res: u8) {
        self.set_sign_and_zero_flag(res);
        self.acc = res;
    }
}

impl Default for Registers {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Registers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = if self.status & 0x1 > 0 { 1 } else { 0 };
        let z = if self.status & 0x2 > 0 { 1 } else { 0 };
        let i = if self.status & 0x4 > 0 { 1 } else { 0 };
        let d = if self.status & 0x8 > 0 { 1 } else { 0 };
        let b = if self.status & 0x10 > 0 { 1 } else { 0 };
        let unused = if self.status & 0x20 > 0 { 1 } else { 0 };
        let v = if self.status & 0x40 > 0 { 1 } else { 0 };
        let s = if self.status & 0x80 > 0 { 1 } else { 0 };

        write!(f,
               "PC:{:0>4X} SP:{:0>2X} A:{:0>2X} X:{:0>2X} Y:{:0>2X} Stat: {:0>2X} s{} v{} _{} b{} \
                d{} i{} z{} c{}",
               self.pc,
               self.sp,
               self.acc,
               self.x,
               self.y,
               self.status,
               s,
               v,
               unused,
               b,
               d,
               i,
               z,
               c)
    }
}
