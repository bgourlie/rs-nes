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
    read_value: u8,
    written: Rc<Cell<u8>>,
}

impl WriterAddressingMode {
    pub fn with_read_value(value: u8) -> Self {
        WriterAddressingMode {
            written: Rc::new(Cell::new(0)),
            read_value: value,
        }
    }

    pub fn new() -> Self {
        WriterAddressingMode {
            written: Rc::new(Cell::new(0)),
            read_value: 0,
        }
    }

    pub fn write_ref(&self) -> Rc<Cell<u8>> {
        self.written.clone()
    }
}

impl AddressingMode<SimpleMemory> for WriterAddressingMode {
    type Output = u8;
    fn read(&self) -> Self::Output {
        self.read_value
    }

    fn write(&self, _: &mut Cpu<SimpleMemory>, value: u8) {
        self.written.set(value)
    }
}

impl AddressingMode<SimpleMemory> for u16 {
    type Output = Self;
    fn read(&self) -> Self::Output {
        *self
    }
}
