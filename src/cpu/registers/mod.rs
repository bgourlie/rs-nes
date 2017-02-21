// TODO: Remove, eventually...
 #![allow(dead_code)]

#[cfg(test)]
mod spec_tests;

const FL_CARRY: u8 = 0b00000001;
const FL_ZERO: u8 = 0b00000010;
const FL_INTERRUPT_DISABLE: u8 = 0b00000100;
const FL_DECIMAL: u8 = 0b00001000;
const FL_BREAK: u8 = 0b00010000;
const FL_UNUSED: u8 = 0b00100000;
const FL_OVERFLOW: u8 = 0b01000000;
const FL_SIGN: u8 = 0b10000000;
const FL_ALWAYS_SET: u8 = FL_UNUSED | FL_BREAK;

#[derive(Clone)]
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
            status: FL_ALWAYS_SET | FL_INTERRUPT_DISABLE,
        }
    }

    pub fn set_status_from_stack(&mut self, val: u8) {
        self.status = val | FL_ALWAYS_SET;
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

    pub fn status_sans_break(&self) -> u8 {
        self.status & !FL_BREAK
    }
}

impl Default for Registers {
    fn default() -> Self {
        Self::new()
    }
}
