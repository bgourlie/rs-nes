pub mod nes_memory;
mod simple_memory;

pub use self::simple_memory::SimpleMemory;
use cpu::TickAction;
use errors::*;
use screen::Screen;
use std::io::Write;
use std::rc::Rc;

pub const ADDRESSABLE_MEMORY: usize = 65536;

pub trait Memory {
    type S: Screen;

    fn tick(&mut self) -> Result<TickAction> {
        Ok(TickAction::None)
    }
    fn write(&mut self, u16, u8) -> Result<()>;
    fn read(&self, u16) -> Result<u8>;
    fn dump<T: Write>(&self, writer: &mut T);
    fn screen_buffer(&self) -> Rc<Self::S>;
    fn hash(&self) -> u64 {
        0
    }
}
