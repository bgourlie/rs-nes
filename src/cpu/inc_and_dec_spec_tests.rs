use cpu::*;
use memory::*;

fn inc_base_1<F>(inc: F)
    where F: Fn(&mut TestCpu, u8) -> u8
{
    let mut cpu = TestCpu::new_test();

    let val = inc(&mut cpu, 1);

    assert_eq!(2, val);
    assert_eq!(false, cpu.registers.zero_flag());
    assert_eq!(false, cpu.registers.sign_flag());
}

fn inc_base_2<F>(inc: F)
    where F: Fn(&mut TestCpu, u8) -> u8
{
    let mut cpu = TestCpu::new_test();

    let val = inc(&mut cpu, 0xff);

    assert_eq!(0, val);
    assert_eq!(true, cpu.registers.zero_flag());
    assert_eq!(false, cpu.registers.sign_flag());
}

fn inc_base_3<F>(inc: F)
    where F: Fn(&mut TestCpu, u8) -> u8
{

    const ORIG_VAL: u8 = 0xfe;

    // sanity check
    assert_eq!(-2, ORIG_VAL as i8);

    let mut cpu = TestCpu::new_test();
    let val = inc(&mut cpu, ORIG_VAL);

    assert_eq!(-1, val as i8);
    assert_eq!(false, cpu.registers.zero_flag());
    assert_eq!(true, cpu.registers.sign_flag());
}

fn dec_base_1<F>(dec: F)
    where F: Fn(&mut TestCpu, u8) -> u8
{

    let mut cpu = TestCpu::new_test();

    let val = dec(&mut cpu, 1);

    assert_eq!(0, val);
    assert_eq!(true, cpu.registers.zero_flag());
    assert_eq!(false, cpu.registers.sign_flag());
}

fn dec_base_2<F>(dec: F)
    where F: Fn(&mut TestCpu, u8) -> u8
{

    let mut cpu = TestCpu::new_test();

    let val = dec(&mut cpu, 0x2);

    assert_eq!(1, val);
    assert_eq!(false, cpu.registers.zero_flag());
    assert_eq!(false, cpu.registers.sign_flag());
}

fn dec_base_3<F>(dec: F)
    where F: Fn(&mut TestCpu, u8) -> u8
{

    const ORIG_VAL: u8 = 0xfe;

    // sanity check
    assert_eq!(-2, ORIG_VAL as i8);

    let mut cpu = TestCpu::new_test();

    let val = dec(&mut cpu, ORIG_VAL);

    assert_eq!(-3, val as i8);
    assert_eq!(false, cpu.registers.zero_flag());
    assert_eq!(true, cpu.registers.sign_flag());
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
        cpu.registers.x = val;
        cpu.inx();
        cpu.registers.x
    });
}

#[test]
fn inx_2() {
    inc_base_2(|ref mut cpu, val| {
        cpu.registers.x = val;
        cpu.inx();
        cpu.registers.x
    });
}

#[test]
fn inx_3() {
    inc_base_3(|ref mut cpu, val| {
        cpu.registers.x = val;
        cpu.inx();
        cpu.registers.x
    });
}

#[test]
fn dex_1() {
    dec_base_1(|ref mut cpu, val| {
        cpu.registers.x = val;
        cpu.dex();
        cpu.registers.x
    });
}

#[test]
fn dex_2() {
    dec_base_2(|ref mut cpu, val| {
        cpu.registers.x = val;
        cpu.dex();
        cpu.registers.x
    });
}

#[test]
fn dex_3() {
    dec_base_3(|ref mut cpu, val| {
        cpu.registers.x = val;
        cpu.dex();
        cpu.registers.x
    });
}

#[test]
fn iny_1() {
    inc_base_1(|ref mut cpu, val| {
        cpu.registers.y = val;
        cpu.iny();
        cpu.registers.y
    });
}

#[test]
fn iny_2() {
    inc_base_2(|ref mut cpu, val| {
        cpu.registers.y = val;
        cpu.iny();
        cpu.registers.y
    });
}

#[test]
fn iny_3() {
    inc_base_3(|ref mut cpu, val| {
        cpu.registers.y = val;
        cpu.iny();
        cpu.registers.y
    });
}

#[test]
fn dey_1() {
    dec_base_1(|ref mut cpu, val| {
        cpu.registers.y = val;
        cpu.dey();
        cpu.registers.y
    });
}

#[test]
fn dey_2() {
    dec_base_2(|ref mut cpu, val| {
        cpu.registers.y = val;
        cpu.dey();
        cpu.registers.y
    });
}

#[test]
fn dey_3() {
    dec_base_3(|ref mut cpu, val| {
        cpu.registers.y = val;
        cpu.dey();
        cpu.registers.y
    });
}
