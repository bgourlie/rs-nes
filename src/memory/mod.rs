pub mod nes_memory;
mod simple_memory;

pub use self::simple_memory::SimpleMemory;
use std::io::Write;

#[cfg(test)]
mod spec_tests;

pub const ADDRESSABLE_MEMORY: usize = 65536;

pub trait Memory: Clone {
    fn tick(&mut self) -> TickAction {
        TickAction::None
    }
    fn store(&mut self, u16, u8);
    fn load(&self, u16) -> u8;
    fn hash(&self) -> u64;
    fn dump<T: Write>(&self, writer: &mut T);
}

#[derive(PartialEq, Eq)]
pub enum TickAction {
    None,
    Nmi,
}
