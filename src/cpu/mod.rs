#[cfg(test)]
mod spec_tests;

#[cfg(test)]
mod functional_test;

pub mod debugger;
mod byte_utils;
mod registers;
mod opcodes;

use cpu::byte_utils::{from_lo_hi, lo_hi, wrapping_dec, wrapping_inc};
pub use cpu::registers::Registers;
use memory::*;

pub const STACK_LOC: u16 = 0x100;
pub const NMI_VECTOR: u16 = 0xfffa;
pub const RESET_VECTOR: u16 = 0xfffc;

#[cfg(test)]
pub type TestCpu = Cpu<SimpleMemory>;

#[cfg(test)]
impl TestCpu {
    pub fn new_test() -> Self {
        let memory = SimpleMemory::new();
        let mut cpu = Cpu::new(memory);
        cpu.registers.pc = 0x200;
        cpu
    }
}

pub struct Cpu<M: Memory> {
    pub registers: Registers,
    memory: M,
}

impl<Mem: Memory> Cpu<Mem> {
    pub fn new(memory: Mem) -> Self {
        Cpu {
            registers: Registers::new(),
            memory: memory,
        }
    }

    pub fn step<F: Fn(&Self)>(&mut self, tick_handler: F) {
        let opcode = self.read_pc(&tick_handler);
        self::opcodes::execute(self, opcode, &tick_handler)
    }

    fn read_memory<F: Fn(&Self)>(&self, addr: u16, tick_handler: F) -> u8 {
        let val = self.memory.load(addr);
        tick_handler(&self);
        val
    }

    fn read_memory16<F: Fn(&Self)>(&self, addr: u16, tick_handler: F) -> u16 {
        let low_byte = self.read_memory(addr, &tick_handler);
        let high_byte = self.read_memory(addr + 1, &tick_handler);
        from_lo_hi(low_byte, high_byte)
    }

    fn read_memory16_zp<F: Fn(&Self)>(&self, addr: u8, tick_handler: F) -> u16 {
        let low_byte = self.read_memory(addr as u16, &tick_handler);
        let high_byte = self.read_memory(wrapping_inc(addr) as u16, &tick_handler);
        from_lo_hi(low_byte, high_byte)
    }

    fn write_memory<F: Fn(&Self)>(&mut self, addr: u16, val: u8, tick_handler: F) {
        self.memory.store(addr, val);
        tick_handler(&self);
    }

    pub fn reset<F: Fn(&Self)>(&mut self, tick_handler: F) {
        let pc_low = self.read_memory(RESET_VECTOR, &tick_handler);
        let pc_high = self.read_memory(RESET_VECTOR + 1, &tick_handler);
        self.registers.pc = from_lo_hi(pc_low, pc_high);
    }

    pub fn nmi<F: Fn(&Self)>(&mut self, tick_handler: F) {
        let stat = self.registers.status;
        let pc = self.registers.pc;
        self.push_stack16(pc, &tick_handler);
        self.push_stack(stat, &tick_handler);
        let pc = self.read_memory16(NMI_VECTOR, &tick_handler);
        self.registers.pc = pc;
    }

    fn read_pc<F: Fn(&Self)>(&mut self, tick_handler: F) -> u8 {
        let pc = self.registers.pc;
        let operand = self.read_memory(pc, &tick_handler);
        self.registers.pc += 1;
        operand
    }

    fn read_pc16<F: Fn(&Self)>(&mut self, tick_handler: F) -> u16 {
        let low_byte = self.read_pc(&tick_handler);
        let high_byte = self.read_pc(&tick_handler);
        low_byte as u16 | (high_byte as u16) << 8
    }

    fn push_stack<F: Fn(&Self)>(&mut self, value: u8, tick_handler: F) {
        let sp = self.registers.sp as u16;
        self.write_memory(STACK_LOC + sp, value, &tick_handler);
        self.registers.sp = wrapping_dec(self.registers.sp);
    }

    fn push_stack16<F: Fn(&Self)>(&mut self, value: u16, tick_handler: F) {
        let (low_byte, high_byte) = lo_hi(value);
        self.push_stack(high_byte, &tick_handler);
        self.push_stack(low_byte, &tick_handler);
    }

    fn pop_stack<F: Fn(&Self)>(&mut self, tick_handler: F) -> u8 {
        let sp = wrapping_inc(self.registers.sp);
        let val = self.read_memory(STACK_LOC + sp as u16, &tick_handler);
        self.registers.sp = sp;
        val
    }

    fn pop_stack16<F: Fn(&Self)>(&mut self, tick_handler: F) -> u16 {
        let low_byte = self.pop_stack(&tick_handler);
        let high_byte = self.pop_stack(&tick_handler);
        from_lo_hi(low_byte, high_byte)
    }
}
