use cpu::*;
use cpu::addressing::Accumulator;
use constants::*;

fn shift_left_base_1<F>(do_shift: F)
    where F: Fn(&mut TestCpu, u8) -> (u8, bool)
{

    const VAL: u8 = 0b10000001;
    let mut cpu = TestCpu::new_test();

    cpu.registers.set_flag(FL_CARRY, true);
    let (result, rotate) = do_shift(&mut cpu, VAL);

    if rotate {
        assert_eq!(0b00000011, result);
    } else {
        assert_eq!(0b00000010, result);
    }

    assert_eq!(false, cpu.registers.get_flag(FL_SIGN));
    assert_eq!(true, cpu.registers.get_flag(FL_CARRY));
    assert_eq!(false, cpu.registers.get_flag(FL_ZERO));
}

fn shift_left_base_2<F>(do_shift: F)
    where F: Fn(&mut TestCpu, u8) -> (u8, bool)
{

    const VAL: u8 = 0b01000000;
    let mut cpu = TestCpu::new_test();

    let (result, _) = do_shift(&mut cpu, VAL);

    assert_eq!(0b10000000, result);

    assert_eq!(true, cpu.registers.get_flag(FL_SIGN));
    assert_eq!(false, cpu.registers.get_flag(FL_CARRY));
    assert_eq!(false, cpu.registers.get_flag(FL_ZERO));
}

fn shift_left_base_3<F>(do_shift: F)
    where F: Fn(&mut TestCpu, u8) -> (u8, bool)
{

    const VAL: u8 = 0b00000000;
    let mut cpu = TestCpu::new_test();

    let (result, _) = do_shift(&mut cpu, VAL);

    assert_eq!(0b00000000, result);

    assert_eq!(false, cpu.registers.get_flag(FL_SIGN));
    assert_eq!(false, cpu.registers.get_flag(FL_CARRY));
    assert_eq!(true, cpu.registers.get_flag(FL_ZERO));
}

fn shift_right_base_1<F>(do_shift: F)
    where F: Fn(&mut TestCpu, u8) -> (u8, bool)
{

    const VAL: u8 = 0b10000001;

    let mut cpu = TestCpu::new_test();

    cpu.registers.set_flag(FL_CARRY, true);
    let (result, rotate) = do_shift(&mut cpu, VAL);

    if rotate {
        assert_eq!(0b11000000, result);
        assert_eq!(true, cpu.registers.get_flag(FL_SIGN));
    } else {
        assert_eq!(0b01000000, result);
        assert_eq!(false, cpu.registers.get_flag(FL_SIGN));
    }

    assert_eq!(true, cpu.registers.get_flag(FL_CARRY));
    assert_eq!(false, cpu.registers.get_flag(FL_ZERO));
}

fn shift_right_base_2<F>(do_shift: F)
    where F: Fn(&mut TestCpu, u8) -> (u8, bool)
{

    const VAL: u8 = 0b01000000;

    let mut cpu = TestCpu::new_test();

    let (result, _) = do_shift(&mut cpu, VAL);

    assert_eq!(0b00100000, result);

    assert_eq!(false, cpu.registers.get_flag(FL_SIGN));
    assert_eq!(false, cpu.registers.get_flag(FL_CARRY));
    assert_eq!(false, cpu.registers.get_flag(FL_ZERO));
}

fn shift_right_base_3<F>(do_shift: F)
    where F: Fn(&mut TestCpu, u8) -> (u8, bool)
{

    const VAL: u8 = 0b00000000;

    let mut cpu = TestCpu::new_test();

    let (result, _) = do_shift(&mut cpu, VAL);

    assert_eq!(0b00000000, result);

    assert_eq!(false, cpu.registers.get_flag(FL_SIGN));
    assert_eq!(false, cpu.registers.get_flag(FL_CARRY));
    assert_eq!(true, cpu.registers.get_flag(FL_ZERO));
}

fn asl(cpu: &mut TestCpu, val: u8) -> (u8, bool) {
    cpu.registers.acc = val;
    cpu.asl(Accumulator);
    (cpu.registers.acc, false)
}

fn rol(cpu: &mut TestCpu, val: u8) -> (u8, bool) {
    cpu.registers.acc = val;
    cpu.rol(Accumulator);
    (cpu.registers.acc, true)
}

fn lsr(cpu: &mut TestCpu, val: u8) -> (u8, bool) {
    cpu.registers.acc = val;
    cpu.lsr(Accumulator);
    (cpu.registers.acc, false)
}

fn ror(cpu: &mut TestCpu, val: u8) -> (u8, bool) {
    cpu.registers.acc = val;
    cpu.ror(Accumulator);
    (cpu.registers.acc, true)
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
