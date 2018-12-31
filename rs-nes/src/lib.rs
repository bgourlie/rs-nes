#![feature(box_syntax)]

#[cfg(test)]
extern crate asm6502;

#[cfg(test)]
extern crate rand;

extern crate byteorder;
extern crate cpu6502;
extern crate rs_nes_macros;

mod apu;
mod cart;
mod input;
mod interconnect;
mod ppu;
mod rom;

pub use crate::{
    apu::Apu,
    cart::{Cart, Nrom128, Nrom256, Uxrom},
    input::{Button, IInput, Input},
    interconnect::NesInterconnect,
    ppu::{IPpu, Ppu, SpriteRenderer, Vram},
    rom::NesRom,
};
use cpu6502::cpu::Cpu;
pub use cpu6502::cpu::Interrupt;

pub type Nes<C> = Cpu<NesInterconnect<Ppu<Vram, SpriteRenderer>, Apu, Input, C>>;

#[cfg(test)]
mod mocks {
    pub use crate::{
        apu::mocks::ApuMock,
        cart::mocks::CartMock,
        input::mocks::InputMock,
        ppu::mocks::{MockSpriteRenderer, MockVram, PpuMock},
    };
}

pub fn load_cart<C: Cart>(cart: C) -> Result<Box<Nes<C>>, &'static str> {
    let interconnect = NesInterconnect::new(cart);
    let mut cpu = box Cpu::new(interconnect, 0x00);
    cpu.reset();
    Ok(cpu)
}
