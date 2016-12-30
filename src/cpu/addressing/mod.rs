pub mod zero_page;

use super::Cpu;
use memory::*;


pub trait ExecutionContext<M: Memory> {
    fn operand<Func: Fn()>(&self, cpu: &mut Cpu<M>, func: Func) -> u8;
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
