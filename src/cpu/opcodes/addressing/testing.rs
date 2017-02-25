use cpu::{TestCpu, TestMemory};
use cpu::opcodes::addressing::AddressingMode;
use errors::*;
use std::cell::Cell;
use std::rc::Rc;

impl AddressingMode<TestMemory> for u8 {
    type Output = u8;

    fn read(&self) -> Self::Output {
        *self
    }

    fn write(&self, cpu: &mut TestCpu, value: u8) -> Result<()> {
        cpu.registers.acc = value;
        Ok(())
    }
}

impl AddressingMode<TestMemory> for i8 {
    type Output = i8;

    fn read(&self) -> Self::Output {
        *self
    }

    fn write(&self, _: &mut TestCpu, _: u8) -> Result<()> {
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

impl AddressingMode<TestMemory> for WriterAddressingMode {
    type Output = u8;
    fn read(&self) -> Self::Output {
        self.read_value
    }

    fn write(&self, _: &mut TestCpu, value: u8) -> Result<()> {
        self.written.set(value);
        Ok(())
    }
}

impl AddressingMode<TestMemory> for u16 {
    type Output = Self;
    fn read(&self) -> Self::Output {
        *self
    }

    fn write(&self, _: &mut TestCpu, _: u8) -> Result<()> {
        unimplemented!()
    }
}
