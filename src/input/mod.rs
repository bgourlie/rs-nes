#[derive(Clone, Default)]
pub struct InputBase {
    probe: u8,
    joy1: u8,
    joy2: u8,
}

pub trait Input: Clone + Default {
    fn write_probe(&mut self, _: u8);
    fn read_joy_1(&self) -> u8;
    fn read_joy_2(&self) -> u8;
}

impl Input for InputBase {
    fn write_probe(&mut self, val: u8) {
        self.probe = val;
    }

    fn read_joy_1(&self) -> u8 {
        self.joy1
    }

    fn read_joy_2(&self) -> u8 {
        self.joy2
    }
}
