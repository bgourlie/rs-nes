pub use crate::ppu::{sprite_renderer::mocks::MockSpriteRenderer, vram::mocks::MockVram};
use crate::{
    cart::Cart,
    ppu::{IPpu, PPU_BUFFER_SIZE},
};
use cpu6502::cpu::Interrupt;

pub struct PpuMock {
    addr: u16,
    value: u8,
    screen: [u8; PPU_BUFFER_SIZE],
}

impl Default for PpuMock {
    fn default() -> Self {
        PpuMock {
            addr: 0,
            value: 0,
            screen: [0; PPU_BUFFER_SIZE],
        }
    }
}

impl PpuMock {
    pub fn addr(&self) -> u16 {
        self.addr
    }

    pub fn value(&self) -> u8 {
        self.value
    }

    pub fn set_value(&mut self, value: u8) {
        self.value = value;
    }
}

impl IPpu for PpuMock {
    fn write<C: Cart>(&mut self, addr: u16, val: u8, _: &mut C) {
        self.addr = addr;
        self.value = val;
    }

    fn read<C: Cart>(&self, _: u16, _: &C) -> u8 {
        self.value
    }

    fn step<C: Cart>(&mut self, _: &C) -> Interrupt {
        Interrupt::None
    }

    fn screen(&self) -> &[u8; PPU_BUFFER_SIZE] {
        &self.screen
    }
}
