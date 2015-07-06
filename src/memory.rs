pub struct Memory {
  addr:[u8; 0xffff]
}

impl Memory {
  pub fn new() -> Memory {
    Memory {
      addr: [0; 0xffff]
    }
  }

  fn store(&mut self, addr: usize, data: u8) -> Result<(), &'static str> {
    if addr > 0xffff {
      Err("memory address out of bounds")
    } else {
      self.addr[addr] = data;
      Ok(()) // TODO: There must be a cleaner way of doing this
    }
  }
}

