use super::AddressingMode;
use memory::Memory;

pub struct Indirect;

impl<M: Memory> AddressingMode<M> for Indirect {
    type Output = u16;

    fn read(&self) -> Self::Output {
        unimplemented!()
    }
}
