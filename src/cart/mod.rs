#[cfg(test)]
pub mod mocks;

mod nrom128;
mod nrom256;
mod uxrom;

pub use self::nrom128::Nrom128;
pub use self::nrom256::Nrom256;
pub use self::uxrom::Uxrom;

pub trait Cart: Sized {
    fn read_prg(&self, addr: u16) -> u8;
    fn write_prg(&mut self, addr: u16, value: u8);
    fn read_chr(&self, addr: u16) -> u8;
    fn write_chr(&mut self, addr: u16, value: u8);
}
