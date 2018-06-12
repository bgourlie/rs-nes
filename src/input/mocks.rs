use input::{Button, IInput};

#[derive(Default)]
pub struct InputMock;

impl IInput for InputMock {
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
