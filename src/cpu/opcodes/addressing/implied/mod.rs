use cpu::opcodes::addressing::AddressingMode;
use memory::Memory;

pub struct Implied;

impl<M: Memory> AddressingMode<M> for Implied {
    type Output = ();

    fn read(&self) -> Self::Output {
        ()
    }
}
