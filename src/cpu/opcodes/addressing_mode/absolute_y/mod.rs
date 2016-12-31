use super::AddressingMode;
use cpu::Cpu;
use memory::Memory;

#[derive(Default)]
pub struct AbsoluteY;

impl<M: Memory> AddressingMode<M> for AbsoluteY {
    fn operand<F: Fn(&Cpu<M>)>(&mut self, _: &mut Cpu<M>, _: F) -> u8 {
        unimplemented!()
    }
}
