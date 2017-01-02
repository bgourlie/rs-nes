use cpu::*;
use cpu::opcodes::addressing_mode::test_addressing_modes::*;

pub fn inc_base_1<F>(inc: F)
    where F: Fn(&mut TestCpu, WriterAddressingMode)
{
    let mut cpu = TestCpu::new_test();
    let test_am = WriterAddressingMode::new(1);
    let value_ref = test_am.value_ref();
    inc(&mut cpu, test_am);
    let written = value_ref.get();
    assert_eq!(2, written);
    assert_eq!(false, cpu.registers.zero_flag());
    assert_eq!(false, cpu.registers.sign_flag());
}

pub fn inc_base_2<F>(inc: F)
    where F: Fn(&mut TestCpu, WriterAddressingMode)
{
    let mut cpu = TestCpu::new_test();
    let test_am = WriterAddressingMode::new(255);
    let value_ref = test_am.value_ref();
    inc(&mut cpu, test_am);
    let written = value_ref.get();
    assert_eq!(0, written);
    assert_eq!(true, cpu.registers.zero_flag());
    assert_eq!(false, cpu.registers.sign_flag());
}

pub fn inc_base_3<F>(inc: F)
    where F: Fn(&mut TestCpu, WriterAddressingMode)
{
    let mut cpu = TestCpu::new_test();
    let test_am = WriterAddressingMode::new(254);
    let value_ref = test_am.value_ref();
    inc(&mut cpu, test_am);
    let written = value_ref.get();
    assert_eq!(-1, written as i8);
    assert_eq!(false, cpu.registers.zero_flag());
    assert_eq!(true, cpu.registers.sign_flag());
}

pub fn dec_base_1<F>(dec: F)
    where F: Fn(&mut TestCpu, WriterAddressingMode)
{
    let mut cpu = TestCpu::new_test();
    let test_am = WriterAddressingMode::new(1);
    let value_ref = test_am.value_ref();
    dec(&mut cpu, test_am);
    let written = value_ref.get();
    assert_eq!(0, written);
    assert_eq!(true, cpu.registers.zero_flag());
    assert_eq!(false, cpu.registers.sign_flag());
}

pub fn dec_base_2<F>(dec: F)
    where F: Fn(&mut TestCpu, WriterAddressingMode)
{
    let mut cpu = TestCpu::new_test();
    let test_am = WriterAddressingMode::new(2);
    let value_ref = test_am.value_ref();
    dec(&mut cpu, test_am);
    let written = value_ref.get();
    assert_eq!(1, written);
    assert_eq!(false, cpu.registers.zero_flag());
    assert_eq!(false, cpu.registers.sign_flag());
}

pub fn dec_base_3<F>(dec: F)
    where F: Fn(&mut TestCpu, WriterAddressingMode)
{
    let mut cpu = TestCpu::new_test();
    let test_am = WriterAddressingMode::new(254);
    let value_ref = test_am.value_ref();
    dec(&mut cpu, test_am);
    let written = value_ref.get();
    assert_eq!(-3, written as i8);
    assert_eq!(false, cpu.registers.zero_flag());
    assert_eq!(true, cpu.registers.sign_flag());
}
