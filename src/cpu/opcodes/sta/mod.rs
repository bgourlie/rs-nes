use super::OpCode;

pub struct Sta;

impl OpCode for Sta {
    type Input = u8;
}
