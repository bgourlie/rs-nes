#[cfg(test)]
mod spec_tests;

#[cfg(all(test, feature = "expensive_tests"))] // Test takes long to run
mod functional_test;

#[cfg(all(test, feature = "expensive_tests"))] // Test takes forever to compile
mod length_and_timing_tests;

pub mod debugger;
mod byte_utils;
mod registers;
mod opcodes;

use cpu::byte_utils::{from_lo_hi, lo_hi, wrapping_dec, wrapping_inc};
use cpu::registers::Registers;
use memory::*;

const STACK_LOC: u16 = 0x100;
const NMI_VECTOR: u16 = 0xfffa;
const RESET_VECTOR: u16 = 0xfffc;

#[cfg(test)]
pub type TestCpu = Cpu<SimpleMemory>;

#[cfg(test)]
impl TestCpu {
    pub fn new_test() -> Self {
        let memory = SimpleMemory::new();
        Cpu::new(memory, 0x200)
    }
}

pub struct Cpu<M: Memory> {
    registers: Registers,
    memory: M,
    cycles: u64,
}

impl<Mem: Memory> Cpu<Mem> {
    pub fn new(memory: Mem, pc: u16) -> Self {
        let mut cpu = Cpu {
            registers: Registers::new(),
            memory: memory,
            cycles: 0,
        };

        cpu.registers.pc = pc;
        cpu
    }

    pub fn step(&mut self) {
        let opcode = self.read_pc();
        self::opcodes::execute(self, opcode)
    }

    fn read_memory(&mut self, addr: u16) -> u8 {
        let val = self.memory.load(addr);
        self.tick();
        val
    }

    fn read_memory16(&mut self, addr: u16) -> u16 {
        let low_byte = self.read_memory(addr);
        let high_byte = self.read_memory(addr + 1);
        from_lo_hi(low_byte, high_byte)
    }

    fn read_memory16_zp(&mut self, addr: u8) -> u16 {
        let low_byte = self.read_memory(addr as u16);
        let high_byte = self.read_memory(wrapping_inc(addr) as u16);
        from_lo_hi(low_byte, high_byte)
    }

    fn write_memory(&mut self, addr: u16, val: u8) {
        self.memory.store(addr, val);
        self.tick()
    }

    pub fn reset(&mut self) {
        let pc_low = self.read_memory(RESET_VECTOR);
        let pc_high = self.read_memory(RESET_VECTOR + 1);
        self.registers.pc = from_lo_hi(pc_low, pc_high);
    }

    pub fn nmi(&mut self) {
        let stat = self.registers.status;
        let pc = self.registers.pc;
        self.push_stack16(pc);
        self.push_stack(stat);
        let pc = self.read_memory16(NMI_VECTOR);
        self.registers.pc = pc;
    }

    fn tick(&mut self) {
        self.cycles += 1;
        if self.memory.tick() == TickAction::Nmi {
            self.nmi()
        }
    }

    fn read_pc(&mut self) -> u8 {
        let pc = self.registers.pc;
        let operand = self.read_memory(pc);
        self.registers.pc += 1;
        operand
    }

    fn read_pc16(&mut self) -> u16 {
        let low_byte = self.read_pc();
        let high_byte = self.read_pc();
        low_byte as u16 | (high_byte as u16) << 8
    }

    fn push_stack(&mut self, value: u8) {
        let sp = self.registers.sp as u16;
        self.write_memory(STACK_LOC + sp, value);
        self.registers.sp = wrapping_dec(self.registers.sp);
    }

    fn push_stack16(&mut self, value: u16) {
        let (low_byte, high_byte) = lo_hi(value);
        self.push_stack(high_byte);
        self.push_stack(low_byte);
    }

    fn pop_stack(&mut self) -> u8 {
        let sp = wrapping_inc(self.registers.sp);
        let val = self.read_memory(STACK_LOC + sp as u16);
        self.registers.sp = sp;
        val
    }

    fn pop_stack16(&mut self) -> u16 {
        let low_byte = self.pop_stack();
        let high_byte = self.pop_stack();
        from_lo_hi(low_byte, high_byte)
    }
}
