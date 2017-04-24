use super::*;

#[test]
fn read() {
    // Reading this register clears the frame interrupt flag (but not the DMC interrupt flag).
    let status = StatusImpl::default();
    status.val.set(0b_1111_1111);
    let result = status.read();
    assert_eq!(0b_1111_1111, result);
    assert_eq!(0b_1011_1111, status.val.get());
}

#[test]
fn write() {
    // Reading this register clears the frame interrupt flag (but not the DMC interrupt flag).
    let mut status = StatusImpl::default();
    assert_eq!(0, status.val.get());

    status.write_4015(0b_1111_1111);
    assert_eq!(0b_0111_1111, status.val.get());
}
