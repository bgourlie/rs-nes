pub mod zero_page;
pub mod immediate;

use super::Cpu;
use memory::*;

pub trait ExecutionContext<M: Memory> {
    fn operand<F: Fn(&Cpu<M>)>(&self, cpu: &mut Cpu<M>, tick_handler: F) -> u8;
    fn write(&self, _: &mut Cpu<M>, _: u8) {
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
