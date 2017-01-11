use cpu::*;

pub fn equal_flag_check_base<F>(setup_and_compare: F)
    where F: Fn(&mut TestCpu, u8, u8)
{
    let mut cpu = TestCpu::new_test();
    setup_and_compare(&mut cpu, 1, 1);
    assert_eq!(true, cpu.registers.zero_flag());
    assert_eq!(true, cpu.registers.carry_flag());
    assert_eq!(false, cpu.registers.sign_flag());
}

pub fn less_than_flag_check_base<F>(setup_and_compare: F)
    where F: Fn(&mut TestCpu, u8, u8)
{
    let mut cpu = TestCpu::new_test();
    setup_and_compare(&mut cpu, 1, 2);
    assert_eq!(false, cpu.registers.zero_flag());
    assert_eq!(false, cpu.registers.carry_flag());
    assert_eq!(true, cpu.registers.sign_flag());
}

pub fn greater_than_flag_check_base<F>(setup_and_compare: F)
    where F: Fn(&mut TestCpu, u8, u8)
{
    let mut cpu = TestCpu::new_test();
    setup_and_compare(&mut cpu, 3, 2);
    assert_eq!(false, cpu.registers.zero_flag());
    assert_eq!(true, cpu.registers.carry_flag());
    assert_eq!(false, cpu.registers.sign_flag());
}
