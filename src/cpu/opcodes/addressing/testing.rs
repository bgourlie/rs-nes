use cpu::TestCpu;
use cpu::opcodes::addressing_mode::AddressingMode;
use memory::SimpleMemory;
use std::cell::Cell;
use std::rc::Rc;

impl AddressingMode<SimpleMemory> for u8 {
    type Output = u8;

    fn read(&self) -> Self::Output {
        *self
    }

    fn write<F: Fn(&TestCpu)>(&self, cpu: &mut TestCpu, value: u8, _: F) {
        cpu.registers.acc = value;
    }
}

impl AddressingMode<SimpleMemory> for i8 {
    type Output = i8;

    fn read(&self) -> Self::Output {
        *self
    }

    fn write<F: Fn(&TestCpu)>(&self, _: &mut TestCpu, _: u8, _: F) {
        unimplemented!()
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

    fn write<F: Fn(&TestCpu)>(&self, _: &mut TestCpu, value: u8, _: F) {
        self.written.set(value)
    }
}

impl AddressingMode<SimpleMemory> for u16 {
    type Output = Self;
    fn read(&self) -> Self::Output {
        *self
    }

    fn write<F: Fn(&TestCpu)>(&self, _: &mut TestCpu, _: u8, _: F) {
        unimplemented!()
    }
}
