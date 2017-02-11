#[derive(Clone, Default)]
pub struct ApuBase;

pub trait Apu: Clone + Default {
    fn write(&mut self, _: u16, _: u8) {}
    fn read_control(&self) -> u8 {
        0
    }
}

impl Apu for ApuBase {}
