/// # Addressing Abstractions
///
/// Many NES instructions can be thought of as functions that operate on a value and store the
/// result somewhere.  Where the value is read from and where the result is written to depends on
/// the addressing mode of that particular opcode.
///
/// These abstractions allow us to implement instructions without worrying about addressing details.

use cpu::Cpu6502;
use memory::*;

pub trait AddressReader<M: Memory> {
    fn read(&self, cpu: &Cpu6502<M>) -> u8;
}

pub trait AddressWriter<M: Memory>: AddressReader<M> {
    fn write(&self, cpu: &mut Cpu6502<M>, val: u8);
}

pub struct Accumulator;

impl<M: Memory> AddressReader<M> for Accumulator {
    fn read(&self, cpu: &Cpu6502<M>) -> u8 {
        cpu.registers.acc
    }
}

impl<M: Memory> AddressWriter<M> for Accumulator {
    fn write(&self, cpu: &mut Cpu6502<M>, val: u8) {
        cpu.registers.acc = val
    }
}

pub type Immediate = u8;

impl<M: Memory> AddressReader<M> for Immediate {
    fn read(&self, _: &Cpu6502<M>) -> u8 {
        *self
    }
}

pub type MemoryAddress = u16;

impl<M: Memory> AddressReader<M> for MemoryAddress {
    fn read(&self, cpu: &Cpu6502<M>) -> u8 {
        cpu.memory.load(*self)
    }
}

impl<M: Memory> AddressWriter<M> for MemoryAddress {
    fn write(&self, cpu: &mut Cpu6502<M>, val: u8) {
        cpu.memory.store(*self, val)
    }
}
