use apu::IApu;

#[derive(Default)]
pub struct ApuMock {
    write_addr: u16,
    write_value: u8,
    control: u8,
}

impl ApuMock {
    pub fn write_addr(&self) -> u16 {
        self.write_addr
    }

    pub fn write_value(&self) -> u8 {
        self.write_value
    }

    pub fn set_control(&mut self, val: u8) {
        self.control = val;
    }
}

impl IApu for ApuMock {
    fn write(&mut self, addr: u16, value: u8) {
        self.write_addr = addr;
        self.write_value = value;
    }

    fn read_control(&self) -> u8 {
        self.control
    }
}
