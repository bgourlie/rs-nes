use super::OpCode;

pub struct Jsr;

impl OpCode for Jsr {
    type Input = u8;
}
