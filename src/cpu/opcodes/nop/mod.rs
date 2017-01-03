use super::OpCode;

pub struct Nop;

impl OpCode for Nop {
    type Input = ();
}
