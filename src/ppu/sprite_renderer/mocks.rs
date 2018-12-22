use crate::{
    cart::Cart,
    ppu::{
        control_register::ControlRegister,
        sprite_renderer::{ISpriteRenderer, SpritePixel},
        vram::IVram,
    },
};
use std::cell::Cell;

#[derive(Default)]
pub struct MockSpriteRenderer {
    pub read_data_called: Cell<bool>,
    pub read_data_increment_addr_called: Cell<bool>,
    pub mock_addr: Cell<u8>,
    pub mock_data: Cell<u8>,
}

impl ISpriteRenderer for MockSpriteRenderer {
    fn read_data(&self) -> u8 {
        self.read_data_called.set(true);
        self.mock_data.get()
    }

    fn read_data_increment_addr(&self) -> u8 {
        self.read_data_increment_addr_called.set(true);
        self.mock_data.get()
    }

    fn write_address(&mut self, addr: u8) {
        self.mock_addr.set(addr)
    }

    fn write_data(&mut self, val: u8) {
        self.mock_data.set(val)
    }

    fn update_palettes<V: IVram>(&mut self, _: &V) {}

    fn dec_x_counters(&mut self) {}

    fn start_sprite_evaluation(&mut self, _: u16, _: ControlRegister) {}

    fn tick_sprite_evaluation(&mut self) {}

    fn fill_registers<V: IVram, C: Cart>(&mut self, _: &V, _: ControlRegister, _: &C) {}

    fn current_pixel(&self) -> SpritePixel {
        SpritePixel {
            value: 0,
            color_index: 0,
            has_priority: true,
            is_sprite_zero: false,
        }
    }
}
