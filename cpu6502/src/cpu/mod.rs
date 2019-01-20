#[cfg(test)]
mod test_fixture;

#[cfg(test)]
mod spec_tests;

#[cfg(test)]
mod functional_tests;

#[cfg(test)]
mod length_and_timing_tests;

mod opcodes;
mod registers;

use crate::{
    byte_utils::{from_lo_hi, lo_hi, wrapping_dec, wrapping_inc},
    cpu::registers::Registers,
};

pub const ADDRESSABLE_MEMORY: usize = 65_536;
const STACK_LOC: u16 = 0x100;
const NMI_VECTOR: u16 = 0xfffa;
const RESET_VECTOR: u16 = 0xfffc;
const BREAK_VECTOR: u16 = 0xfffe;

#[allow(dead_code)]
#[derive(Eq, PartialEq, Copy, Clone)]
pub enum Interrupt {
    None,
    Nmi,
    Irq,
}

pub trait Interconnect {
    fn read(&self, address: u16) -> u8;
    fn write(&mut self, address: u16, value: u8);
    fn tick(&mut self) -> Interrupt;
    fn elapsed_cycles(&self) -> usize;
}

pub struct Cpu<I: Interconnect> {
    registers: Registers,
    pub interconnect: I,
    pending_interrupt: Interrupt,
}

impl<I: Interconnect> Cpu<I> {
    pub fn new(interconnect: I, pc: u16) -> Self {
        let mut cpu = Cpu {
            registers: Registers::new(),
            interconnect,
            pending_interrupt: Interrupt::None,
        };
        cpu.registers.pc = pc;
        cpu
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
        let val = self.interconnect.read(addr);
        self.tick();
        val
    }

    fn read_memory16(&mut self, addr: u16) -> u16 {
        let low_byte = self.read_memory(addr);
        let high_byte = self.read_memory(addr + 1);
        from_lo_hi(low_byte, high_byte)
    }

    fn read_memory16_zp(&mut self, addr: u8) -> u16 {
        let low_byte = self.read_memory(u16::from(addr));
        let high_byte = self.read_memory(u16::from(wrapping_inc(addr)));
        from_lo_hi(low_byte, high_byte)
    }

    fn write_memory(&mut self, addr: u16, val: u8) {
        self.interconnect.write(addr, val);
        self.tick();
    }

    fn tick(&mut self) {
        if self.interconnect.tick() == Interrupt::Nmi {
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
        u16::from(low_byte) | u16::from(high_byte) << 8
    }

    fn push_stack(&mut self, value: u8) {
        let sp = u16::from(self.registers.sp);
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
        let val = self.read_memory(STACK_LOC + u16::from(sp));
        self.registers.sp = sp;
        val
    }

    fn pop_stack16(&mut self) -> u16 {
        let low_byte = self.pop_stack();
        let high_byte = self.pop_stack();
        from_lo_hi(low_byte, high_byte)
    }
}
