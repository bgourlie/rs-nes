
pub mod debugger;
mod registers;
mod execution_context;
mod opcodes;

use std::num::Wrapping;

use self::registers::*;
use self::opcodes::{OpCode, AddressingMode, Instruction};
use self::execution_context::zero_page::ZeroPage;
use self::execution_context::immediate::Immediate;

use constants::*;
use memory::*;

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
    pub cycles: u64,
    pub registers: Registers,
    pub memory: M,
}

impl<Mem: Memory> Cpu<Mem> {
    pub fn new(memory: Mem) -> Self {
        Cpu {
            cycles: 0,
            registers: Registers::new(),
            memory: memory,
        }
    }

    pub fn step<F: Fn(&Self)>(&mut self, tick_handler: F) {
        let opcode_byte = self.read_op();
        tick_handler(self);
        let (opcode, addressing_mode) = self::opcodes::decode(opcode_byte);
        match addressing_mode {
            AddressingMode::ZeroPage => self.step_zeropage(opcode, tick_handler),
            AddressingMode::Immediate => self.step_immediate(opcode, tick_handler),
            AddressingMode::Absolute => unimplemented!(),
            AddressingMode::AbsoluteX => unimplemented!(),
            AddressingMode::AbsoluteY => unimplemented!(),
            AddressingMode::Accumulator => unimplemented!(),
            AddressingMode::Implied => unimplemented!(),
            AddressingMode::IndexedIndirect => unimplemented!(),
            AddressingMode::Indirect => unimplemented!(),
            AddressingMode::IndirectIndexed => unimplemented!(),
            AddressingMode::Relative => unimplemented!(),
            AddressingMode::ZeroPageX => unimplemented!(),
            AddressingMode::ZeroPageY => unimplemented!(),
        };
    }

    fn step_zeropage<F: Fn(&Self)>(&mut self, opcode: OpCode, tick_handler: F) {
        match opcode {
            OpCode::Adc => {
                let adc = self::opcodes::adc::Adc;
                adc.execute(self, ZeroPage, tick_handler)
            }
            _ => panic!("Unexpected zeropage opcode"),
        }
    }

    fn step_immediate<F: Fn(&Self)>(&mut self, opcode: OpCode, tick_handler: F) {
        match opcode {
            OpCode::Adc => {
                let adc = self::opcodes::adc::Adc;
                adc.execute(self, Immediate, tick_handler)
            }
            _ => panic!("unexpected immediate opcode"),
        }
    }

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
