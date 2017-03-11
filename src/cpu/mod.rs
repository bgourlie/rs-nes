#[cfg(test)]
mod spec_tests;

#[cfg(all(test, feature = "slow_tests"))]
mod functional_tests;

#[cfg(all(test, feature = "slow_tests"))]
mod length_and_timing_tests;

#[cfg(feature = "debugger")]
pub mod debugger;

mod registers;
mod opcodes;

use byte_utils::{from_lo_hi, lo_hi, wrapping_dec, wrapping_inc};
use cpu::registers::Registers;
use errors::*;
use memory::*;

const STACK_LOC: u16 = 0x100;
const NMI_VECTOR: u16 = 0xfffa;
const RESET_VECTOR: u16 = 0xfffc;
const BREAK_VECTOR: u16 = 0xfffe;

#[cfg(test)]
pub type TestMemory = SimpleMemory;

#[cfg(test)]
pub type TestCpu = Cpu<TestMemory>;

#[cfg(test)]
impl TestCpu {
    pub fn new_test() -> Self {
        let memory = SimpleMemory::default();
        Cpu::new_init_pc(memory, 0x200)
    }
}


#[derive(PartialEq, Eq)]
pub enum TickAction {
    None,
    Nmi,
}

#[allow(dead_code)]
enum Interrupt {
    None,
    Nmi,
    Irq,
}

pub struct Cpu<M: Memory> {
    registers: Registers,
    memory: M,
    pending_interrupt: Interrupt,
    cycles: u64,
}

impl<Mem: Memory> Cpu<Mem> {
    pub fn new_init_pc(memory: Mem, pc: u16) -> Self {
        let mut cpu = Self::new(memory);
        cpu.registers.pc = pc;
        cpu
    }

    pub fn new(memory: Mem) -> Self {
        Cpu {
            registers: Registers::new(),
            memory: memory,
            cycles: 0,
            pending_interrupt: Interrupt::None,
        }
    }

    pub fn step(&mut self) -> Result<()> {
        let opcode = self.read_pc()?;
        self::opcodes::execute(self, opcode)?;

        match self.pending_interrupt {
            Interrupt::None => Ok(()),
            Interrupt::Nmi => {
                self.pending_interrupt = Interrupt::None;
                self.nmi()
            }
            Interrupt::Irq => {
                self.pending_interrupt = Interrupt::None;
                self.irq()
            }
        }
    }

    pub fn reset(&mut self) -> Result<()> {
        let pc_low = self.read_memory(RESET_VECTOR)?;
        let pc_high = self.read_memory(RESET_VECTOR + 1)?;
        self.registers.pc = from_lo_hi(pc_low, pc_high);
        Ok(())
    }

    fn nmi(&mut self) -> Result<()> {
        self.push_pc_and_status()?;
        let pc = self.read_memory16(NMI_VECTOR)?;
        self.registers.set_interrupt_disable_flag(true);
        self.registers.pc = pc;
        Ok(())
    }

    fn irq(&mut self) -> Result<()> {
        if self.registers.interrupt_disable_flag() {
            Ok(())
        } else {
            self.push_pc_and_status()?;
            let pc = self.read_memory16(BREAK_VECTOR)?;
            self.registers.set_interrupt_disable_flag(true);
            self.registers.pc = pc;
            Ok(())
        }
    }

    fn push_pc_and_status(&mut self) -> Result<()> {
        let stat = self.registers.status_sans_break();
        let pc = self.registers.pc;
        self.push_stack16(pc)?;
        self.push_stack(stat)
    }

    fn read_memory(&mut self, addr: u16) -> Result<u8> {
        let val = self.memory.read(addr)?;
        self.tick()?;
        Ok(val)
    }

    fn read_memory16(&mut self, addr: u16) -> Result<u16> {
        let low_byte = self.read_memory(addr)?;
        let high_byte = self.read_memory(addr + 1)?;
        Ok(from_lo_hi(low_byte, high_byte))
    }

    fn read_memory16_zp(&mut self, addr: u8) -> Result<u16> {
        let low_byte = self.read_memory(addr as u16)?;
        let high_byte = self.read_memory(wrapping_inc(addr) as u16)?;
        Ok(from_lo_hi(low_byte, high_byte))
    }

    fn write_memory(&mut self, addr: u16, val: u8) -> Result<()> {
        // If OAM dma occurs, additional cycles will elapse
        self.cycles += self.memory.write(addr, val, self.cycles)?;
        self.tick()
    }

    fn tick(&mut self) -> Result<()> {
        self.cycles += 1;
        if self.memory.tick()? == TickAction::Nmi {
            self.pending_interrupt = Interrupt::Nmi;
        }
        Ok(())
    }

    fn read_pc(&mut self) -> Result<u8> {
        let pc = self.registers.pc;
        let operand = self.read_memory(pc)?;
        self.registers.pc += 1;
        Ok(operand)
    }

    fn read_pc16(&mut self) -> Result<u16> {
        let low_byte = self.read_pc()?;
        let high_byte = self.read_pc()?;
        Ok(low_byte as u16 | (high_byte as u16) << 8)
    }

    fn push_stack(&mut self, value: u8) -> Result<()> {
        let sp = self.registers.sp as u16;
        self.write_memory(STACK_LOC + sp, value)?;
        self.registers.sp = wrapping_dec(self.registers.sp);
        Ok(())
    }

    fn push_stack16(&mut self, value: u16) -> Result<()> {
        let (low_byte, high_byte) = lo_hi(value);
        self.push_stack(high_byte)?;
        self.push_stack(low_byte)?;
        Ok(())
    }

    fn pop_stack(&mut self) -> Result<u8> {
        let sp = wrapping_inc(self.registers.sp);
        let val = self.read_memory(STACK_LOC + sp as u16)?;
        self.registers.sp = sp;
        Ok(val)
    }

    fn pop_stack16(&mut self) -> Result<u16> {
        let low_byte = self.pop_stack()?;
        let high_byte = self.pop_stack()?;
        Ok(from_lo_hi(low_byte, high_byte))
    }
}
