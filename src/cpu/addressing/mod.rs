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


pub trait AddressReader<M: Memory, D: Debugger<M>> {
    fn read(&self, cpu: &Cpu<M, D>) -> u8;
}

pub trait AddressWriter<M: Memory, D: Debugger<M>>: AddressReader<M, D> {
    fn write(&self, cpu: &mut Cpu<M, D>, val: u8);
}

pub struct Accumulator;

impl<M: Memory, D: Debugger<M>> AddressReader<M, D> for Accumulator {
    fn read(&self, cpu: &Cpu<M, D>) -> u8 {
        cpu.registers.acc
    }
}

impl<M: Memory, D: Debugger<M>> AddressWriter<M, D> for Accumulator {
    fn write(&self, cpu: &mut Cpu<M, D>, val: u8) {
        cpu.registers.acc = val
    }
}

pub type Immediate = u8;

impl<M: Memory, D: Debugger<M>> AddressReader<M, D> for Immediate {
    fn read(&self, _: &Cpu<M, D>) -> u8 {
        *self
    }
}

pub type MemoryAddress = u16;

impl<M: Memory, D: Debugger<M>> AddressReader<M, D> for MemoryAddress {
    fn read(&self, cpu: &Cpu<M, D>) -> u8 {
        cpu.memory.load(*self)
    }
}

impl<M: Memory, D: Debugger<M>> AddressWriter<M, D> for MemoryAddress {
    fn write(&self, cpu: &mut Cpu<M, D>, val: u8) {
        cpu.memory.store(*self, val)
    }
}
