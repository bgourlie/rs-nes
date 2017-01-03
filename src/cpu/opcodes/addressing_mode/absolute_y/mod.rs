use super::AddressingMode;
use memory::Memory;

#[derive(Default)]
pub struct AbsoluteY;

impl<M: Memory> AddressingMode<M> for AbsoluteY {
    type Output = u8;
}
