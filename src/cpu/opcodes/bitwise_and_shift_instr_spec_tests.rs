use cpu::*;
use cpu::opcodes::*;

fn asl(cpu: &mut TestCpu, val: u8) -> (u8, bool) {
    Asl::execute(cpu, val);
    (cpu.registers.acc, false)
}

#[test]
fn asl_1() {
    shift_left_base_1(asl);
}

#[test]
fn asl_2() {
    shift_left_base_2(asl);
}

#[test]
fn asl_3() {
    shift_left_base_3(asl);
}

fn lsr(cpu: &mut TestCpu, val: u8) -> (u8, bool) {
    Lsr::execute(cpu, val);
    (cpu.registers.acc, false)
}

#[test]
fn lsr_1() {
    shift_right_base_1(lsr);
}

#[test]
fn lsr_2() {
    shift_right_base_2(lsr);
}

#[test]
fn lsr_3() {
    shift_right_base_3(lsr);
}

fn ror(cpu: &mut TestCpu, val: u8) -> (u8, bool) {
    Ror::execute(cpu, val);
    (cpu.registers.acc, true)
}

#[test]
fn ror_1() {
    shift_right_base_1(ror);
}

#[test]
fn ror_2() {
    shift_right_base_2(ror);
}

#[test]
fn ror_3() {
    shift_right_base_3(ror);
}

fn rol(cpu: &mut TestCpu, val: u8) -> (u8, bool) {
    Rol::execute(cpu, val);
    (cpu.registers.acc, true)
}

#[test]
fn rol_1() {
    shift_left_base_1(rol);
}

#[test]
fn rol_2() {
    shift_left_base_2(rol);
}

#[test]
fn rol_3() {
    shift_left_base_3(rol);
}

#[test]
fn and() {
    let mut cpu = TestCpu::new_test();
    cpu.registers.acc = 0_u8;
    And::execute(&mut cpu, 255_u8);
    assert_eq!(0, cpu.registers.acc);
    assert_eq!(true, cpu.registers.zero_flag());
    assert_eq!(false, cpu.registers.sign_flag());

    let mut cpu = TestCpu::new_test();
    cpu.registers.acc = 0b11110000_u8;
    And::execute(&mut cpu, 0b10101010_u8);
    assert_eq!(0b10100000, cpu.registers.acc);
    assert_eq!(false, cpu.registers.zero_flag());
    assert_eq!(true, cpu.registers.sign_flag());
}

#[test]
fn bit_zero_flag_behavior() {
    let mut cpu = TestCpu::new_test();
    cpu.registers.acc = 0;
    Bit::execute(&mut cpu, 0_u8);
    assert_eq!(true, cpu.registers.zero_flag());

    let mut cpu = TestCpu::new_test();
    cpu.registers.acc = 0b11110000;
    Bit::execute(&mut cpu, 0b00001111_u8);
    assert_eq!(true, cpu.registers.zero_flag());

    let mut cpu = TestCpu::new_test();
    cpu.registers.acc = 0b00111100;
    Bit::execute(&mut cpu, 0b00011000_u8);
    assert_eq!(false, cpu.registers.zero_flag());
}

#[test]
fn bit_sign_flag_behavior() {
    let mut cpu = TestCpu::new_test();
    Bit::execute(&mut cpu, 0b01111111_u8);
    assert_eq!(false, cpu.registers.sign_flag());

    let mut cpu = TestCpu::new_test();
    Bit::execute(&mut cpu, 0b10000000_u8);
    assert_eq!(true, cpu.registers.sign_flag());
}

#[test]
fn bit_overflow_flag_behavior() {
    let mut cpu = TestCpu::new_test();
    Bit::execute(&mut cpu, 0b10111111_u8);
    assert_eq!(false, cpu.registers.overflow_flag());

    let mut cpu = TestCpu::new_test();
    Bit::execute(&mut cpu, 0b01000000_u8);
    assert_eq!(true, cpu.registers.overflow_flag());
}

fn shift_left_base_1<F>(do_shift: F)
    where F: Fn(&mut TestCpu, u8) -> (u8, bool)
{

    const VAL: u8 = 0b10000001;
    let mut cpu = TestCpu::new_test();

    cpu.registers.set_carry_flag(true);
    let (result, rotate) = do_shift(&mut cpu, VAL);

    if rotate {
        assert_eq!(0b00000011, result);
    } else {
        assert_eq!(0b00000010, result);
    }

    assert_eq!(false, cpu.registers.sign_flag());
    assert_eq!(true, cpu.registers.carry_flag());
    assert_eq!(false, cpu.registers.zero_flag());
}

fn shift_left_base_2<F>(do_shift: F)
    where F: Fn(&mut TestCpu, u8) -> (u8, bool)
{

    const VAL: u8 = 0b01000000;
    let mut cpu = TestCpu::new_test();

    let (result, _) = do_shift(&mut cpu, VAL);

    assert_eq!(0b10000000, result);

    assert_eq!(true, cpu.registers.sign_flag());
    assert_eq!(false, cpu.registers.carry_flag());
    assert_eq!(false, cpu.registers.zero_flag());
}

fn shift_left_base_3<F>(do_shift: F)
    where F: Fn(&mut TestCpu, u8) -> (u8, bool)
{

    const VAL: u8 = 0b00000000;
    let mut cpu = TestCpu::new_test();

    let (result, _) = do_shift(&mut cpu, VAL);

    assert_eq!(0b00000000, result);

    assert_eq!(false, cpu.registers.sign_flag());
    assert_eq!(false, cpu.registers.carry_flag());
    assert_eq!(true, cpu.registers.zero_flag());
}

fn shift_right_base_1<F>(do_shift: F)
    where F: Fn(&mut TestCpu, u8) -> (u8, bool)
{

    const VAL: u8 = 0b10000001;

    let mut cpu = TestCpu::new_test();

    cpu.registers.set_carry_flag(true);
    let (result, rotate) = do_shift(&mut cpu, VAL);

    if rotate {
        assert_eq!(0b11000000, result);
        assert_eq!(true, cpu.registers.sign_flag());
    } else {
        assert_eq!(0b01000000, result);
        assert_eq!(false, cpu.registers.sign_flag());
    }

    assert_eq!(true, cpu.registers.carry_flag());
    assert_eq!(false, cpu.registers.zero_flag());
}

fn shift_right_base_2<F>(do_shift: F)
    where F: Fn(&mut TestCpu, u8) -> (u8, bool)
{

    const VAL: u8 = 0b01000000;

    let mut cpu = TestCpu::new_test();

    let (result, _) = do_shift(&mut cpu, VAL);

    assert_eq!(0b00100000, result);

    assert_eq!(false, cpu.registers.sign_flag());
    assert_eq!(false, cpu.registers.carry_flag());
    assert_eq!(false, cpu.registers.zero_flag());
}

fn shift_right_base_3<F>(do_shift: F)
    where F: Fn(&mut TestCpu, u8) -> (u8, bool)
{

    const VAL: u8 = 0b00000000;

    let mut cpu = TestCpu::new_test();

    let (result, _) = do_shift(&mut cpu, VAL);

    assert_eq!(0b00000000, result);

    assert_eq!(false, cpu.registers.sign_flag());
    assert_eq!(false, cpu.registers.carry_flag());
    assert_eq!(true, cpu.registers.zero_flag());
}
