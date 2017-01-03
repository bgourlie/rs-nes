use std::cell::Cell;
use std::rc::Rc;
use cpu::Cpu;
use memory::SimpleMemory;
use super::AddressingMode;

impl AddressingMode<SimpleMemory> for u8 {
    type Output = u8;

    fn read(&self) -> Self::Output {
        *self
    }

    fn write(&self, cpu: &mut Cpu<SimpleMemory>, value: u8) {
        cpu.registers.acc = value;
    }
}

impl AddressingMode<SimpleMemory> for i8 {
    type Output = i8;

    fn read(&self) -> Self::Output {
        *self
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
    type Output = u8;
    fn read(&self) -> Self::Output {
        self.value.get()
    }

    fn write(&self, _: &mut Cpu<SimpleMemory>, value: u8) {
        self.value.set(value)
    }
}

impl AddressingMode<SimpleMemory> for u16 {
    type Output = Self;
    fn read(&self) -> Self::Output {
        *self
    }
}
