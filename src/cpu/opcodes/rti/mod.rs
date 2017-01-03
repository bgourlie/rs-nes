use super::OpCode;

pub struct Rti;

impl OpCode for Rti {
    type Input = ();
}
