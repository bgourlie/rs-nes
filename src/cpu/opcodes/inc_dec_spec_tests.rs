use cpu::*;
use cpu::opcodes::*;
use cpu::opcodes::am_test_utils::*;

#[test]
fn dec_test1() {
    dec_base_1(|ref mut cpu, val| {
        let am = WriterAddressingMode::with_read_value(val);
        let write_ref = am.write_ref();
        Dec::execute(cpu, am);
        write_ref.get()
    });
}

#[test]
fn dec_test2() {
    dec_base_2(|ref mut cpu, val| {
        let am = WriterAddressingMode::with_read_value(val);
        let write_ref = am.write_ref();
        Dec::execute(cpu, am);
        write_ref.get()
    });
}

#[test]
fn dec_test3() {
    dec_base_3(|ref mut cpu, val| {
        let am = WriterAddressingMode::with_read_value(val);
        let write_ref = am.write_ref();
        Dec::execute(cpu, am);
        write_ref.get()
    });
}

#[test]
fn dex_test1() {
    dec_base_1(|ref mut cpu, val| {
        cpu.registers.x = val;
        Dex::execute(cpu, Implied);
        cpu.registers.x
    });
}

#[test]
fn dex_test2() {
    dec_base_2(|ref mut cpu, val| {
        cpu.registers.x = val;
        Dex::execute(cpu, Implied);
        cpu.registers.x
    });
}

#[test]
fn dex_test3() {
    dec_base_3(|ref mut cpu, val| {
        cpu.registers.x = val;
        Dex::execute(cpu, Implied);
        cpu.registers.x
    });
}

#[test]
fn dey_test1() {
    dec_base_1(|ref mut cpu, val| {
        cpu.registers.y = val;
        Dey::execute(cpu, Implied);
        cpu.registers.y
    });
}

#[test]
fn dey_test2() {
    dec_base_2(|ref mut cpu, val| {
        cpu.registers.y = val;
        Dey::execute(cpu, Implied);
        cpu.registers.y
    });
}

#[test]
fn dey_test3() {
    dec_base_3(|ref mut cpu, val| {
        cpu.registers.y = val;
        Dey::execute(cpu, Implied);
        cpu.registers.y
    });
}

#[test]
fn inc_test1() {
    inc_base_1(|ref mut cpu, val| {
        let am = WriterAddressingMode::with_read_value(val);
        let write_ref = am.write_ref();
        Inc::execute(cpu, am);
        write_ref.get()
    });
}

#[test]
fn inc_test2() {
    inc_base_2(|ref mut cpu, val| {
        let am = WriterAddressingMode::with_read_value(val);
        let write_ref = am.write_ref();
        Inc::execute(cpu, am);
        write_ref.get()
    });
}

#[test]
fn inc_test3() {
    inc_base_3(|ref mut cpu, val| {
        let am = WriterAddressingMode::with_read_value(val);
        let write_ref = am.write_ref();
        Inc::execute(cpu, am);
        write_ref.get()
    });
}

#[test]
fn inx_test1() {
    inc_base_1(|ref mut cpu, val| {
        cpu.registers.x = val;
        Inx::execute(cpu, Implied);
        cpu.registers.x
    });
}

#[test]
fn inx_test2() {
    inc_base_2(|ref mut cpu, val| {
        cpu.registers.x = val;
        Inx::execute(cpu, Implied);
        cpu.registers.x
    });
}

#[test]
fn inx_test3() {
    inc_base_3(|ref mut cpu, val| {
        cpu.registers.x = val;
        Inx::execute(cpu, Implied);
        cpu.registers.x
    });
}

#[test]
fn iny_test1() {
    inc_base_1(|ref mut cpu, val| {
        cpu.registers.y = val;
        Iny::execute(cpu, Implied);
        cpu.registers.y
    });
}

#[test]
fn iny_test2() {
    inc_base_2(|ref mut cpu, val| {
        cpu.registers.y = val;
        Iny::execute(cpu, Implied);
        cpu.registers.y
    });
}

#[test]
fn iny_test3() {
    inc_base_3(|ref mut cpu, val| {
        cpu.registers.y = val;
        Iny::execute(cpu, Implied);
        cpu.registers.y
    });
}

fn inc_base_1<F>(inc: F)
where
    F: Fn(&mut TestCpu, u8) -> u8,
{
    let mut cpu = TestCpu::new_test();
    let val = inc(&mut cpu, 1);
    assert_eq!(2, val);
    assert_eq!(false, cpu.registers.zero_flag());
    assert_eq!(false, cpu.registers.sign_flag());
}

fn inc_base_2<F>(inc: F)
where
    F: Fn(&mut TestCpu, u8) -> u8,
{
    let mut cpu = TestCpu::new_test();
    let val = inc(&mut cpu, 255);
    assert_eq!(0, val);
    assert_eq!(true, cpu.registers.zero_flag());
    assert_eq!(false, cpu.registers.sign_flag());
}

fn inc_base_3<F>(inc: F)
where
    F: Fn(&mut TestCpu, u8) -> u8,
{
    let mut cpu = TestCpu::new_test();
    let val = inc(&mut cpu, 254);
    assert_eq!(-1, val as i8);
    assert_eq!(false, cpu.registers.zero_flag());
    assert_eq!(true, cpu.registers.sign_flag());
}

fn dec_base_1<F>(dec: F)
where
    F: Fn(&mut TestCpu, u8) -> u8,
{
    let mut cpu = TestCpu::new_test();
    let val = dec(&mut cpu, 1);
    assert_eq!(0, val);
    assert_eq!(true, cpu.registers.zero_flag());
    assert_eq!(false, cpu.registers.sign_flag());
}

fn dec_base_2<F>(dec: F)
where
    F: Fn(&mut TestCpu, u8) -> u8,
{
    let mut cpu = TestCpu::new_test();
    let val = dec(&mut cpu, 2);
    assert_eq!(1, val);
    assert_eq!(false, cpu.registers.zero_flag());
    assert_eq!(false, cpu.registers.sign_flag());
}

fn dec_base_3<F>(dec: F)
where
    F: Fn(&mut TestCpu, u8) -> u8,
{
    let mut cpu = TestCpu::new_test();
    let val = dec(&mut cpu, 254);
    assert_eq!(-3, val as i8);
    assert_eq!(false, cpu.registers.zero_flag());
    assert_eq!(true, cpu.registers.sign_flag());
}
