#[cfg(test)]
pub mod mocks;

#[derive(Default)]
pub struct Apu;

pub trait IApu: Default {
    fn write(&mut self, _: u16, _: u8) {}
    fn read_control(&self) -> u8 {
        0
    }
}

impl IApu for Apu {}
