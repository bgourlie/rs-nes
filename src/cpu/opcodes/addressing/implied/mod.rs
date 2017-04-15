use cpu::opcodes::addressing::AddressingMode;
use memory::Memory;
use screen::Screen;

pub struct Implied;

impl<S: Screen, M: Memory<S>> AddressingMode<S, M> for Implied {
    type Output = ();

    fn read(&self) -> Self::Output {
        ()
    }
}
