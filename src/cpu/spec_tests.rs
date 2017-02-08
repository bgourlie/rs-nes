use cpu::*;
use cpu::byte_utils::lo_hi;
use memory::*;

#[test]
fn reset() {
    let mut cpu = TestCpu::new_test();
    let (addr_low, addr_high) = lo_hi(0xdead);
    cpu.memory.store(RESET_VECTOR, addr_low).unwrap();
    cpu.memory.store(RESET_VECTOR + 1, addr_high).unwrap();
    cpu.reset().unwrap();
    assert_eq!(cpu.registers.pc, 0xdead);
}

#[test]
fn nmi() {
    let mut cpu = TestCpu::new_test();
    let (addr_low, addr_high) = lo_hi(0xdead);
    cpu.memory.store(NMI_VECTOR, addr_low).unwrap();
    cpu.memory.store(NMI_VECTOR + 1, addr_high).unwrap();
    cpu.nmi().unwrap();
    assert_eq!(cpu.registers.pc, 0xdead);
}

#[test]
fn push_stack() {
    let mut cpu = TestCpu::new_test();
    cpu.registers.sp = 0xff;
    let sp = cpu.registers.sp;
    cpu.push_stack(0xde).unwrap();
    let mem = cpu.read_memory(STACK_LOC + sp as u16).unwrap();
    assert_eq!(0xfe, cpu.registers.sp);
    assert_eq!(0xde, mem);
}

#[test]
fn push_stack16() {
    let mut cpu = TestCpu::new_test();
    cpu.registers.sp = 0xff;
    let sp = cpu.registers.sp;
    cpu.push_stack16(0xdead).unwrap();
    let mem = cpu.read_memory16(STACK_LOC + sp as u16 - 1).unwrap();
    assert_eq!(0xfd, cpu.registers.sp);
    assert_eq!(0xdead, mem);
}

#[test]
fn pop_stack() {
    let mut cpu = TestCpu::new_test();
    cpu.registers.sp = 0xfe;
    let sp = cpu.registers.sp;
    cpu.write_memory(STACK_LOC + sp as u16 + 1, 0xf0).unwrap();
    let val = cpu.pop_stack().unwrap();
    assert_eq!(0xff, cpu.registers.sp);
    assert_eq!(0xf0, val);
}

#[test]
fn pop_stack16() {
    let mut cpu = TestCpu::new_test();
    cpu.registers.sp = 0xfd;
    let sp = cpu.registers.sp;
    let (val_low, val_high) = lo_hi(0xf00d);
    cpu.write_memory(STACK_LOC + sp as u16 + 1, val_low).unwrap();
    cpu.write_memory(STACK_LOC + sp as u16 + 2, val_high).unwrap();
    let val = cpu.pop_stack16().unwrap();
    assert_eq!(0xff, cpu.registers.sp);
    assert_eq!(0xf00d, val);
}
