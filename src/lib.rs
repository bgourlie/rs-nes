#![feature(proc_macro)]

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate log;
extern crate env_logger;
extern crate websocket;
extern crate iron;
extern crate router;
extern crate serde;
extern crate serde_json;
extern crate byteorder;
extern crate rand;

mod constants;
pub mod memory;
pub mod cpu;
