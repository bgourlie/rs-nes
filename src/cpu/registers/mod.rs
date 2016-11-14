use std::fmt;

use constants::*;

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

    pub fn get_flag(&self, mask: u8) -> bool {
        self.status & mask != 0
    }

    pub fn set_flag(&mut self, mask: u8, val: bool) {
        if val {
            self.status |= mask;
        } else {
            self.status &= !mask;
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
