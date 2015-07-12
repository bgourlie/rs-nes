// NES MEMORY MAP
// http://wiki.nesdev.com/w/index.php/Sample_RAM_map
//
// Address      Size    Description
// ---------------------------------
// $0000-$07FF	$0800	2KB internal RAM
// $0800-$0FFF	$0800	Mirrors of $0000-$07FF
// $1000-$17FF	$0800
// $1800-$1FFF	$0800
// $2000-$2007	$0008	NES PPU registers
// $2008-$3FFF	$1FF8	Mirrors of $2000-2007 (repeats every 8 bytes)
// $4000-$401F	$0020	NES APU and I/O registers
// $4020-$FFFF	$BFE0	Cartridge space: PRG ROM, PRG RAM, and mapper registers (See Note)

const ADDRESSABLE_MEMORY: usize = 0xFFFF;

pub struct Memory {
  addr:[u8; ADDRESSABLE_MEMORY]
}

impl Memory {
  pub fn new() -> Memory {
    Memory {
      addr: [0; ADDRESSABLE_MEMORY]
    }
  }

  pub fn store(&mut self, addr: usize, data: u8) -> Result<(), &'static str> {
    if addr > ADDRESSABLE_MEMORY {
      Err("memory address out of bounds")
    } else {
      self.addr[addr] = data;
      Ok(()) 
    }
  }

  pub fn load(&mut self, addr: usize) -> Result<u8, &'static str> {
    if addr > ADDRESSABLE_MEMORY {
      Err("memory address out of bounds")
    } else {
      Ok(self.addr[addr])
    }
  }
}

