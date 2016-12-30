pub mod absolute;
pub mod absolute_x;
pub mod absolute_y;
pub mod accumulator;
pub mod immediate;
pub mod implied;
pub mod indexed_indirect;
pub mod relative;
pub mod zero_page;
pub mod zero_page_x;
pub mod zero_page_y;

use super::Cpu;
use memory::*;

pub trait ExecutionContext<M: Memory> {
    fn operand<F: Fn(&Cpu<M>)>(&mut self, cpu: &mut Cpu<M>, tick_handler: F) -> u8;
    fn write<F: Fn(&Cpu<M>)>(&self, _: &mut Cpu<M>, _: u8, _: F) {
        unimplemented!();
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
