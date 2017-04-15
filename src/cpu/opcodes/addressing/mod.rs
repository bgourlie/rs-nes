#[cfg(test)]
pub mod testing;

mod absolute;
mod absolute_address;
mod absolute_x;
mod absolute_y;
mod accumulator;
mod indirect;
mod immediate;
mod implied;
mod indexed_indirect;
mod indirect_indexed;
mod relative;
mod zero_page;
mod zero_page_x;
mod zero_page_y;

use cpu::Cpu;
pub use cpu::opcodes::addressing::absolute::Absolute;
pub use cpu::opcodes::addressing::absolute_address::AbsoluteAddress;
pub use cpu::opcodes::addressing::absolute_x::AbsoluteX;
pub use cpu::opcodes::addressing::absolute_y::AbsoluteY;
pub use cpu::opcodes::addressing::accumulator::Accumulator;
pub use cpu::opcodes::addressing::immediate::Immediate;
pub use cpu::opcodes::addressing::implied::Implied;
pub use cpu::opcodes::addressing::indexed_indirect::IndexedIndirect;
pub use cpu::opcodes::addressing::indirect::Indirect;
pub use cpu::opcodes::addressing::indirect_indexed::IndirectIndexed;
pub use cpu::opcodes::addressing::relative::Relative;
pub use cpu::opcodes::addressing::zero_page::ZeroPage;
pub use cpu::opcodes::addressing::zero_page_x::ZeroPageX;
pub use cpu::opcodes::addressing::zero_page_y::ZeroPageY;
use memory::*;
use screen::Screen;

pub trait AddressingMode<S: Screen, M: Memory<S>> {
    type Output;
    fn read(&self) -> Self::Output;
    fn write(&self, _: &mut Cpu<S, M>, _: u8) {
        unimplemented!();
    }
}
