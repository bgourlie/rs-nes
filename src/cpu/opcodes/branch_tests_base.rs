use cpu::*;

pub fn test_branch_not_crossing_page_boundary_positive_offset<F>(setup_and_branch: F)
    where F: Fn(&mut TestCpu, i8)
{
    let mut cpu = TestCpu::new_test();
    cpu.registers.pc = 43656;
    setup_and_branch(&mut cpu, 5);
    assert_eq!(43661, cpu.registers.pc);
}

pub fn test_branch_not_crossing_page_boundary_negative_offset<F>(setup_and_branch: F)
    where F: Fn(&mut TestCpu, i8)
{
    let mut cpu = TestCpu::new_test();
    cpu.registers.pc = 43656;
    setup_and_branch(&mut cpu, -1);
    assert_eq!(43655, cpu.registers.pc);
}

pub fn test_no_branch<F>(setup_and_branch: F)
    where F: Fn(&mut TestCpu, i8)
{
    let mut cpu = TestCpu::new_test();
    cpu.registers.pc = 30;
    setup_and_branch(&mut cpu, -20);
    assert_eq!(30, cpu.registers.pc);
}
