use std::cell::Cell;

#[derive(Default)]
pub struct InputBase {
    probe: u8,
    controllers: ControllerState,
}

/// Stores both controller states, low byte it controller one, high byte is controller 2
#[derive(Default)]
pub struct ControllerState {
    state: Cell<u16>,
}

impl ControllerState {
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

pub trait Input: Default {
    fn write_probe(&mut self, _: u8);
    fn read_joy_1(&self) -> u8;
    fn read_joy_2(&self) -> u8;
}

impl Input for InputBase {
    fn write_probe(&mut self, val: u8) {
        self.probe = val;
    }

    fn read_joy_1(&self) -> u8 {
        0
    }

    fn read_joy_2(&self) -> u8 {
        0
    }
}
