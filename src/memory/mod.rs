#[cfg(test)]
mod spec_tests;

const ADDRESSABLE_MEMORY: usize = 65536;

pub struct Memory {
    addr: [u8; ADDRESSABLE_MEMORY],
}

impl Memory {
    pub fn new() -> Memory {
        Memory { addr: [0; ADDRESSABLE_MEMORY] }
    }

    pub fn store_many(&mut self, addr: u16, data: &[u8]) {
        for i in 0..data.len() {
            self.store(addr + i as u16, data[i]);
        }
    }

    pub fn get_bytes(&self) -> &[u8] {
        &self.addr
    }

    pub fn store(&mut self, addr: u16, data: u8) {
        let addr = addr as usize;
        self.addr[addr] = data;
    }

    pub fn store16(&mut self, addr: u16, data: u16) {
        let lowb = (data & 0xff) as u8;
        let highb = ((data >> 8) & 0xff) as u8;
        self.store(addr, lowb);
        self.store(addr + 1, highb);
    }

    pub fn load(&self, addr: u16) -> u8 {
        let addr = addr as usize;
        self.addr[addr]
    }

    pub fn load16(&self, addr: u16) -> u16 {
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
