use super::*;

#[test]
fn write() {
    let mut reg = ScrollRegister::new();
    assert_eq!(0, reg.x_pos);
    assert_eq!(0, reg.y_pos);

    reg.write(10);
    assert_eq!(10, reg.x_pos);
    assert_eq!(0, reg.y_pos);

    reg.write(11);
    assert_eq!(10, reg.x_pos);
    assert_eq!(11, reg.y_pos);

    reg.write(12);
    assert_eq!(12, reg.x_pos);
    assert_eq!(11, reg.y_pos);

    reg.write(13);
    assert_eq!(12, reg.x_pos);
    assert_eq!(13, reg.y_pos);
}

#[test]
fn clear_latch() {
    let mut reg = ScrollRegister::new();
    assert_eq!(0, reg.x_pos);
    assert_eq!(0, reg.y_pos);

    reg.write(10);
    assert_eq!(10, reg.x_pos);
    assert_eq!(0, reg.y_pos);

    assert_eq!(LatchState::WriteY, reg.latch_state.get());
    reg.clear_latch();
    assert_eq!(LatchState::WriteX, reg.latch_state.get());
}
