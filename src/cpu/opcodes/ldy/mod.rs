use super::OpCode;

pub struct Ldy;

impl OpCode for Ldy {
    type Input = u8;
}
