use std::fs::File;
use std::io::Write;

use ppu::*;

#[cfg(test)]
mod spec_tests;

const ADDRESSABLE_MEMORY: usize = 65536;

pub trait Memory {
    fn store(&mut self, u16, u8);
    fn store16(&mut self, u16, u16);
    fn load(&self, u16) -> u8;
    fn load16(&self, u16) -> u16;

    // probably a premature optimization, but we implement inc and dec here so
    // that we can alter the values in place.
    fn inc(&mut self, u16) -> u8;
    fn dec(&mut self, u16) -> u8;
    fn dump(&self, file: &'static str);
}

pub struct NesMemory {
    ram: [u8; 2048],
    ppu: Ppu,
}

impl NesMemory {
    pub fn new() -> Self {
        NesMemory {
            ppu: Ppu::new(),
            ram: [0; 2048],
        }
    }
}

impl Memory for NesMemory {
    fn store(&mut self, addr: u16, data: u8) {
        let addr = addr as usize;
        match addr {
            0x0...0x1fff => {
                // 2048kb ram, mirrored 4 times
                self.ram[addr & 0x7ff] = data;
            }
            0x2000...0x3fff => {
                // PPU registers, mirrored every 8 bytes
                match addr & 0x7 {
                    0x0 => {
                        unimplemented!();
                    } // PPUCTRL
                    0x1 => {
                        unimplemented!();
                    } // PPUMASK
                    0x2 => {
                        unimplemented!();
                    } // PPUSTATUS
                    0x3 => {
                        unimplemented!();
                    } // OAMADDR
                    0x4 => {
                        unimplemented!();
                    } // OAMDATA
                    0x5 => {
                        unimplemented!();
                    } // PPUSCROLL
                    0x6 => {
                        unimplemented!();
                    } // PPUADDR
                    0x7 => {
                        unimplemented!();
                    } // PPUDATA
                    _ => {
                        panic!("This should never happen");
                    }
                }
            }
            0x4000 => {
                unimplemented!();
            } // APU Rectangle 1 W
            0x4001 => {
                unimplemented!();
            } // APU Rectangle 1 W
            0x4002 => {
                unimplemented!();
            } // APU Rectangle 1 W
            0x4003 => {
                unimplemented!();
            } // APU Rectangle 1 W
            0x4004 => {
                unimplemented!();
            } // APU Rectangle 2 W
            0x4005 => {
                unimplemented!();
            } // APU Rectangle 2 W
            0x4006 => {
                unimplemented!();
            } // APU Rectangle 2 W
            0x4007 => {
                unimplemented!();
            } // APU Rectangle 2 W
            0x4008 => {
                unimplemented!();
            } // APU Triangle W
            0x4009 => {
                unimplemented!();
            } // APU Triangle W
            0x400a => {
                unimplemented!();
            } // APU Triangle W
            0x400b => {
                unimplemented!();
            } // APU Triangle W
            0x400c => {
                unimplemented!();
            } // APU Noise W
            0x400d => {
                unimplemented!();
            } // APU Noise W
            0x400e => {
                unimplemented!();
            } // APU Noise W
            0x400f => {
                unimplemented!();
            } // APU Noise W
            0x4010 => {
                unimplemented!();
            } // APU DMC W
            0x4011 => {
                unimplemented!();
            } // APU DMC W
            0x4012 => {
                unimplemented!();
            } // APU DMC W
            0x4013 => {
                unimplemented!();
            } // APU DMC W
            0x4014 => {
                unimplemented!();
            } // Sprite DMA W
            0x4015 => {
                unimplemented!();
            } // Sound Status RW
            0x4016 => {
                unimplemented!();
            } // Input 1 RW
            0x4017 => {
                unimplemented!();
            } // Input 2, Frame Counter RW
            0x4018 => {
                unimplemented!();
            } // ??
            0x4019 => {
                unimplemented!();
            } // ??
            0x401a => {
                unimplemented!();
            } // ??
            0x401b => {
                unimplemented!();
            } // ??
            0x401c => {
                unimplemented!();
            } // ??
            0x401d => {
                unimplemented!();
            } // ??
            0x401e => {
                unimplemented!();
            } // ??
            0x401f => {
                unimplemented!();
            } // ??
            0x4020...0xffff => {
                unimplemented!();
            } // mapped to cartridge
            _ => {
                panic!("Should never get here.");
            }
        }
    }

    fn store16(&mut self, addr: u16, data: u16) {
        unimplemented!();
    }

    fn load(&self, addr: u16) -> u8 {
        unimplemented!();
    }

    fn load16(&self, addr: u16) -> u16 {
        unimplemented!();
    }

    fn inc(&mut self, addr: u16) -> u8 {
        unimplemented!();
    }

    fn dec(&mut self, addr: u16) -> u8 {
        unimplemented!();
    }

    fn dump(&self, file_loc: &'static str) {
        unimplemented!();
    }
}

pub struct SimpleMemory {
    addr: [u8; ADDRESSABLE_MEMORY],
}

impl SimpleMemory {
    pub fn new() -> Self {
        SimpleMemory { addr: [0; ADDRESSABLE_MEMORY] }
    }

    pub fn store_many(&mut self, addr: u16, data: &[u8]) {
        for i in 0..data.len() {
            self.store(addr + i as u16, data[i]);
        }
    }
}

impl Memory for SimpleMemory {
    fn store(&mut self, addr: u16, data: u8) {
        let addr = addr as usize;
        self.addr[addr] = data;
    }

    fn store16(&mut self, addr: u16, data: u16) {
        let lowb = (data & 0xff) as u8;
        let highb = ((data >> 8) & 0xff) as u8;
        self.store(addr, lowb);
        self.store(addr + 1, highb);
    }

    fn load(&self, addr: u16) -> u8 {
        let addr = addr as usize;
        self.addr[addr]
    }

    fn load16(&self, addr: u16) -> u16 {
        let addr = addr as usize;
        self.addr[addr] as u16 | (self.addr[addr + 1] as u16) << 8
    }

    fn inc(&mut self, addr: u16) -> u8 {
        let addr = addr as usize;
        self.addr[addr] = (self.addr[addr] as u16 + 1) as u8;
        self.addr[addr]
    }

    fn dec(&mut self, addr: u16) -> u8 {
        let addr = addr as usize;
        self.addr[addr] = (self.addr[addr] as i16 - 1) as u8;
        self.addr[addr]
    }

    fn dump(&self, file_loc: &'static str) {
        let mut f = File::create(file_loc).unwrap();
        f.write_all(&self.addr).unwrap();
    }
}
