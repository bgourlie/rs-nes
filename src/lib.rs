#![feature(proc_macro)]
#![feature(test)]
#![allow(unknown_lints)]
#![allow(cast_lossless)]

#[cfg(test)]
extern crate asm6502;

#[cfg(test)]
extern crate rand;

#[cfg(test)]
extern crate test;

extern crate cpu6502;

extern crate rs_nes_macros;

mod apu;
mod cart;
mod input;
mod interconnect;
mod ppu;
mod rom;

use apu::Apu;
use cart::{Cart, Nrom128Cart};
use cpu6502::cpu::Cpu;
use input::Input;
pub use input::{Button, IInput};
use interconnect::NesInterconnect;
pub use ppu::IPpu;
use ppu::{Ppu, SpriteRenderer, Vram};
use rom::NesRom;
use std::io::Read;
use std::rc::Rc;

type NesCpu = Cpu<NesInterconnect<Ppu<Vram<Nrom128Cart>, SpriteRenderer>, Apu, Input, Nrom128Cart>>;

pub fn load_cart<R: Read>(input: R) -> Result<NesCpu, &'static str> {
    let rom = NesRom::load(input)?;
    let cart = Rc::new(Box::new(Nrom128Cart::new(rom)?));
    let vram = Vram::new(cart.clone());
    let ppu = Ppu::new(vram);
    let input = Input::default();
    let apu = Apu::default();
    let mem = NesInterconnect::new(cart, ppu, input, apu);
    let mut cpu = Cpu::new(mem, 0x00);
    cpu.reset();
    Ok(cpu)
}
