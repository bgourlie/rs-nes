use memory::Memory;

pub struct Vram {
    pattern0: [u8; 4096],
    pattern1: [u8; 4096],
    nametable0: [u8; 1024],
    nametable1: [u8; 1024],
    nametable2: [u8; 1024],
    nametable3: [u8; 1024],
    palette: [u8; 32],
    oam: [u8; 256],
}

impl Vram {
    pub fn new() -> Self {
        Vram {
            pattern0: [0; 4096],
            pattern1: [0; 4096],
            nametable0: [0; 1024],
            nametable1: [0; 1024],
            nametable2: [0; 1024],
            nametable3: [0; 1024],
            palette: [0; 32],
            oam: [0; 256],
        }
    }
}

impl Memory for Vram {
    fn store(&mut self, addr: u16, data: u8) {
        unimplemented!();
    }

    fn load(&self, addr: u16) -> u8 {
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
