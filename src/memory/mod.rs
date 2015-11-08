use std::fs::File;
use std::io::Write;

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

pub struct SimpleMemory {
    addr: [u8; ADDRESSABLE_MEMORY],
}

impl SimpleMemory {
    pub fn new() -> SimpleMemory {
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
