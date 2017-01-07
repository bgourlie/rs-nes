use super::AddressingMode;
use memory::Memory;

pub struct IndirectIndexed;

impl IndirectIndexed {
    pub fn new<F: Fn(&Cpu<M>), M: Memory>(cpu: &mut Cpu<M>, tick_handler: F) -> Self {}
}

impl<M: Memory> AddressingMode<M> for IndirectIndexed {
    type Output = u8;

    fn read(&self) -> Self::Output {
        unimplemented!()
    }
}
