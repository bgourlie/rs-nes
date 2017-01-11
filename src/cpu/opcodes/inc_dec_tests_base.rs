use cpu::*;

pub fn inc_base_1<F>(inc: F)
    where F: Fn(&mut TestCpu, u8) -> u8
{
    let mut cpu = TestCpu::new_test();
    let val = inc(&mut cpu, 1);
    assert_eq!(2, val);
    assert_eq!(false, cpu.registers.zero_flag());
    assert_eq!(false, cpu.registers.sign_flag());
}

pub fn inc_base_2<F>(inc: F)
    where F: Fn(&mut TestCpu, u8) -> u8
{
    let mut cpu = TestCpu::new_test();
    let val = inc(&mut cpu, 255);
    assert_eq!(0, val);
    assert_eq!(true, cpu.registers.zero_flag());
    assert_eq!(false, cpu.registers.sign_flag());
}

pub fn inc_base_3<F>(inc: F)
    where F: Fn(&mut TestCpu, u8) -> u8
{
    let mut cpu = TestCpu::new_test();
    let val = inc(&mut cpu, 254);
    assert_eq!(-1, val as i8);
    assert_eq!(false, cpu.registers.zero_flag());
    assert_eq!(true, cpu.registers.sign_flag());
}

pub fn dec_base_1<F>(dec: F)
    where F: Fn(&mut TestCpu, u8) -> u8
{
    let mut cpu = TestCpu::new_test();
    let val = dec(&mut cpu, 1);
    assert_eq!(0, val);
    assert_eq!(true, cpu.registers.zero_flag());
    assert_eq!(false, cpu.registers.sign_flag());
}

pub fn dec_base_2<F>(dec: F)
    where F: Fn(&mut TestCpu, u8) -> u8
{
    let mut cpu = TestCpu::new_test();
    let val = dec(&mut cpu, 2);
    assert_eq!(1, val);
    assert_eq!(false, cpu.registers.zero_flag());
    assert_eq!(false, cpu.registers.sign_flag());
}

pub fn dec_base_3<F>(dec: F)
    where F: Fn(&mut TestCpu, u8) -> u8
{
    let mut cpu = TestCpu::new_test();
    let val = dec(&mut cpu, 254);
    assert_eq!(-3, val as i8);
    assert_eq!(false, cpu.registers.zero_flag());
    assert_eq!(true, cpu.registers.sign_flag());
}
