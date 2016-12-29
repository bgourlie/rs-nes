
pub mod debugger;
mod registers;
mod addressing;
mod opcodes;

use std::num::Wrapping;

use self::debugger::*;
use self::registers::*;
use self::opcodes::Instruction;

use constants::*;
use memory::*;

#[cfg(test)]
pub type TestCpu = Cpu<SimpleMemory, NoOpDebugger<SimpleMemory>>;

#[cfg(test)]
impl TestCpu {
    pub fn new_test() -> Self {
        let memory = SimpleMemory::new();
        let debugger = NoOpDebugger::new();
        Cpu::new(memory, debugger)
    }
}

pub struct Cpu<M: Memory, D: Debugger<M>> {
    pub cycles: u64,
    pub registers: Registers,
    pub memory: M,
    pub debugger: D,
}

enum CpuState<M, D, I>
    where M: Memory,
          D: Debugger<M>,
          I: Instruction<M, D>
{
    Decoding(M, D),
    Executing(M, D, I),
}

impl<Mem: Memory, D: Debugger<Mem>> Cpu<Mem, D> {
    pub fn new(memory: Mem, debugger: D) -> Self {
        Cpu {
            cycles: 0,
            registers: Registers::new(),
            memory: memory,
            debugger: debugger,
        }
    }

    pub fn tick(&mut self) {}

    pub fn reset(&mut self) {
        let pc_start = self.memory.load16(RESET_VECTOR);
        self.registers.pc = pc_start;
    }

    pub fn nmi(&mut self) {
        let (pc, stat) = (self.registers.pc, self.registers.status);
        self.push_stack16(pc);
        self.push_stack(stat);
        self.registers.pc = self.memory.load16(NMI_VECTOR);
    }


    fn read_op(&mut self) -> u8 {
        let pc = self.registers.pc;
        let operand = self.memory.load(pc);
        self.registers.pc += 1;
        operand
    }

    fn read_op16(&mut self) -> u16 {
        let pc = self.registers.pc;
        let operand = self.memory.load16(pc);
        self.registers.pc += 2;
        operand
    }

    fn push_stack(&mut self, value: u8) {
        self.memory.store(STACK_LOC + self.registers.sp as u16, value);
        self.registers.sp = (Wrapping(self.registers.sp) - Wrapping(1)).0;
    }

    fn peek_stack(&mut self) -> u8 {
        self.memory.load(STACK_LOC + ((Wrapping(self.registers.sp) + Wrapping(1)).0) as u16)
    }

    fn pop_stack(&mut self) -> u8 {
        let val = self.peek_stack();
        self.registers.sp = (Wrapping(self.registers.sp) + Wrapping(1)).0;
        val
    }

    fn push_stack16(&mut self, value: u16) {
        let stack_loc = Wrapping(STACK_LOC);
        let sp = Wrapping(self.registers.sp);
        let one = Wrapping(1);
        let two = Wrapping(2);
        self.memory.store16((stack_loc + Wrapping((sp - one).0 as u16)).0, value);
        self.registers.sp = (sp - two).0;
    }

    fn peek_stack16(&mut self) -> u16 {
        let lowb = self.memory
            .load(STACK_LOC +
                  ((Wrapping(self.registers.sp) +
                    Wrapping(1_u8))
                .0) as u16) as u16;
        let highb = self.memory
            .load(STACK_LOC +
                  ((Wrapping(self.registers.sp) +
                    Wrapping(2_u8))
                .0) as u16) as u16;
        lowb | (highb << 8)
    }

    fn pop_stack16(&mut self) -> u16 {
        let val = self.peek_stack16();
        self.registers.sp = (Wrapping(self.registers.sp) + Wrapping(2_u8)).0;
        val
    }
}
