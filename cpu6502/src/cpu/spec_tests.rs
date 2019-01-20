use crate::{
    byte_utils::lo_hi,
    cpu::{test_fixture::TestCpu, *},
};

#[test]
fn reset() {
    let mut cpu = TestCpu::new_test();
    let (addr_low, addr_high) = lo_hi(0xdead);
    cpu.interconnect.write(RESET_VECTOR, addr_low);
    cpu.interconnect.write(RESET_VECTOR + 1, addr_high);
    cpu.reset();
    assert_eq!(cpu.registers.pc, 0xdead);
}

#[test]
fn nmi() {
    let mut cpu = TestCpu::new_test();
    let (addr_low, addr_high) = lo_hi(0xdead);
    cpu.interconnect.write(NMI_VECTOR, addr_low);
    cpu.interconnect.write(NMI_VECTOR + 1, addr_high);
    cpu.nmi();
    assert_eq!(cpu.registers.pc, 0xdead);
}

#[test]
fn push_stack() {
    let mut cpu = TestCpu::new_test();
    cpu.registers.sp = 0xff;
    let sp = cpu.registers.sp;
    cpu.push_stack(0xde);
    let mem = cpu.read_memory(STACK_LOC + sp as u16);
    assert_eq!(0xfe, cpu.registers.sp);
    assert_eq!(0xde, mem);
}

#[test]
fn push_stack16() {
    let mut cpu = TestCpu::new_test();
    cpu.registers.sp = 0xff;
    let sp = cpu.registers.sp;
    cpu.push_stack16(0xdead);
    let mem = cpu.read_memory16(STACK_LOC + sp as u16 - 1);
    assert_eq!(0xfd, cpu.registers.sp);
    assert_eq!(0xdead, mem);
}

#[test]
fn pop_stack() {
    let mut cpu = TestCpu::new_test();
    cpu.registers.sp = 0xfe;
    let sp = cpu.registers.sp;
    cpu.write_memory(STACK_LOC + sp as u16 + 1, 0xf0);
    let val = cpu.pop_stack();
    assert_eq!(0xff, cpu.registers.sp);
    assert_eq!(0xf0, val);
}

#[test]
fn pop_stack16() {
    let mut cpu = TestCpu::new_test();
    cpu.registers.sp = 0xfd;
    let sp = cpu.registers.sp;
    let (val_low, val_high) = lo_hi(0xf00d);
    cpu.write_memory(STACK_LOC + sp as u16 + 1, val_low);
    cpu.write_memory(STACK_LOC + sp as u16 + 2, val_high);
    let val = cpu.pop_stack16();
    assert_eq!(0xff, cpu.registers.sp);
    assert_eq!(0xf00d, val);
}
