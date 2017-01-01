mod byte_utils;
mod registers;
mod opcodes;

use std::num::Wrapping;
use memory::*;
pub use self::registers::Registers;
use self::byte_utils::{lo_hi, from_lo_hi};

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
    pub memory: M,
}

impl<Mem: Memory> Cpu<Mem> {
    pub fn new(memory: Mem) -> Self {
        Cpu {
            registers: Registers::new(),
            memory: memory,
        }
    }

    pub fn step<F: Fn(&Self)>(&mut self, tick_handler: F) {
        let opcode = self.read_op();
        tick_handler(self);
        self::opcodes::execute(self, opcode, &tick_handler)
    }

    pub fn reset(&mut self) {
        let pc_start_low = self.memory.load(RESET_VECTOR);
        let pc_start_high = self.memory.load(RESET_VECTOR + 1);
        self.registers.pc = from_lo_hi(pc_start_low, pc_start_high);
    }

    pub fn nmi(&mut self) {
        let stat = self.registers.status;
        let (pc_low_byte, pc_high_byte) = lo_hi(self.registers.pc);
        self.push_stack(pc_low_byte);
        self.push_stack(pc_high_byte);
        self.push_stack(stat);
        let pc_low_byte = self.memory.load(NMI_VECTOR);
        let pc_high_byte = self.memory.load(NMI_VECTOR + 1);
        self.registers.pc = from_lo_hi(pc_low_byte, pc_high_byte);
    }

    fn read_op(&mut self) -> u8 {
        let pc = self.registers.pc;
        let operand = self.memory.load(pc);
        self.registers.pc += 1;
        operand
    }

    fn push_stack(&mut self, value: u8) {
        self.memory.store(STACK_LOC + self.registers.sp as u16, value);
        self.registers.sp = (Wrapping(self.registers.sp) - Wrapping(1)).0;
    }

    fn pop_stack(&mut self) -> u8 {
        let val = self.memory
            .load(STACK_LOC + ((Wrapping(self.registers.sp) + Wrapping(1)).0) as u16);
        self.registers.sp = (Wrapping(self.registers.sp) + Wrapping(1)).0;
        val
    }
}
