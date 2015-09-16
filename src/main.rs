extern crate iron;
extern crate mount;
extern crate staticfile;

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

fn main() {
  let mut mount = Mount::new();

  mount.mount("/api/rom_data", rom_data).mount("/", Static::new(
      Path::new("www/")));

  println!("Server running on http://localhost:3000");
  Iron::new(mount).http("127.0.0.1:3000").unwrap();
}

fn rom_data(req: &mut Request) -> IronResult<Response> {
  let rom = NesRom::read("/Users/brian/Desktop/roms/Super Mario Bros. 3 (U) (PRG1) [!].nes").unwrap();
  Ok(Response::with((status::Ok, "Hello!")))
}
