#![feature(proc_macro)]
#![feature(test)]

#[cfg(test)]
extern crate asm6502;

#[cfg(test)]
extern crate rand;

#[cfg(test)]
extern crate test;

extern crate byteorder;
extern crate cpu6502;
extern crate rs_nes_macros;

mod apu;
mod cart;
mod input;
mod interconnect;
mod ppu;
mod rom;

use apu::Apu;
use cart::Cart;
pub use cart::{Nrom128, Nrom256, Uxrom};
use cpu6502::cpu::Cpu;
use input::Input;
pub use input::{Button, IInput};
use interconnect::NesInterconnect;
pub use ppu::IPpu;
use ppu::{Ppu, SpriteRenderer, Vram};
pub use rom::NesRom;

#[cfg(test)]
mod mocks {
    pub use apu::mocks::ApuMock;
    pub use cart::mocks::CartMock;
    pub use input::mocks::InputMock;
    pub use ppu::mocks::{MockSpriteRenderer, MockVram, PpuMock};
}

pub fn load_cart<C: Cart>(
    cart: Box<C>,
) -> Result<Cpu<NesInterconnect<Ppu<Vram, SpriteRenderer>, Apu, Input, C>>, &'static str> {
    let vram = Vram::new();
    let ppu = Ppu::new(vram);
    let input = Input::default();
    let apu = Apu::default();
    let mem = NesInterconnect::new(cart, ppu, input, apu);
    let mut cpu = Cpu::new(mem, 0x00);
    cpu.reset();
    Ok(cpu)
}
