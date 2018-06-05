#![feature(proc_macro)]
#![allow(unknown_lints)]
#![allow(cast_lossless)]

#[cfg(test)]
extern crate asm6502;

#[cfg(test)]
extern crate rand;

extern crate rs_nes_macros;

mod apu;
mod byte_utils;
pub mod cpu;
pub mod input;
pub mod memory;
pub mod ppu;
pub mod rom;
pub mod screen;
