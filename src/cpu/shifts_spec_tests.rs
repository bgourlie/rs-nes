use cpu::*;
use constants::*;

fn shift_left_base_1<F>(do_shift: F)
    where F: Fn(&mut Cpu6502, u8) -> (u8, bool)
{

    const VAL: u8 = 0b10000001;
    let mut cpu = Cpu6502::new();

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
    where F: Fn(&mut Cpu6502, u8) -> (u8, bool)
{

    const VAL: u8 = 0b01000000;
    let mut cpu = Cpu6502::new();

    let (result, rotate) = do_shift(&mut cpu, VAL);

    assert_eq!(0b10000000, result);

    assert_eq!(true, cpu.registers.get_flag(FL_SIGN));
    assert_eq!(false, cpu.registers.get_flag(FL_CARRY));
    assert_eq!(false, cpu.registers.get_flag(FL_ZERO));
}

fn shift_left_base_3<F>(do_shift: F)
    where F: Fn(&mut Cpu6502, u8) -> (u8, bool)
{

    const VAL: u8 = 0b00000000;
    let mut cpu = Cpu6502::new();

    let (result, rotate) = do_shift(&mut cpu, VAL);

    assert_eq!(0b00000000, result);

    assert_eq!(false, cpu.registers.get_flag(FL_SIGN));
    assert_eq!(false, cpu.registers.get_flag(FL_CARRY));
    assert_eq!(true, cpu.registers.get_flag(FL_ZERO));
}

fn shift_right_base_1<F>(do_shift: F)
    where F: Fn(&mut Cpu6502, u8) -> (u8, bool)
{

    const VAL: u8 = 0b10000001;

    let mut cpu = Cpu6502::new();

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
    where F: Fn(&mut Cpu6502, u8) -> (u8, bool)
{

    const VAL: u8 = 0b01000000;

    let mut cpu = Cpu6502::new();

    let (result, rotate) = do_shift(&mut cpu, VAL);

    assert_eq!(0b00100000, result);

    assert_eq!(false, cpu.registers.get_flag(FL_SIGN));
    assert_eq!(false, cpu.registers.get_flag(FL_CARRY));
    assert_eq!(false, cpu.registers.get_flag(FL_ZERO));
}

fn shift_right_base_3<F>(do_shift: F)
    where F: Fn(&mut Cpu6502, u8) -> (u8, bool)
{

    const VAL: u8 = 0b00000000;

    let mut cpu = Cpu6502::new();

    let (result, rotate) = do_shift(&mut cpu, VAL);

    assert_eq!(0b00000000, result);

    assert_eq!(false, cpu.registers.get_flag(FL_SIGN));
    assert_eq!(false, cpu.registers.get_flag(FL_CARRY));
    assert_eq!(true, cpu.registers.get_flag(FL_ZERO));
}

#[test]
fn asl_1() {
    shift_left_base_1(|ref mut cpu, val| (cpu.asl(val), false));
}

#[test]
fn asl_2() {
    shift_left_base_2(|ref mut cpu, val| (cpu.asl(val), false));
}

#[test]
fn asl_3() {
    shift_left_base_3(|ref mut cpu, val| (cpu.asl(val), false));
}

#[test]
fn rol_1() {
    shift_left_base_1(|ref mut cpu, val| (cpu.rol(val), true));
}

#[test]
fn rol_2() {
    shift_left_base_2(|ref mut cpu, val| (cpu.rol(val), true));
}

#[test]
fn rol_3() {
    shift_left_base_3(|ref mut cpu, val| (cpu.rol(val), true));
}

#[test]
fn lsr_1() {
    shift_right_base_1(|ref mut cpu, val| (cpu.lsr(val), false));
}

#[test]
fn lsr_2() {
    shift_right_base_2(|ref mut cpu, val| (cpu.lsr(val), false));
}

#[test]
fn lsr_3() {
    shift_right_base_3(|ref mut cpu, val| (cpu.lsr(val), false));
}

#[test]
fn ror_1() {
    shift_right_base_1(|ref mut cpu, val| (cpu.ror(val), true));
}

#[test]
fn ror_2() {
    shift_right_base_2(|ref mut cpu, val| (cpu.ror(val), true));
}

#[test]
fn ror_3() {
    shift_right_base_3(|ref mut cpu, val| (cpu.ror(val), true));
}
