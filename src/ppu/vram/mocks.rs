use ppu::control_register::IncrementAmount;
use ppu::vram::IVram;
use ppu::write_latch::LatchState;
use std::cell::Cell;

#[derive(Default)]
pub struct MockVram {
    pub mock_addr: Cell<u8>,
    pub mock_data: Cell<u8>,
    pub scroll_write_called: Cell<bool>,
    pub control_write_called: Cell<bool>,
    pub coarse_x_increment_called: Cell<bool>,
    pub fine_y_increment_called: Cell<bool>,
    pub copy_horizontal_pos_to_addr_called: Cell<bool>,
    pub copy_vertical_pos_to_addr_called: Cell<bool>,
}

impl MockVram {
    pub fn reset_mock(&self) {
        self.mock_addr.set(0);
        self.mock_data.set(0);
        self.scroll_write_called.set(false);
        self.control_write_called.set(false);
        self.coarse_x_increment_called.set(false);
        self.fine_y_increment_called.set(false);
        self.copy_horizontal_pos_to_addr_called.set(false);
        self.copy_vertical_pos_to_addr_called.set(false);
    }
}

impl IVram for MockVram {
    fn write_ppu_addr(&self, latch_state: LatchState) {
        let val = match latch_state {
            LatchState::FirstWrite(val) => val,
            LatchState::SecondWrite(val) => val,
        };

        self.mock_addr.set(val)
    }

    fn write_ppu_data(&mut self, val: u8, _: IncrementAmount) {
        self.mock_data.set(val);
    }

    fn read_ppu_data(&self, _: IncrementAmount) -> u8 {
        self.mock_data.get()
    }

    fn ppu_data(&self) -> u8 {
        self.mock_data.get()
    }

    fn read(&self, _: u16) -> u8 {
        0
    }

    fn addr(&self) -> u16 {
        0
    }

    fn scroll_write(&self, _: LatchState) {
        self.scroll_write_called.set(true)
    }

    fn control_write(&self, _: u8) {
        self.control_write_called.set(true)
    }

    fn coarse_x_increment(&self) {
        self.coarse_x_increment_called.set(true)
    }

    fn fine_y_increment(&self) {
        self.fine_y_increment_called.set(true)
    }

    fn copy_horizontal_pos_to_addr(&self) {
        self.copy_horizontal_pos_to_addr_called.set(true)
    }

    fn copy_vertical_pos_to_addr(&self) {
        self.copy_vertical_pos_to_addr_called.set(true)
    }

    fn fine_x(&self) -> u8 {
        0
    }
}
