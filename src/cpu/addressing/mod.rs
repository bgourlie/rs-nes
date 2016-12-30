/// # Addressing Abstractions
///
/// Many NES instructions can be thought of as functions that operate on a value and store the
/// result somewhere.  Where the value is read from and where the result is written to depends on
/// the addressing mode of that particular opcode.
///
/// These abstractions allow us to implement instructions without worrying about addressing details.

use super::Cpu;
use super::debugger::Debugger;
use memory::*;


pub trait AddressingMode<M: Memory, D: Debugger<M>> {
    fn tick(&self, cpu: &mut Cpu<M, D>) -> Self;
    fn operand(&self) -> u8;
    fn write(&self, _: &mut Cpu<M, D>, _: u8) {
        unimplemented!();
    }
}

pub struct Accumulator {
    value: u8,
}

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

#[derive(Debug)]
pub enum ZeroPage {
    PreRead,
    OperandFetched(u8),
}

impl<M: Memory, D: Debugger<M>> AddressingMode<M, D> for ZeroPage {
    fn tick(&self, cpu: &mut Cpu<M, D>) -> Self {
        match *self {
            ZeroPage::PreRead => {
                let addr = cpu.read_op();
                ZeroPage::OperandFetched(addr)
            }
            _ => panic!("tick called during unexpected state: {:?}", *self),
        }
    }

    fn operand(&self) -> u8 {
        match *self {
            ZeroPage::OperandFetched(operand) => operand,
            _ => panic!("operand called during unexpected state: {:?}", self),
        }
    }
}
