use memory::SimpleMemory;
use cpu::Cpu;
use super::AddressingMode;

impl AddressingMode<SimpleMemory> for u8 {
    fn operand<F: Fn(&Cpu<SimpleMemory>)>(&mut self, _: &mut Cpu<SimpleMemory>, _: F) -> u8 {
        *self
    }
}
