extern crate iron;
extern crate mount;
extern crate staticfile;

use std::fs::File;
use std::io::Read;
use std::path::Path;

use iron::{Iron, Request, Response, IronResult};
use iron::status;
use staticfile::Static;
use mount::Mount;

mod constants;
mod memory;
mod cpu;
mod rom_loader;

use rom_loader::NesRom;
use cpu::Cpu6502;

fn main() {
  // let mut mount = Mount::new();

  // mount.mount("/api/rom_data", rom_data).mount("/", Static::new(
  //     Path::new("www/")));

  // println!("Server running on http://localhost:3000");
  //Iron::new(mount).http("127.0.0.1:3000").unwrap();

  let mut f = File::open("/Users/brian/Desktop/roms/6502_functional_test.bin")
      .unwrap();

  let mut vec = Vec::<u8>::new();
  let bytes_read = f.read_to_end(&mut vec).unwrap();
  if bytes_read < 65536 {
    panic!("expected 16kb of data");
  }

  let mut cpu = Cpu6502::new();
  cpu.load(0, &vec, 0x400);
  cpu.step(); 
}

// fn rom_data(req: &mut Request) -> IronResult<Response> {
//   let rom = NesRom::read("/Users/brian/Desktop/roms/Super Mario Bros. 3 (U) (PRG1) [!].nes").unwrap();
//   Ok(Response::with((status::Ok, "Hello!")))
// }
