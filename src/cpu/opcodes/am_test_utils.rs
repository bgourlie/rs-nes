use audio::NoAudio;
use cpu::{TestCpu, TestMemory};
use cpu::opcodes::AddressingMode;
use input::NoInput;
use screen::NoScreen;
use std::cell::Cell;
use std::rc::Rc;

impl AddressingMode<NoScreen, NoInput, NoAudio, TestMemory> for u8 {
    type Output = u8;

    fn read(&self) -> Self::Output {
        *self
    }

    fn write(&self, cpu: &mut TestCpu, value: u8) {
        cpu.registers.acc = value;
    }
}

impl AddressingMode<NoScreen, NoInput, NoAudio, TestMemory> for i8 {
    type Output = i8;

    fn read(&self) -> Self::Output {
        *self
    }

    fn write(&self, _: &mut TestCpu, _: u8) {
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

impl AddressingMode<NoScreen, NoInput, NoAudio, TestMemory> for WriterAddressingMode {
    type Output = u8;
    fn read(&self) -> Self::Output {
        self.read_value
    }

    fn write(&self, _: &mut TestCpu, value: u8) {
        self.written.set(value);
    }
}

impl AddressingMode<NoScreen, NoInput, NoAudio, TestMemory> for u16 {
    type Output = Self;
    fn read(&self) -> Self::Output {
        *self
    }

    fn write(&self, _: &mut TestCpu, _: u8) {
        unimplemented!()
    }
}
