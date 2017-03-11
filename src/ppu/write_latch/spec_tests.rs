use super::*;

#[test]
fn write() {
    let latch = WriteLatch::default();
    assert_eq!(LatchState::FirstWrite(0xde), latch.write(0xde));
    assert_eq!(LatchState::SecondWrite(0xad), latch.write(0xad));
}

#[test]
fn clear_latch() {
    let latch = WriteLatch::default();
    assert_eq!(LatchState::FirstWrite(0xff), latch.write(0xff));
    latch.clear();
    assert_eq!(LatchState::FirstWrite(0xfe), latch.write(0xfe));
}
