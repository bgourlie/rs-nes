mod byte_utils;
mod registers;
mod opcodes;

use memory::*;
pub use self::registers::Registers;
use self::byte_utils::{lo_hi, from_lo_hi, wrapping_inc, wrapping_dec};

pub const STACK_LOC: u16 = 0x100;
pub const NMI_VECTOR: u16 = 0xfffa;
pub const RESET_VECTOR: u16 = 0xfffc;

#[cfg(test)]
pub type TestCpu = Cpu<SimpleMemory>;

#[cfg(test)]
impl TestCpu {
    pub fn new_test() -> Self {
        let memory = SimpleMemory::new();
        Cpu::new(memory)
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
        let opcode = self.read_op(&tick_handler);
        self::opcodes::execute(self, opcode, &tick_handler)
    }

    fn read_memory<F: Fn(&Self)>(&self, addr: u16, tick_handler: F) -> u8 {
        let val = self.memory.load(addr);
        tick_handler(&self);
        val
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
        let (pc_low, pc_high) = lo_hi(self.registers.pc);
        self.push_stack(pc_low, &tick_handler);
        self.push_stack(pc_high, &tick_handler);
        self.push_stack(stat, &tick_handler);
        let pc_low = self.read_memory(NMI_VECTOR, &tick_handler);
        let pc_high = self.read_memory(NMI_VECTOR + 1, &tick_handler);
        self.registers.pc = from_lo_hi(pc_low, pc_high);
    }

    fn read_op<F: Fn(&Self)>(&mut self, tick_handler: F) -> u8 {
        let pc = self.registers.pc;
        let operand = self.read_memory(pc, &tick_handler);
        self.registers.pc += 1;
        operand
    }

    fn push_stack<F: Fn(&Self)>(&mut self, value: u8, tick_handler: F) {
        let sp = self.registers.sp as u16;
        self.write_memory(STACK_LOC + sp, value, &tick_handler);
        self.registers.sp = wrapping_dec(self.registers.sp);
    }

    fn pop_stack<F: Fn(&Self)>(&mut self, tick_handler: F) -> u8 {
        let val = self.read_memory(STACK_LOC + wrapping_inc(self.registers.sp) as u16,
                                   &tick_handler);
        self.registers.sp = wrapping_inc(self.registers.sp);
        val
    }
}
