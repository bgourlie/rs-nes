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

  pub fn store(&mut self, addr: u16, data: u8) {
    let addr = addr as usize;
    match addr {
      0x0 ... 0x7FF => {
        // mirrored ram
        self.addr[addr] = data;
        self.addr[addr + 0x800] = data;
        self.addr[addr + 0x1000] = data;
        self.addr[addr + 0x1800] = data;
      },
      0x800 ... 0x1FFF => panic!("write to mirrored memory"),
      0x2000 ... 0x401F => {
        // registers
        self.addr[addr] = data;
      },
      0x4020 ... 0x5FFF | 0x8000 ... 0xEFFF => panic!("write to rom"),
      0x6000 ... 0x7FFF => {
        // sram
        self.addr[addr] = data;
      },
      _ => panic!("memory access out of bounds")
    }
  }

  pub fn store16(&mut self, addr: u16, data: u16) {
    let lowb = (data & 0xff) as u8;
    let highb = ((data >> 8) & 0xff) as u8;
    self.store(addr, lowb);
    self.store(addr + 1, highb);
  }

  pub fn load(&mut self, addr: u16) -> u8 {
    let addr = addr as usize;
    self.addr[addr]
  }

  pub fn load16(&mut self, addr: u16) -> u16 {
    let addr = addr as usize;
    self.addr[addr] as u16 | (self.addr[addr + 1] as u16) << 8
  }

  // probably a premature optimization, but we implement inc and dec here so
  // that we can alter the values in place.
  pub fn inc(&mut self, addr: u16) -> u8 {
    let addr = addr as usize;
    self.addr[addr] = (self.addr[addr] as u16 + 1) as u8;
    self.addr[addr]
  }

  pub fn dec(&mut self, addr: u16) -> u8 {
    let addr = addr as usize;
    self.addr[addr] = (self.addr[addr] as i16 - 1) as u8;
    self.addr[addr]
  }
}

