#[cfg(feature = "debugger")]
#[macro_use]
extern crate log;

#[macro_use]
extern crate lazy_static;

extern crate sdl2;

#[cfg(feature = "debugger")]
extern crate env_logger;

#[cfg(feature = "debugger")]
extern crate serde;

#[cfg(feature = "debugger")]
extern crate serde_json;

#[cfg(feature = "debugger")]
#[macro_use]
extern crate serde_derive;

#[cfg(feature = "debugger")]
extern crate websocket;

#[cfg(feature = "debugger")]
extern crate iron;

#[cfg(feature = "debugger")]
extern crate router;

#[cfg(feature = "debugger")]
extern crate byteorder;

#[cfg(feature = "debugger")]
extern crate chan;

#[cfg(feature = "debugger")]
extern crate seahash;

#[cfg(feature = "debugger")]
extern crate base64;

#[cfg(feature = "debugger")]
extern crate png;

#[cfg(test)]
extern crate asm6502;

#[cfg(test)]
extern crate rand;

pub mod rom;
pub mod memory;
pub mod cpu;
pub mod ppu;
pub mod screen;
pub mod input;
pub mod audio;
pub mod audio_out;
pub mod apu;
mod byte_utils;
