use cpu::Cpu;
use memory::Memory;
use super::AddressingMode;

pub struct IndexedIndirect;

impl IndexedIndirect {
    pub fn new<F: Fn(&Cpu<M>), M: Memory>(_: &mut Cpu<M>, _: F) -> Self {
        IndexedIndirect
    }
}

impl<M: Memory> AddressingMode<M> for IndexedIndirect {
    type Output = u8;

    fn read(&self) -> Self::Output {
        unimplemented!()
    }

    fn write<F: Fn(&Cpu<M>)>(&self, _: &mut Cpu<M>, _: u8, _: F) {
        unimplemented!()
    }
}
