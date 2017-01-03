use super::AddressingMode;
use memory::Memory;

pub struct Accumulator;

impl<M: Memory> AddressingMode<M> for Accumulator {
    type Output = u8;
}
