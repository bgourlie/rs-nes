use super::*;
use ppu::write_latch::LatchState;

#[test]
fn write() {
    let mut reg = ScrollRegisterBase::default();
    assert_eq!(0, reg.x_pos);
    assert_eq!(0, reg.y_pos);

    reg.write(LatchState::FirstWrite(10));
    assert_eq!(10, reg.x_pos);
    assert_eq!(0, reg.y_pos);

    reg.write(LatchState::SecondWrite(11));
    assert_eq!(10, reg.x_pos);
    assert_eq!(11, reg.y_pos);

    reg.write(LatchState::FirstWrite(12));
    assert_eq!(12, reg.x_pos);
    assert_eq!(11, reg.y_pos);

    reg.write(LatchState::SecondWrite(13));
    assert_eq!(12, reg.x_pos);
    assert_eq!(13, reg.y_pos);
}
