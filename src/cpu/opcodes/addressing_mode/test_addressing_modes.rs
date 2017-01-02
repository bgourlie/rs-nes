use std::cell::Cell;
use std::rc::Rc;
use cpu::Cpu;
use memory::SimpleMemory;
use super::AddressingMode;

impl AddressingMode<SimpleMemory> for u8 {
    fn operand(&self) -> u8 {
        *self
    }

    fn write(&self, cpu: &mut Cpu<SimpleMemory>, value: u8) {
        cpu.registers.acc = value;
    }
}

impl AddressingMode<SimpleMemory> for i8 {
    fn operand(&self) -> u8 {
        *self as u8
    }
}

pub struct WriterAddressingMode {
    value: Rc<Cell<u8>>,
}

impl WriterAddressingMode {
    pub fn new(value: u8) -> Self {
        WriterAddressingMode { value: Rc::new(Cell::new(value)) }
    }

    pub fn value_ref(&self) -> Rc<Cell<u8>> {
        self.value.clone()
    }
}

impl AddressingMode<SimpleMemory> for WriterAddressingMode {
    fn operand(&self) -> u8 {
        self.value.get()
    }

    fn write(&self, _: &mut Cpu<SimpleMemory>, value: u8) {
        self.value.set(value)
    }
}
