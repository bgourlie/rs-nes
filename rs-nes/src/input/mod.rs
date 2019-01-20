#[cfg(test)]
pub mod mocks;

use std::cell::Cell;

const STROBE_A: u8 = 0;
const STROBE_B: u8 = 1;
const STROBE_SELECT: u8 = 2;
const STROBE_START: u8 = 3;
const STROBE_UP: u8 = 4;
const STROBE_DOWN: u8 = 5;
const STROBE_LEFT: u8 = 6;
const STROBE_RIGHT: u8 = 7;

pub enum Button {
    A,
    B,
    Select,
    Start,
    Up,
    Down,
    Left,
    Right,
}

#[derive(Default)]
pub struct Input {
    strobe: Cell<u8>,
    state: Cell<u8>,
}

pub trait IInput: Default {
    fn write(&mut self, addr: u16, val: u8);
    fn read(&self, addr: u16) -> u8;
    fn player1_press(&self, button: Button);
    fn player1_release(&self, button: Button);
}

impl IInput for Input {
    fn write(&mut self, addr: u16, _: u8) {
        if addr == 4016 {
            self.strobe.set(STROBE_A)
        }
    }

    fn read(&self, addr: u16) -> u8 {
        debug_assert!(addr == 0x4016 || addr == 0x4017);
        if addr == 0x4016 {
            let strobe = self.strobe.get();
            let state = self.state.get();
            self.strobe.set((strobe + 1) & 7);
            match strobe {
                STROBE_A => state & 1,
                STROBE_B => (state & (1 << 1)) >> 1,
                STROBE_SELECT => (state & (1 << 2)) >> 2,
                STROBE_START => (state & (1 << 3)) >> 3,
                STROBE_UP => (state & (1 << 4)) >> 4,
                STROBE_DOWN => (state & (1 << 5)) >> 5,
                STROBE_LEFT => (state & (1 << 6)) >> 6,
                STROBE_RIGHT => (state & (1 << 7)) >> 7,
                _ => unreachable!(),
            }
        } else {
            0
        }
    }

    fn player1_press(&self, button: Button) {
        let state = self.state.get();
        match button {
            Button::A => self.state.set(state | 1),
            Button::B => self.state.set(state | (1 << 1)),
            Button::Select => self.state.set(state | (1 << 2)),
            Button::Start => self.state.set(state | (1 << 3)),
            Button::Up => self.state.set(state | (1 << 4)),
            Button::Down => self.state.set(state | (1 << 5)),
            Button::Left => self.state.set(state | (1 << 6)),
            Button::Right => self.state.set(state | (1 << 7)),
        }
    }

    fn player1_release(&self, button: Button) {
        let state = self.state.get();
        match button {
            Button::A => self.state.set(state & !1),
            Button::B => self.state.set(state & !(1 << 1)),
            Button::Select => self.state.set(state & !(1 << 2)),
            Button::Start => self.state.set(state & !(1 << 3)),
            Button::Up => self.state.set(state & !(1 << 4)),
            Button::Down => self.state.set(state & !(1 << 5)),
            Button::Left => self.state.set(state & !(1 << 6)),
            Button::Right => self.state.set(state & !(1 << 7)),
        }
    }
}

#[derive(Default)]
pub struct NoInput;

impl IInput for NoInput {
    fn write(&mut self, _: u16, _: u8) {}

    fn read(&self, _: u16) -> u8 {
        0
    }

    fn player1_press(&self, _: Button) {
        unimplemented!()
    }

    fn player1_release(&self, _: Button) {
        unimplemented!()
    }
}
