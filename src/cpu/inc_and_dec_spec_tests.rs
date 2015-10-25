use cpu::*;
use constants::*;

fn inc_base_1<F>(inc: F)
    where F: Fn(&mut Cpu6502, u8) -> u8
{

    let mut cpu = Cpu6502::new();

    let val = inc(&mut cpu, 1);

    assert_eq!(2, val);
    assert_eq!(false, cpu.registers.get_flag(FL_ZERO));
    assert_eq!(false, cpu.registers.get_flag(FL_SIGN));
}

fn inc_base_2<F>(inc: F)
    where F: Fn(&mut Cpu6502, u8) -> u8
{

    let mut cpu = Cpu6502::new();

    let val = inc(&mut cpu, 0xff);

    assert_eq!(0, val);
    assert_eq!(true, cpu.registers.get_flag(FL_ZERO));
    assert_eq!(false, cpu.registers.get_flag(FL_SIGN));
}

fn inc_base_3<F>(inc: F)
    where F: Fn(&mut Cpu6502, u8) -> u8
{

    const ORIG_VAL: u8 = 0xfe;

    // sanity check
    assert_eq!(-2, ORIG_VAL as i8);

    let mut cpu = Cpu6502::new();

    let val = inc(&mut cpu, ORIG_VAL);

    assert_eq!(-1, val as i8);
    assert_eq!(false, cpu.registers.get_flag(FL_ZERO));
    assert_eq!(true, cpu.registers.get_flag(FL_SIGN));
}

fn dec_base_1<F>(dec: F)
    where F: Fn(&mut Cpu6502, u8) -> u8
{

    let mut cpu = Cpu6502::new();

    let val = dec(&mut cpu, 1);

    assert_eq!(0, val);
    assert_eq!(true, cpu.registers.get_flag(FL_ZERO));
    assert_eq!(false, cpu.registers.get_flag(FL_SIGN));
}

fn dec_base_2<F>(dec: F)
    where F: Fn(&mut Cpu6502, u8) -> u8
{

    let mut cpu = Cpu6502::new();

    let val = dec(&mut cpu, 0x2);

    assert_eq!(1, val);
    assert_eq!(false, cpu.registers.get_flag(FL_ZERO));
    assert_eq!(false, cpu.registers.get_flag(FL_SIGN));
}

fn dec_base_3<F>(dec: F)
    where F: Fn(&mut Cpu6502, u8) -> u8
{

    const ORIG_VAL: u8 = 0xfe;

    // sanity check
    assert_eq!(-2, ORIG_VAL as i8);

    let mut cpu = Cpu6502::new();

    let val = dec(&mut cpu, ORIG_VAL);

    assert_eq!(-3, val as i8);
    assert_eq!(false, cpu.registers.get_flag(FL_ZERO));
    assert_eq!(true, cpu.registers.get_flag(FL_SIGN));
}

#[test]
fn inc_1() {
    inc_base_1(|ref mut cpu, val| {
        cpu.memory.store(0x0, val);
        cpu.inc(0x0);
        cpu.memory.load(0x0)
    });
}

#[test]
fn inc_2() {
    inc_base_2(|ref mut cpu, val| {
        cpu.memory.store(0x0, val);
        cpu.inc(0x0);
        cpu.memory.load(0x0)
    });
}

#[test]
fn inc_3() {
    inc_base_3(|ref mut cpu, val| {
        cpu.memory.store(0x0, val);
        cpu.inc(0x0);
        cpu.memory.load(0x0)
    });
}

#[test]
fn dec_1() {
    dec_base_1(|ref mut cpu, val| {
        cpu.memory.store(0x0, val);
        cpu.dec(0x0);
        cpu.memory.load(0x0)
    });
}

#[test]
fn dec_2() {
    dec_base_2(|ref mut cpu, val| {
        cpu.memory.store(0x0, val);
        cpu.dec(0x0);
        cpu.memory.load(0x0)
    });
}

#[test]
fn dec_3() {
    dec_base_3(|ref mut cpu, val| {
        cpu.memory.store(0x0, val);
        cpu.dec(0x0);
        cpu.memory.load(0x0)
    });
}

#[test]
fn inx_1() {
    inc_base_1(|ref mut cpu, val| {
        cpu.registers.irx = val;
        cpu.inx();
        cpu.registers.irx
    });
}

#[test]
fn inx_2() {
    inc_base_2(|ref mut cpu, val| {
        cpu.registers.irx = val;
        cpu.inx();
        cpu.registers.irx
    });
}

#[test]
fn inx_3() {
    inc_base_3(|ref mut cpu, val| {
        cpu.registers.irx = val;
        cpu.inx();
        cpu.registers.irx
    });
}

#[test]
fn dex_1() {
    dec_base_1(|ref mut cpu, val| {
        cpu.registers.irx = val;
        cpu.dex();
        cpu.registers.irx
    });
}

#[test]
fn dex_2() {
    dec_base_2(|ref mut cpu, val| {
        cpu.registers.irx = val;
        cpu.dex();
        cpu.registers.irx
    });
}

#[test]
fn dex_3() {
    dec_base_3(|ref mut cpu, val| {
        cpu.registers.irx = val;
        cpu.dex();
        cpu.registers.irx
    });
}

#[test]
fn iny_1() {
    inc_base_1(|ref mut cpu, val| {
        cpu.registers.iry = val;
        cpu.iny();
        cpu.registers.iry
    });
}

#[test]
fn iny_2() {
    inc_base_2(|ref mut cpu, val| {
        cpu.registers.iry = val;
        cpu.iny();
        cpu.registers.iry
    });
}

#[test]
fn iny_3() {
    inc_base_3(|ref mut cpu, val| {
        cpu.registers.iry = val;
        cpu.iny();
        cpu.registers.iry
    });
}

#[test]
fn dey_1() {
    dec_base_1(|ref mut cpu, val| {
        cpu.registers.iry = val;
        cpu.dey();
        cpu.registers.iry
    });
}

#[test]
fn dey_2() {
    dec_base_2(|ref mut cpu, val| {
        cpu.registers.iry = val;
        cpu.dey();
        cpu.registers.iry
    });
}

#[test]
fn dey_3() {
    dec_base_3(|ref mut cpu, val| {
        cpu.registers.iry = val;
        cpu.dey();
        cpu.registers.iry
    });
}
