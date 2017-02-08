pub mod nes_memory;
mod simple_memory;

pub use self::simple_memory::SimpleMemory;
use cpu::TickAction;
use errors::*;
use std::io::Write;

pub const ADDRESSABLE_MEMORY: usize = 65536;

pub trait Memory: Clone {
    fn tick(&mut self) -> Result<TickAction> {
        Ok(TickAction::None)
    }
    fn store(&mut self, u16, u8) -> Result<()>;
    fn load(&self, u16) -> Result<u8>;
    fn dump<T: Write>(&self, writer: &mut T);
    fn hash(&self) -> u64 {
        0
    }
}
