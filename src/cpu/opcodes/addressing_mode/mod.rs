#[cfg(test)]
mod test_addressing_mode;

mod absolute;
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

pub use self::absolute::Absolute;
pub use self::absolute_x::AbsoluteX;
pub use self::absolute_y::AbsoluteY;
pub use self::accumulator::Accumulator;
pub use self::immediate::Immediate;
pub use self::implied::Implied;
pub use self::indirect::Indirect;
pub use self::indexed_indirect::IndexedIndirect;
pub use self::indirect_indexed::IndirectIndexed;
pub use self::relative::Relative;
pub use self::zero_page::ZeroPage;
pub use self::zero_page_x::ZeroPageX;
pub use self::zero_page_y::ZeroPageY;

use super::Cpu;
use memory::*;

pub trait AddressingMode<M: Memory> {
    fn execute<F: Fn(&Cpu<M>)>(cpu: &mut Cpu<M>, tick_handler: F) -> Self;
    fn operand<F: Fn(&Cpu<M>)>(&mut self, cpu: &mut Cpu<M>, tick_handler: F) -> u8;
    fn write<F: Fn(&Cpu<M>)>(&self, _: &mut Cpu<M>, _: u8, _: F) {
        unimplemented!()
    }
}

// pub struct Accumulator {
//    value: u8,
// }

// impl<M: Memory, D: Debugger<M>> AddressingMode<M, D> for Accumulator {
//    fn operand(self) -> u8 {
//        self.value
//    }
//
//    fn write(&self, cpu: &mut Cpu<M, D>, val: u8) {
//        cpu.registers.acc = val
//    }
//    fn tick(&mut self, cpu: &mut Cpu<M, D>) {
//        unimplemented!()
//    }
// }

// pub struct Immediate {
//    value: u8
// }
//
// impl<M: Memory, D: Debugger<M>> AddressingMode<M, D> for Immediate {
//    fn operand(self) -> u8 {
//        self.value
//    }
//    fn tick(&mut self, cpu: &mut Cpu<M, D>) {
//        unimplemented!()
//    }
// }
