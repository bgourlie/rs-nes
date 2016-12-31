use cpu::Cpu;
use memory::SimpleMemory;
use super::AddressingMode;

// A convenience addressing mode for testing, implemented on u8 so it can be passed directly to the
// OpCode's execute method.  Writes are written to the accumulator.

impl AddressingMode<SimpleMemory> for u8 {
    fn operand(&self) -> u8 {
        *self
    }

    fn write(&self, cpu: &mut Cpu<SimpleMemory>, value: u8) {
        cpu.registers.acc = value;
    }
}
