use memory::SimpleMemory;
use super::AddressingMode;

impl AddressingMode<SimpleMemory> for u8 {
    fn operand(&self) -> u8 {
        *self
    }
}
