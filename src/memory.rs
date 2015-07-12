// NES MEMORY MAP
// http://emudocs.org/NES/nestech.txt
//
// +---------+-------+-------+-----------------------+
// | Address | Size  | Flags | Description           |
// +---------+-------+-------+-----------------------+
// | $0000   | $800  |       | RAM                   |
// | $0800   | $800  | M     | RAM                   |
// | $1000   | $800  | M     | RAM                   |
// | $1800   | $800  | M     | RAM                   |
// | $2000   | 8     |       | Registers             |
// | $2008   | $1FF8 |  R    | Registers             |
// | $4000   | $20   |       | Registers             |
// | $4020   | $1FDF |       | Expansion ROM         |
// | $6000   | $2000 |       | SRAM                  |
// | $8000   | $4000 |       | PRG-ROM               |
// | $C000   | $4000 |       | PRG-ROM               |
// +---------+-------+-------+-----------------------+
//        Flag Legend: M = Mirror of $0000
//                     R = Mirror of $2000-2008 every 8 bytes
//                         (e.g. $2008=$2000, $2018=$2000, etc.)

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

