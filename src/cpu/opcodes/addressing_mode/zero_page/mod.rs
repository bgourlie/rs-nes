use super::AddressingMode;
use cpu::Cpu;
use memory::Memory;

pub struct ZeroPage {
    addr: u16,
    value: u8,
}

impl ZeroPage {
    pub fn new<F: Fn(&Cpu<M>), M: Memory>(_: &mut Cpu<M>, _: F) -> Self {
        unimplemented!();
    }
}

impl<M: Memory> AddressingMode<M> for ZeroPage {
    type Output = u8;
}
