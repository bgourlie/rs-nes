pub mod nes_memory;
mod simple_memory;

pub use self::simple_memory::SimpleMemory;
use cpu::Interrupt;
use input::Input;
use screen::Screen;
use std::io::Write;

pub const ADDRESSABLE_MEMORY: usize = 65_536;

pub trait Memory<I: Input, S: Screen> {
    fn tick(&mut self) -> Interrupt {
        Interrupt::None
    }
    fn write(&mut self, u16, u8, u64) -> u64;
    fn read(&self, u16) -> u8;
    fn screen(&self) -> &S;
    fn input(&self) -> &I;
    fn dump<T: Write>(&self, writer: &mut T);
    fn hash(&self) -> u64 {
        0
    }
}
