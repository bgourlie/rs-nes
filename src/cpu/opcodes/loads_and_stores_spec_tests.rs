use cpu::opcodes::am_test_utils::*;
use cpu::opcodes::*;
use cpu::*;

#[test]
fn lda_value_set() {
    let mut cpu = TestCpu::new_test();
    Lda::execute(&mut cpu, 0xff_u8);
    assert_eq!(0xff, cpu.registers.acc);
}

#[test]
fn lda_flags_sign_and_zero_1() {
    let mut cpu = TestCpu::new_test();
    Lda::execute(&mut cpu, 0x0_u8);
    assert_eq!(true, cpu.registers.zero_flag());
    assert_eq!(false, cpu.registers.sign_flag());
}

#[test]
fn lda_flags_sign_and_zero_2() {
    let mut cpu = TestCpu::new_test();
    Lda::execute(&mut cpu, 0x1_u8);
    assert_eq!(false, cpu.registers.zero_flag());
    assert_eq!(false, cpu.registers.sign_flag());
}

#[test]
fn lda_flags_sign_and_zero_3() {
    let mut cpu = TestCpu::new_test();
    Lda::execute(&mut cpu, 0x80_u8);
    assert_eq!(false, cpu.registers.zero_flag());
    assert_eq!(true, cpu.registers.sign_flag());
}

#[test]
fn ldx_value_set() {
    let mut cpu = TestCpu::new_test();
    Ldx::execute(&mut cpu, 0xff_u8);
    assert_eq!(0xff, cpu.registers.x);
}

#[test]
fn ldx_flags_sign_and_zero_1() {
    let mut cpu = TestCpu::new_test();
    Ldx::execute(&mut cpu, 0x0_u8);
    assert_eq!(true, cpu.registers.zero_flag());
    assert_eq!(false, cpu.registers.sign_flag());
}

#[test]
fn ldx_flags_sign_and_zero_2() {
    let mut cpu = TestCpu::new_test();
    Ldx::execute(&mut cpu, 0x1_u8);
    assert_eq!(false, cpu.registers.zero_flag());
    assert_eq!(false, cpu.registers.sign_flag());
}

#[test]
fn ldx_flags_sign_and_zero_3() {
    let mut cpu = TestCpu::new_test();
    Ldx::execute(&mut cpu, 0x80_u8);
    assert_eq!(false, cpu.registers.zero_flag());
    assert_eq!(true, cpu.registers.sign_flag());
}

#[test]
fn ldy_value_set() {
    let mut cpu = TestCpu::new_test();
    Ldy::execute(&mut cpu, 0xff_u8);
    assert_eq!(0xff, cpu.registers.y);
}

#[test]
fn ldy_flags_sign_and_zero_1() {
    let mut cpu = TestCpu::new_test();
    Ldy::execute(&mut cpu, 0x0_u8);
    assert_eq!(true, cpu.registers.zero_flag());
    assert_eq!(false, cpu.registers.sign_flag());
}

#[test]
fn ldy_flags_sign_and_zero_2() {
    let mut cpu = TestCpu::new_test();
    Ldy::execute(&mut cpu, 0x1_u8);
    assert_eq!(false, cpu.registers.zero_flag());
    assert_eq!(false, cpu.registers.sign_flag());
}

#[test]
fn ldy_flags_sign_and_zero_3() {
    let mut cpu = TestCpu::new_test();
    Ldy::execute(&mut cpu, 0x80_u8);
    assert_eq!(false, cpu.registers.zero_flag());
    assert_eq!(true, cpu.registers.sign_flag());
}

#[test]
fn sta() {
    let mut cpu = TestCpu::new_test();
    cpu.registers.acc = 0xff;
    let am = WriterAddressingMode::new();
    let write_ref = am.write_ref();
    Sta::execute(&mut cpu, am);
    assert_eq!(0xff, write_ref.get());
}

#[test]
fn stx() {
    let mut cpu = TestCpu::new_test();
    cpu.registers.x = 0xff;
    let am = WriterAddressingMode::new();
    let write_ref = am.write_ref();
    Stx::execute(&mut cpu, am);
    assert_eq!(0xff, write_ref.get());
}

#[test]
fn sty() {
    let mut cpu = TestCpu::new_test();
    cpu.registers.y = 0xff;
    let am = WriterAddressingMode::new();
    let write_ref = am.write_ref();
    Sty::execute(&mut cpu, am);
    assert_eq!(0xff, write_ref.get());
}
