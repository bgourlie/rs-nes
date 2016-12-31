use super::AddressingMode;
use cpu::Cpu;
use memory::Memory;

pub struct Indirect;

impl<M: Memory> AddressingMode<M> for Indirect {
    fn operand<F: Fn(&Cpu<M>)>(&mut self, _: &mut Cpu<M>, _: F) -> u8 {
        unimplemented!()
    }
}
