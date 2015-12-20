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
        match addr {
            0x0...0x1fff => {
                // 2kb ram, mirrored 4 times
                let addr = addr as usize;
                self.ram[addr & 0x7ff] = data;
            }
            0x2000...0x3fff => {
                // PPU registers, mirrored every 8 bytes
                self.ppu.write_reg(addr, data);
            }
            0x4000 => {
                // APU Rectangle 1 W
                unimplemented!();
            }
            0x4001 => {
                // APU Rectangle 1 W
                unimplemented!();
            }
            0x4002 => {
                // APU Rectangle 1 W
                unimplemented!();
            }
            0x4003 => {
                // APU Rectangle 1 W
                unimplemented!();
            }
            0x4004 => {
                // APU Rectangle 2 W
                unimplemented!();
            }
            0x4005 => {
                // APU Rectangle 2 W
                unimplemented!();
            }
            0x4006 => {
                // APU Rectangle 2 W
                unimplemented!();
            }
            0x4007 => {
                // APU Rectangle 2 W
                unimplemented!();
            }
            0x4008 => {
                // APU Triangle W
                unimplemented!();
            }
            0x4009 => {
                // APU Triangle W
                unimplemented!();
            }
            0x400a => {
                // APU Triangle W
                unimplemented!();
            }
            0x400b => {
                // APU Triangle W
                unimplemented!();
            }
            0x400c => {
                // APU Noise W
                unimplemented!();
            }
            0x400d => {
                // APU Noise W
                unimplemented!();
            }
            0x400e => {
                // APU Noise W
                unimplemented!();
            }
            0x400f => {
                // APU Noise W
                unimplemented!();
            }
            0x4010 => {
                // APU DMC W
                unimplemented!();
            }
            0x4011 => {
                // APU DMC W
                unimplemented!();
            }
            0x4012 => {
                // APU DMC W
                unimplemented!();
            }
            0x4013 => {
                // APU DMC W
                unimplemented!();
            }
            0x4014 => {
                // Sprite DMA W
                unimplemented!();
            }
            0x4015 => {
                // Sound Status RW
                unimplemented!();
            }
            0x4016 => {
                // Input 1 RW
                unimplemented!();
            }
            0x4017 => {
                // Input 2, Frame Counter RW
                unimplemented!();
            }
            0x4018 => {
                // ??
                unimplemented!();
            }
            0x4019 => {
                // ??
                unimplemented!();
            }
            0x401a => {
                // ??
                unimplemented!();
            }
            0x401b => {
                // ??
                unimplemented!();
            }
            0x401c => {
                // ??
                unimplemented!();
            }
            0x401d => {
                // ??
                unimplemented!();
            }
            0x401e => {
                // ??
                unimplemented!();
            }
            0x401f => {
                // ??
                unimplemented!();
            }
            0x4020...0xffff => {
                // mapped to cartridge
                unimplemented!();
            }
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
