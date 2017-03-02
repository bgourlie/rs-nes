use super::{ADDRESSABLE_MEMORY, Memory};
use errors::*;
use screen::Screen;

#[cfg(feature = "debugger")]
use seahash;
use std::cell::RefCell;
use std::io::Write;
use std::rc::Rc;

pub struct SimpleMemory<S> {
    addr: [u8; ADDRESSABLE_MEMORY],
    screen: Rc<RefCell<S>>,
}

impl<S: Screen> SimpleMemory<S> {
    pub fn new(screen: Rc<RefCell<S>>) -> Self {
        SimpleMemory {
            addr: [0; ADDRESSABLE_MEMORY],
            screen: screen,
        }
    }

    pub fn store_many(&mut self, addr: u16, data: &[u8]) {
        for (i, byte) in data.iter().enumerate() {
            self.write(addr + i as u16, *byte).unwrap();
        }
    }
}

impl<S: Screen> Default for SimpleMemory<S> {
    fn default() -> Self {
        Self::new(Rc::new(RefCell::new(S::default())))
    }
}

impl<S: Screen> Memory for SimpleMemory<S> {
    type S = S;

    fn write(&mut self, addr: u16, data: u8) -> Result<()> {
        let addr = addr as usize;
        self.addr[addr] = data;
        Ok(())
    }

    fn read(&self, addr: u16) -> Result<u8> {
        let addr = addr as usize;
        Ok(self.addr[addr])
    }

    fn dump<T: Write>(&self, writer: &mut T) {
        writer.write_all(&self.addr).unwrap();
    }

    #[cfg(feature = "debugger")]
    fn hash(&self) -> u64 {
        seahash::hash(&self.addr)
    }

    fn screen_buffer(&self) -> Rc<RefCell<Self::S>> {
        self.screen.clone()
    }
}
