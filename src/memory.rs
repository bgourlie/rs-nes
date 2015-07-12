pub struct Memory {
  addr:[u8; 0xffff]
}

impl Memory {
  pub fn new() -> Memory {
    Memory {
      addr: [0; 0xffff]
    }
  }

  pub fn store(&mut self, addr: usize, data: u8) -> Result<(), &'static str> {
    if addr > 0xffff {
      Err("memory address out of bounds")
    } else {
      self.addr[addr] = data;
      Ok(()) 
    }
  }

  pub fn load(&mut self, addr: usize) -> Result<u8, &'static str> {
    if addr > 0xffff {
      Err("memory address out of bounds")
    } else {
      Ok(self.addr[addr])
    }
  }
}

