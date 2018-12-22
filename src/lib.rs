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

pub use crate::{
    apu::Apu,
    cart::{Cart, Nrom128, Nrom256, Uxrom},
    input::{Button, IInput, Input},
    interconnect::NesInterconnect,
    ppu::{IPpu, Ppu, SpriteRenderer, Vram},
    rom::NesRom,
};
use cpu6502::cpu::Cpu;

#[cfg(test)]
mod mocks {
    pub use crate::{
        apu::mocks::ApuMock,
        cart::mocks::CartMock,
        input::mocks::InputMock,
        ppu::mocks::{MockSpriteRenderer, MockVram, PpuMock},
    };
}

pub fn load_cart<C: Cart>(
    cart: C,
) -> Result<Box<Cpu<NesInterconnect<Ppu<Vram, SpriteRenderer>, Apu, Input, C>>>, &'static str> {
    let vram = Vram::new();
    let ppu = Ppu::new(vram);
    let input = Input::default();
    let apu = Apu::default();
    let mem = NesInterconnect::new(cart, ppu, input, apu);
    let mut cpu = Box::new(Cpu::new(mem, 0x00));
    cpu.reset();
    Ok(cpu)
}
