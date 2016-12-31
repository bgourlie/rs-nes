use cpu::*;

pub fn branch_not_crossing_page_boundary_positive_rel_addr<F>(setup_and_branch: F)
    where F: Fn(&mut TestCpu, i8)
{
    let mut cpu = TestCpu::new_test();

    const PC_START: u16 = 43656;
    const REL_ADDR: i8 = 5;

    cpu.registers.pc = 43656;
    setup_and_branch(&mut cpu, 5);

    // assert_eq!(1, cycles);
    assert_eq!(43661, cpu.registers.pc);
}

pub fn branch_not_crossing_page_boundary_negative_rel_addr<F>(setup_and_branch: F)
    where F: Fn(&mut TestCpu, i8)
{
    let mut cpu = TestCpu::new_test();
    cpu.registers.pc = 43656;
    setup_and_branch(&mut cpu, -1);
    // assert_eq!(1, cycles);
    assert_eq!(43655, cpu.registers.pc);
}

pub fn branch_crossing_page_boundary_positive_rel_addr<F>(setup_and_branch: F)
    where F: Fn(&mut TestCpu, i8)
{
    let mut cpu = TestCpu::new_test();
    cpu.registers.pc = 43656;
    setup_and_branch(&mut cpu, 127);
    // assert_eq!(2, cycles);
    assert_eq!(43783, cpu.registers.pc);
}

pub fn branch_crossing_page_boundary_negative_rel_addr<F>(setup_and_branch: F)
    where F: Fn(&mut TestCpu, i8)
{
    let mut cpu = TestCpu::new_test();
    cpu.registers.pc = 43520;
    setup_and_branch(&mut cpu, -128);
    assert_eq!(43392, cpu.registers.pc);
    // assert_eq!(2, cycles);
}

pub fn no_branch<F>(setup_and_branch: F)
    where F: Fn(&mut TestCpu, i8)
{
    let mut cpu = TestCpu::new_test();
    cpu.registers.pc = 30;
    setup_and_branch(&mut cpu, -20);

    // don't adjust pc when carry is set
    assert_eq!(30, cpu.registers.pc);

    //    // no additional cycle when not branching
    //    assert_eq!(0, cycles);
}
