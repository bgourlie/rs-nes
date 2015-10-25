pub const STACK_LOC: u16 = 0x100;

pub const BRK_VECTOR: u16 = 0xfffe;
pub const NMI_VECTOR: u16 = 0xfffa;
pub const RESET_VECTOR: u16 = 0xfffc;

pub const FL_CARRY: u8 = 0b00000001;
pub const FL_ZERO: u8 = 0b00000010;
pub const FL_INTERRUPT_DISABLE: u8 = 0b00000100;
pub const FL_DECIMAL: u8 = 0b00001000;
pub const FL_BRK: u8 = 0b00010000;
pub const FL_UNUSED: u8 = 0b00100000;
pub const FL_OVERFLOW: u8 = 0b01000000;
pub const FL_SIGN: u8 = 0b10000000;
