// This abstraction is influenced by a similar abstraction in sprocketnes.

use cpu::Cpu6502;
use memory::*;

trait AddressingModeReader<M: Memory> {
    fn read(&self, cpu: &Cpu6502<M>) -> u8;
}

trait AddressingModeWriter<M: Memory>: AddressingModeReader<M> {
    fn write(&self, cpu: &mut Cpu6502<M>, val: u8);
}

struct AccumulatorAddressingMode;

impl<M: Memory> AddressingModeReader<M> for AccumulatorAddressingMode {
    fn read(&self, cpu: &Cpu6502<M>) -> u8 {
        cpu.registers.acc
    }
}

impl<M: Memory> AddressingModeWriter<M> for AccumulatorAddressingMode {
    fn write(&self, cpu: &mut Cpu6502<M>, val: u8) {
        cpu.registers.acc = val
    }
}

type ImmediateAddressingMode = u8;

impl<M: Memory> AddressingModeReader<M> for ImmediateAddressingMode {
    fn read(&self, cpu: &Cpu6502<M>) -> u8 {
        *self
    }
}

type MemoryAddressingMode = u16;

impl<M: Memory> AddressingModeReader<M> for MemoryAddressingMode {
    fn read(&self, cpu: &Cpu6502<M>) -> u8 {
        cpu.memory.load(*self)
    }
}

impl<M: Memory> AddressingModeWriter<M> for MemoryAddressingMode {
    fn write(&self, cpu: &mut Cpu6502<M>, val: u8) {
        cpu.memory.store(*self, val)
    }
}
