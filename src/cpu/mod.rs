#[cfg(test)]
mod spec_tests;

#[cfg(test)]
mod functional_tests;

#[cfg(test)]
mod length_and_timing_tests;

#[cfg(feature = "debugger")]
pub mod debugger;

mod opcodes;
mod registers;

use byte_utils::{from_lo_hi, lo_hi, wrapping_dec, wrapping_inc};
use cpu::registers::Registers;
use input::*;
use memory::*;
use screen::*;
use std::marker::PhantomData;

const STACK_LOC: u16 = 0x100;
const NMI_VECTOR: u16 = 0xfffa;
const RESET_VECTOR: u16 = 0xfffc;
const BREAK_VECTOR: u16 = 0xfffe;

#[cfg(test)]
pub type TestMemory = SimpleMemory;

#[cfg(test)]
pub type TestCpu = Cpu<NoScreen, NoInput, TestMemory>;

#[cfg(test)]
impl TestCpu {
    pub fn new_test() -> Self {
        let memory = SimpleMemory::default();
        Cpu::new_init_pc(memory, 0x200)
    }
}

#[allow(dead_code)]
#[derive(Eq, PartialEq, Copy, Clone)]
pub enum Interrupt {
    None,
    Nmi,
    Irq,
}

pub struct Cpu<S: Screen, I: Input, M: Memory<I, S>> {
    registers: Registers,
    pub memory: M,
    pending_interrupt: Interrupt,
    pub cycles: u64,
    phantom_s: PhantomData<S>,
    phantom_i: PhantomData<I>,
}

impl<S: Screen, I: Input, M: Memory<I, S>> Cpu<S, I, M> {
    pub fn new_init_pc(memory: M, pc: u16) -> Self {
        let mut cpu = Self::new(memory);
        cpu.registers.pc = pc;
        cpu
    }

    pub fn new(memory: M) -> Self {
        Cpu {
            registers: Registers::new(),
            memory: memory,
            cycles: 0,
            pending_interrupt: Interrupt::None,
            phantom_s: PhantomData,
            phantom_i: PhantomData,
        }
    }

    pub fn step(&mut self) -> Interrupt {
        let opcode = self.read_pc();
        self::opcodes::execute(self, opcode);

        let pending_interrupt = self.pending_interrupt;
        match pending_interrupt {
            Interrupt::None => (),
            Interrupt::Nmi => {
                self.pending_interrupt = Interrupt::None;
                self.nmi();
            }
            Interrupt::Irq => {
                self.pending_interrupt = Interrupt::None;
                self.irq();
            }
        }
        pending_interrupt
    }

    pub fn reset(&mut self) {
        let pc_low = self.read_memory(RESET_VECTOR);
        let pc_high = self.read_memory(RESET_VECTOR + 1);
        self.registers.pc = from_lo_hi(pc_low, pc_high);
    }

    fn nmi(&mut self) {
        self.push_pc_and_status();
        let pc = self.read_memory16(NMI_VECTOR);
        self.registers.set_interrupt_disable_flag(true);
        self.registers.pc = pc;
    }

    fn irq(&mut self) {
        if !self.registers.interrupt_disable_flag() {
            self.push_pc_and_status();
            let pc = self.read_memory16(BREAK_VECTOR);
            self.registers.set_interrupt_disable_flag(true);
            self.registers.pc = pc;
        }
    }

    fn push_pc_and_status(&mut self) {
        let stat = self.registers.status_sans_break();
        let pc = self.registers.pc;
        self.push_stack16(pc);
        self.push_stack(stat)
    }

    fn read_memory(&mut self, addr: u16) -> u8 {
        let val = self.memory.read(addr);
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
        // If OAM dma occurs, additional cycles will elapse
        self.cycles += self.memory.write(addr, val, self.cycles);
        self.tick()
    }

    fn tick(&mut self) {
        self.cycles += 1;
        if self.memory.tick() == Interrupt::Nmi {
            self.pending_interrupt = Interrupt::Nmi;
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
        ()
    }

    fn push_stack16(&mut self, value: u16) {
        let (low_byte, high_byte) = lo_hi(value);
        self.push_stack(high_byte);
        self.push_stack(low_byte);
        ()
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
