use cpu::*;

pub fn shift_left_base_1<F>(do_shift: F)
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

pub fn shift_left_base_2<F>(do_shift: F)
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

pub fn shift_left_base_3<F>(do_shift: F)
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
