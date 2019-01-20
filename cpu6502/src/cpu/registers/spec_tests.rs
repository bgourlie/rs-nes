use crate::cpu::registers::Registers;

#[test]
fn carry_flag() {
    let regs = new_with_status(0b00000000);
    assert_eq!(false, regs.carry_flag());

    let regs = new_with_status(0b00000001);
    assert_eq!(true, regs.carry_flag())
}

#[test]
fn zero_flag() {
    let regs = new_with_status(0b00000000);
    assert_eq!(false, regs.zero_flag());

    let regs = new_with_status(0b00000010);
    assert_eq!(true, regs.zero_flag())
}

#[test]
fn zero_interrupt_disable() {
    let regs = new_with_status(0b00000000);
    assert_eq!(false, regs.interrupt_disable_flag());

    let regs = new_with_status(0b00000100);
    assert_eq!(true, regs.interrupt_disable_flag())
}

#[test]
fn decimal_flag() {
    let regs = new_with_status(0b00000000);
    assert_eq!(false, regs.decimal_flag());

    let regs = new_with_status(0b00001000);
    assert_eq!(true, regs.decimal_flag())
}

#[test]
fn overflow_flag() {
    let regs = new_with_status(0b00000000);
    assert_eq!(false, regs.overflow_flag());

    let regs = new_with_status(0b01000000);
    assert_eq!(true, regs.overflow_flag())
}

#[test]
fn sign_flag() {
    let regs = new_with_status(0b00000000);
    assert_eq!(false, regs.sign_flag());

    let regs = new_with_status(0b10000000);
    assert_eq!(true, regs.sign_flag())
}

#[test]
fn set_status_from_stack() {
    // Break and unused bits are not overwritten when pulling from stack
    let mut regs = new_with_status(0b00110000);
    regs.set_status_from_stack(0b00000000);
    assert_eq!(0b00110000, regs.status);
}

fn new_with_status(stat: u8) -> Registers {
    let mut regs = Registers::new();
    regs.status = stat;
    regs
}
