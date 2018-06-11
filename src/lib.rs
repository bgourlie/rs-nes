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
pub mod input;
pub mod interconnect;
pub mod ppu;
pub mod rom;

pub use cpu6502::cpu::Cpu;
