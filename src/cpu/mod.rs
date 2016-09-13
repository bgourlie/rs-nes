#[cfg(test)]
mod adc_spec_tests;

#[cfg(test)]
mod branching_spec_tests;

#[cfg(test)]
mod cmp_spec_tests;

#[cfg(test)]
mod inc_and_dec_spec_tests;

#[cfg(test)]
mod jumps_and_calls_spec_tests;

#[cfg(test)]
mod lda_spec_tests;

#[cfg(test)]
mod sbc_spec_tests;

#[cfg(test)]
mod shifts_spec_tests;

#[cfg(test)]
mod stack_utils_spec_tests;

#[cfg(test)]
mod status_flag_spec_tests;

#[cfg(test)]
mod store_spec_tests;

#[cfg(test)]
mod interrupt_spec_tests;

#[cfg(test)]
mod functional_tests;

mod registers;
mod debugger;

use std::num::Wrapping;

use constants::*;
use cpu::debugger::*;
use cpu::registers::*;
use memory::*;

// Graciously taken from FCEU
#[cfg_attr(rustfmt, rustfmt_skip)]
const CYCLE_TABLE: [u8; 256] = [
    7,6,2,8,3,3,5,5,3,2,2,2,4,4,6,6, // 0x00
    2,5,2,8,4,4,6,6,2,4,2,7,4,4,7,7, // 0x10
    6,6,2,8,3,3,5,5,4,2,2,2,4,4,6,6, // 0x20
    2,5,2,8,4,4,6,6,2,4,2,7,4,4,7,7, // 0x30
    6,6,2,8,3,3,5,5,3,2,2,2,3,4,6,6, // 0x40
    2,5,2,8,4,4,6,6,2,4,2,7,4,4,7,7, // 0x50
    6,6,2,8,3,3,5,5,4,2,2,2,5,4,6,6, // 0x60
    2,5,2,8,4,4,6,6,2,4,2,7,4,4,7,7, // 0x70
    2,6,2,6,3,3,3,3,2,2,2,2,4,4,4,4, // 0x80
    2,6,2,6,4,4,4,4,2,5,2,5,5,5,5,5, // 0x90
    2,6,2,6,3,3,3,3,2,2,2,2,4,4,4,4, // 0xA0
    2,5,2,5,4,4,4,4,2,4,2,4,4,4,4,4, // 0xB0
    2,6,2,8,3,3,5,5,2,2,2,2,4,4,6,6, // 0xC0
    2,5,2,8,4,4,6,6,2,4,2,7,4,4,7,7, // 0xD0
    2,6,3,8,3,3,5,5,2,2,2,2,4,4,6,6, // 0xE0
    2,5,2,8,4,4,6,6,2,4,2,7,4,4,7,7, // 0xF0
];

#[cfg_attr(rustfmt, rustfmt_skip)]
const LEN_TABLE: [u8; 256] = [
    1,2,0,0,0,2,2,0,1,2,1,0,0,3,3,0, // 0x00
    2,2,0,0,0,2,2,0,1,3,0,0,0,3,3,0, // 0x10
    3,2,0,0,2,2,2,0,1,2,1,0,3,3,3,0, // 0x20
    2,2,0,0,0,2,2,0,1,3,0,0,0,3,3,0, // 0x30
    1,2,0,0,0,2,2,0,1,2,1,0,3,3,3,0, // 0x40
    2,2,0,0,0,2,2,0,1,3,0,0,0,3,3,0, // 0x50
    1,2,0,0,0,2,2,0,1,2,1,0,3,3,3,0, // 0x60
    2,2,0,0,0,2,2,0,1,3,0,0,0,3,3,0, // 0x70
    0,2,0,0,2,2,2,0,1,0,1,0,3,3,3,0, // 0x80
    2,2,0,0,2,2,2,0,1,3,1,0,0,3,0,0, // 0x90
    2,2,2,0,2,2,2,0,1,2,1,0,3,3,3,0, // 0xA0
    2,2,0,0,2,2,2,0,1,3,1,0,3,3,3,0, // 0xB0
    2,2,0,0,2,2,2,0,1,2,1,0,3,3,3,0, // 0xC0
    2,2,0,0,0,2,2,0,1,3,0,0,0,3,3,0, // 0xD0
    2,2,0,0,2,2,2,0,1,2,1,0,3,3,3,0, // 0xE0
    2,2,0,0,0,2,2,0,1,3,0,0,0,3,3,0, // 0xF0
];

// TODO: consolidate logic with similar implementation in Register
fn page_crossed(val1: u16, val2: u16) -> bool {
    val1 & 0xFF00 != val2 & 0xFF00
}

pub struct Cpu6502<T: Memory> {
    pub cycles: u64,
    pub registers: Registers,
    pub memory: T,
}

impl<T: Memory> Cpu6502<T> {
    pub fn new(memory: T) -> Self {
        Cpu6502 {
            cycles: 0,
            registers: Registers::new(),
            memory: memory,
        }
    }

    pub fn reset(&mut self) {
        let pc_start = self.memory.load16(RESET_VECTOR);
        self.registers.pc = pc_start;
    }

    pub fn nmi(&mut self) {
        let (pc, stat) = (self.registers.pc, self.registers.stat);
        self.push_stack16(pc);
        self.push_stack(stat);
        self.registers.pc = self.memory.load16(NMI_VECTOR);
    }

    pub fn step(&mut self) -> Result<Instruction, &'static str> {
        let op_code = self.read_op();
        let instr_len = LEN_TABLE[op_code as usize];
        let mut cycles = CYCLE_TABLE[op_code as usize];
        let instr: Instruction;

        match instr_len {
            0 => {
                return Err("Unexpected opcode encountered");
            }
            1 => {
                instr = Instruction::new1(op_code);
                self.do_op1(op_code);
            }
            2 => {
                let operand = self.read_op();
                instr = Instruction::new2(op_code, operand);
                cycles += self.do_op2(op_code, operand);
            }
            3 => {
                let operand = self.read_op16();
                instr = Instruction::new3(op_code, operand);
                cycles += self.do_op3(op_code, operand);
            }
            _ => {
                panic!("Shouldn't get here");
            }
        }

        self.cycles += cycles as u64;
        Ok(instr)
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

    fn abs_indexed_base(&self, base_addr: u16, index: u8) -> (u8, u16, bool) {
        let target_addr = base_addr + index as u16;
        let page_crossed = page_crossed(base_addr, target_addr);
        (self.memory.load(target_addr), target_addr, page_crossed)
    }

    fn get_absx(&self, val: u16) -> (u8, u16, bool) {
        let x = self.registers.irx;
        self.abs_indexed_base(val, x)
    }

    fn get_absy(&self, val: u16) -> (u8, u16, bool) {
        let y = self.registers.iry;
        self.abs_indexed_base(val, y)
    }

    fn get_indx(&self, base_addr: u8) -> u8 {
        let target_addr = self.indexed_indirect_addr(base_addr);
        self.memory.load(target_addr)
    }

    fn get_indy(&self, base_addr: u8) -> (u8, bool) {
        let target_addr = self.indirect_indexed_addr(base_addr);
        let page_crossed = page_crossed(base_addr as u16, target_addr);
        (self.memory.load(target_addr), page_crossed)
    }

    fn indexed_indirect_addr(&self, base_addr: u8) -> u16 {
        let ind_addr = (Wrapping(base_addr) + Wrapping(self.registers.irx)).0 as u16;
        self.memory.load16(ind_addr)
    }

    fn indirect_indexed_addr(&self, base_addr: u8) -> u16 {
        self.memory.load16(base_addr as u16) + self.registers.iry as u16
    }

    fn zpx_addr(&self, base_addr: u8) -> u16 {
        (Wrapping(base_addr) + Wrapping(self.registers.irx)).0 as u16
    }

    fn zpy_addr(&self, base_addr: u8) -> u16 {
        (Wrapping(base_addr) + Wrapping(self.registers.iry)).0 as u16
    }

    fn get_zpx(&self, base_addr: u8) -> u8 {
        let target_addr = self.zpx_addr(base_addr);
        self.memory.load(target_addr)
    }

    fn get_zpy(&self, base_addr: u8) -> u8 {
        let target_addr = self.zpy_addr(base_addr);
        self.memory.load(target_addr)
    }

    fn do_op2(&mut self, opcode: u8, operand: u8) -> u8 {
        let mut cycles = 0;
        match opcode {
            0xa1 => {
                let val = self.get_indx(operand);
                self.lda(val);
            }
            0xa5 => {
                let addr = operand as u16;
                let val = self.memory.load(addr);
                self.lda(val);
            }
            0xa9 => {
                let val = operand;
                self.lda(val);
            }
            0xb1 => {
                let (val, page_crossed) = self.get_indy(operand);
                self.lda(val);
                if page_crossed {
                    cycles += 1;
                }
            }
            0xb5 => {
                let val = self.get_zpx(operand);
                self.lda(val);
            }
            0xa2 => {
                let val = operand;
                self.ldx(val);
            }
            0xa6 => {
                let addr = operand as u16;
                let val = self.memory.load(addr);
                self.ldx(val);
            }
            0xb6 => {
                let val = self.get_zpy(operand);
                self.ldx(val);
            }
            0xa0 => {
                let val = operand;
                self.ldy(val);
            }
            0xa4 => {
                let addr = operand as u16;
                let val = self.memory.load(addr);
                self.ldy(val);
            }
            0xb4 => {
                let addr = self.zpx_addr(operand);
                let val = self.memory.load(addr);
                self.ldy(val);
            }
            0x85 => {
                let addr = operand as u16;
                self.sta(addr);
            }
            0x95 => {
                let addr = self.zpx_addr(operand);;
                self.sta(addr);
            }
            0x81 => {
                let target_addr = self.indexed_indirect_addr(operand);
                self.sta(target_addr);
            }
            0x91 => {
                let target_addr = self.indirect_indexed_addr(operand);
                self.sta(target_addr);
            }
            0x86 => {
                let addr = operand as u16;
                self.stx(addr);
            }
            0x96 => {
                let addr = self.zpy_addr(operand);
                self.stx(addr);
            }
            0x84 => {
                let addr = operand as u16;
                self.sty(addr);
            }
            0x94 => {
                let addr = self.zpx_addr(operand);
                self.sty(addr);
            }
            0x69 => {
                let val = operand;
                self.adc(val);
            }
            0x65 => {
                let addr = operand as u16;
                let val = self.memory.load(addr);
                self.adc(val);
            }
            0x75 => {
                let val = self.get_zpx(operand);
                self.adc(val);
            }
            0x61 => {
                let val = self.get_indx(operand);
                self.adc(val);
            }
            0x71 => {
                let (val, page_crossed) = self.get_indy(operand);
                self.adc(val);
                if page_crossed {
                    cycles += 1;
                }
            }
            0xe9 => {
                let val = operand;
                self.sbc(val);
            }
            0xe5 => {
                let addr = operand as u16;
                let val = self.memory.load(addr);
                self.sbc(val);
            }
            0xf5 => {
                let val = self.get_zpx(operand);
                self.sbc(val);
            }
            0xe1 => {
                let val = self.get_indx(operand);
                self.sbc(val);
            }
            0xf1 => {
                let (val, page_crossed) = self.get_indy(operand);
                self.sbc(val);
                if page_crossed {
                    cycles += 1;
                }
            }
            0xc9 => {
                self.cmp(operand);
            }
            0xc5 => {
                let addr = operand as u16;
                let val = self.memory.load(addr);
                self.cmp(val);
            }
            0xd5 => {
                let val = self.get_zpx(operand);
                self.cmp(val);
            }
            0xc1 => {
                let val = self.get_indx(operand);
                self.cmp(val);
            }
            0xd1 => {
                let (val, page_crossed) = self.get_indy(operand);
                self.cmp(val);
                if page_crossed {
                    cycles += 1;
                }
            }
            0xe0 => {
                let val = operand;
                self.cpx(val);
            }
            0xe4 => {
                let addr = operand as u16;
                let val = self.memory.load(addr);
                self.cpx(val);
            }
            0xc0 => {
                let val = operand;
                self.cpy(val);
            }
            0xc4 => {
                let addr = operand as u16;
                let val = self.memory.load(addr);
                self.cpy(val);
            }
            0x29 => {
                let val = operand;
                self.and(val);
            }
            0x25 => {
                let addr = operand as u16;
                let val = self.memory.load(addr);
                self.and(val);
            }
            0x35 => {
                let val = self.get_zpx(operand);
                self.and(val);
            }
            0x21 => {
                let val = self.get_indx(operand);
                self.and(val);
            }
            0x31 => {
                let (val, page_crossed) = self.get_indy(operand);
                self.and(val);
                if page_crossed {
                    cycles += 1;
                }
            }
            0x09 => {
                let val = operand;
                self.ora(val);
            }
            0x05 => {
                let addr = operand as u16;
                let val = self.memory.load(addr);
                self.ora(val);
            }
            0x15 => {
                let val = self.get_zpx(operand);
                self.ora(val);
            }
            0x01 => {
                let val = self.get_indx(operand);
                self.ora(val);
            }
            0x11 => {
                let (val, page_crossed) = self.get_indy(operand);
                self.ora(val);
                if page_crossed {
                    cycles += 1;
                }
            }
            0x49 => {
                let val = operand;
                self.eor(val);
            }
            0x45 => {
                let addr = operand as u16;
                let val = self.memory.load(addr);
                self.eor(val);
            }
            0x55 => {
                let val = self.get_zpx(operand);
                self.eor(val);
            }
            0x41 => {
                let val = self.get_indx(operand);
                self.eor(val);
            }
            0x51 => {
                let (val, page_crossed) = self.get_indy(operand);
                self.eor(val);
                if page_crossed {
                    cycles += 1;
                }
            }
            0x24 => {
                let addr = operand as u16;
                let val = self.memory.load(addr);
                self.bit(val);
            }
            // rol
            0x26 => {
                let addr = operand as u16;
                let val = self.memory.load(addr);
                let val = self.rol(val);
                self.memory.store(addr, val);
            }
            0x36 => {
                let addr = self.zpx_addr(operand);
                let val = self.memory.load(addr);
                let val = self.rol(val);
                self.memory.store(addr, val);
            }

            // ror
            0x66 => {
                let addr = operand as u16;
                let val = self.memory.load(addr);
                let val = self.ror(val);
                self.memory.store(addr, val);
            }
            0x76 => {
                let addr = self.zpx_addr(operand);
                let val = self.memory.load(addr);
                let val = self.ror(val);
                self.memory.store(addr, val);
            }
            0x06 => {
                let addr = operand as u16;
                let val = self.memory.load(addr);
                let val = self.asl(val);
                self.memory.store(addr, val);
            }
            0x16 => {
                let addr = self.zpx_addr(operand);
                let val = self.memory.load(addr);
                let val = self.asl(val);
                self.memory.store(addr, val);
            }
            0x46 => {
                let addr = operand as u16;
                let val = self.memory.load(addr);
                let val = self.lsr(val);
                self.memory.store(addr, val);
            }
            0x56 => {
                let addr = self.zpx_addr(operand);
                let val = self.memory.load(addr);
                let val = self.lsr(val);
                self.memory.store(addr, val);
            }
            0xe6 => {
                let addr = operand as u16;
                self.inc(addr);
            }
            0xf6 => {
                let addr = self.zpx_addr(operand);
                self.inc(addr);
            }
            0xc6 => {
                let addr = operand as u16;
                self.dec(addr);
            }
            0xd6 => {
                let addr = self.zpx_addr(operand);
                self.dec(addr);
            }
            0x10 => {
                let rel_addr = operand as i8;
                cycles += self.bpl(rel_addr);
            }
            0x30 => {
                let rel_addr = operand as i8;
                cycles += self.bmi(rel_addr);
            }
            0x50 => {
                let rel_addr = operand as i8;
                cycles += self.bvc(rel_addr);
            }
            0x70 => {
                let rel_addr = operand as i8;
                cycles += self.bvs(rel_addr);
            }
            0x90 => {
                let rel_addr = operand as i8;
                cycles += self.bcc(rel_addr);
            }
            0xb0 => {
                let rel_addr = operand as i8;
                cycles += self.bcs(rel_addr);
            }
            0xd0 => {
                let rel_addr = operand as i8;
                cycles += self.bne(rel_addr);
            }
            0xf0 => {
                let rel_addr = operand as i8;
                cycles += self.beq(rel_addr);
            }
            _ => {
                panic!("Unexpected 2-byte instruction encountered.");
            }
        }

        cycles
    }

    fn do_op3(&mut self, opcode: u8, operand: u16) -> u8 {
        let mut cycles = 0;
        match opcode {
            0xad => {
                let val = self.memory.load(operand);
                self.lda(val);
            }
            0xb9 => {
                let (val, _, page_crossed) = self.get_absy(operand);
                self.lda(val);
                if page_crossed {
                    cycles += 1;
                }
            }
            0xbd => {
                let (val, _, page_crossed) = self.get_absx(operand);
                self.lda(val);
                if page_crossed {
                    cycles += 1;
                }
            }
            0xae => {
                let val = self.memory.load(operand);
                self.ldx(val);
            }
            0xbe => {
                let (val, _, page_crossed) = self.get_absy(operand);
                self.ldx(val);
                if page_crossed {
                    cycles += 1;
                }
            }
            0xac => {
                let val = self.memory.load(operand);
                self.ldy(val);
            }
            0xbc => {
                let (val, _, page_crossed) = self.get_absx(operand);
                self.ldy(val);
                if page_crossed {
                    cycles += 1;
                }
            }
            0x8d => {
                let addr = operand;
                self.sta(addr);
            }
            0x9d => {
                let addr = operand + self.registers.irx as u16;
                self.sta(addr);
            }
            0x99 => {
                let addr = operand + self.registers.iry as u16;
                self.sta(addr);
            }
            0x8e => {
                let addr = operand;
                self.stx(addr);
            }
            0x8c => {
                let addr = operand;
                self.sty(addr);
            }
            0x6d => {
                let val = self.memory.load(operand);
                self.adc(val);
            }
            0x7d => {
                let (val, _, page_crossed) = self.get_absx(operand);
                self.adc(val);
                if page_crossed {
                    cycles += 1;
                }
            }
            0x79 => {
                let (val, _, page_crossed) = self.get_absy(operand);
                self.adc(val);
                if page_crossed {
                    cycles += 1
                }
            }
            0xed => {
                let val = self.memory.load(operand);
                self.sbc(val);
            }
            0xfd => {
                let (val, _, page_crossed) = self.get_absx(operand);
                self.sbc(val);
                if page_crossed {
                    cycles += 1;
                }
            }
            0xf9 => {
                let (val, _, page_crossed) = self.get_absy(operand);
                self.sbc(val);
                if page_crossed {
                    cycles += 1;
                }
            }
            0xcd => {
                let val = self.memory.load(operand);
                self.cmp(val);
            }
            0xdd => {
                let (val, _, page_crossed) = self.get_absx(operand);
                self.cmp(val);
                if page_crossed {
                    cycles += 1;
                }
            }
            0xd9 => {
                let (val, _, page_crossed) = self.get_absy(operand);
                self.cmp(val);
                if page_crossed {
                    cycles += 1;
                }
            }
            0x2d => {
                let val = self.memory.load(operand);
                self.and(val);
            }
            0xec => {
                let val = self.memory.load(operand);
                self.cpx(val);
            }
            0xcc => {
                let val = self.memory.load(operand);
                self.cpy(val);
            }
            0x3d => {
                let (val, _, page_crossed) = self.get_absx(operand);
                self.and(val);
                if page_crossed {
                    cycles += 1;
                }
            }
            0x39 => {
                let (val, _, page_crossed) = self.get_absy(operand);
                self.and(val);
                if page_crossed {
                    cycles += 1;
                }
            }
            0x0d => {
                let val = self.memory.load(operand);
                self.ora(val);
            }
            0x1d => {
                let (val, _, page_crossed) = self.get_absx(operand);
                self.ora(val);
                if page_crossed {
                    cycles += 1;
                }
            }
            0x19 => {
                let (val, _, page_crossed) = self.get_absy(operand);
                self.ora(val);
                if page_crossed {
                    cycles += 1;
                }
            }
            0x4d => {
                let val = self.memory.load(operand);
                self.eor(val);
            }
            0x5d => {
                let (val, _, page_crossed) = self.get_absx(operand);
                self.eor(val);
                if page_crossed {
                    cycles += 1;
                }
            }
            0x59 => {
                let (val, _, page_crossed) = self.get_absy(operand);
                self.eor(val);
                if page_crossed {
                    cycles += 1;
                }
            }
            0x2c => {
                let val = self.memory.load(operand);
                self.bit(val);
            }
            0x2e => {
                let addr = operand;
                let val = self.memory.load(addr);
                let val = self.rol(val);
                self.memory.store(addr, val);
            }
            0x3e => {
                let addr = operand;
                let (val, addr, _) = self.get_absx(addr);
                let val = self.rol(val);
                self.memory.store(addr, val);
            }
            0x6e => {
                let addr = operand;
                let val = self.memory.load(addr);
                let val = self.ror(val);
                self.memory.store(addr, val);
            }
            0x7e => {
                let (val, addr, _) = self.get_absx(operand);
                let val = self.ror(val);
                self.memory.store(addr, val);
            }
            0x0e => {
                let addr = operand;
                let val = self.memory.load(operand);
                let val = self.asl(val);
                self.memory.store(addr, val);
            }
            0x1e => {
                let (val, addr, _) = self.get_absx(operand);
                let val = self.asl(val);
                self.memory.store(addr, val);
            }
            0x4e => {
                let addr = operand;
                let val = self.memory.load(addr);
                let val = self.lsr(val);
                self.memory.store(addr, val);
            }
            0x5e => {
                let (val, addr, _) = self.get_absx(operand);
                let val = self.lsr(val);
                self.memory.store(addr, val);
            }
            0xee => {
                let addr = operand;
                self.inc(addr);
            }
            0xfe => {
                let addr = operand + self.registers.irx as u16;
                self.inc(addr);
            }
            0xce => {
                let addr = operand;
                self.dec(addr);
            }
            0xde => {
                let addr = operand + self.registers.irx as u16;
                self.dec(addr);
            }
            0x4c => {
                let addr = operand;
                self.jmp(addr);
            }
            0x6c => {
                let addr = operand;
                let lo_byte = self.memory.load(addr);
                let hi_byte;

                // recreate indirect jump bug in nmos 6502
                if addr & 0x00ff == 0x00ff {
                    hi_byte = self.memory.load(addr & 0xff00);
                } else {
                    hi_byte = self.memory.load(addr + 1);
                }

                let addr = (hi_byte as u16) << 8 | lo_byte as u16;
                self.jmp(addr);
            }
            0x20 => {
                let addr = operand;
                self.jsr(addr);
            }
            _ => {
                panic!("Unexpected 3-byte instruction encountered");
            }
        }

        cycles
    }

    fn do_op1(&mut self, opcode: u8) {
        match opcode {
            0x0a => {
                let val = self.registers.acc;
                self.registers.acc = self.asl(val);
            }
            0x2a => {
                let val = self.registers.acc;
                self.registers.acc = self.rol(val);
            }
            0x6a => {
                let val = self.registers.acc;
                self.registers.acc = self.ror(val);
            }
            0x4a => {
                let val = self.registers.acc;
                self.registers.acc = self.lsr(val);
            }
            0xe8 => {
                self.inx();
            }
            0xca => {
                self.dex();
            }
            0xc8 => {
                self.iny();
            }
            0x88 => {
                self.dey();
            }
            0xaa => {
                self.tax();
            }
            0xa8 => {
                self.tay();
            }
            0x8a => {
                self.txa();
            }
            0x98 => {
                self.tya();
            }
            0x9a => {
                self.txs();
            }
            0xba => {
                self.tsx();
            }
            0x18 => {
                self.clc();
            }
            0x38 => {
                self.sec();
            }
            0x58 => {
                self.cli();
            }
            0x78 => {
                self.sei();
            }
            0xb8 => {
                self.clv();
            }
            0xd8 => {
                self.cld();
            }
            0xf8 => {
                self.sed();
            }
            0x60 => {
                self.rts();
            }
            0x00 => {
                // The BRK instruction is actually encoded as 2 bytes, one for the
                // instruction, and an additional padding byte.  We increment the
                // program counter to accommodate this, which *must* be done before
                // invoking the brk instruction since it pushes the program counter
                // to the stack.
                self.registers.pc += 1;
                self.brk();
            }

            0x40 => {
                self.rti();
            }
            0x48 => {
                self.pha();
            }
            0x68 => {
                self.pla();
            }
            0x08 => {
                self.php();
            }
            0x28 => {
                self.plp();
            }
            0xea => {
                self.nop();
            }
            _ => {
                panic!("unexpected opcode encountered");
            }
        }
    }

    fn push_stack(&mut self, value: u8) {
        self.memory.store(STACK_LOC + self.registers.sp as u16, value);
        self.registers.sp = (Wrapping(self.registers.sp) - Wrapping(1_u8)).0;
    }

    fn peek_stack(&mut self) -> u8 {
        self.memory.load(STACK_LOC + ((Wrapping(self.registers.sp) + Wrapping(1_u8)).0) as u16)
    }

    fn pop_stack(&mut self) -> u8 {
        let val = self.peek_stack();
        self.registers.sp = (Wrapping(self.registers.sp) + Wrapping(1_u8)).0;
        val
    }

    fn push_stack16(&mut self, value: u16) {
        if self.registers.sp < 2 {
            panic!("stack overflow"); // FIXME: this should wrap, not panic
        }
        self.memory.store16(STACK_LOC + (self.registers.sp - 1) as u16, value);
        self.registers.sp = (Wrapping(self.registers.sp) - Wrapping(2_u8)).0;
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

    /// ## Implementation of the 6502 instruction set
    ///
    /// Any instruction that consumes additional cycles under certain conditions
    /// will return the number of conditional cycles.  This will not include
    /// cycles that can be determined simply by decoding the instruction.

    /// ## Register Transfers (TODO: tests)

    fn tax(&mut self) {
        self.registers.irx = self.registers.acc;
        let x = self.registers.irx;
        self.registers.set_sign_and_zero_flag(x);
    }

    fn tay(&mut self) {
        self.registers.iry = self.registers.acc;
        let y = self.registers.iry;
        self.registers.set_sign_and_zero_flag(y);
    }

    fn txa(&mut self) {
        self.registers.acc = self.registers.irx;
        let acc = self.registers.acc;
        self.registers.set_sign_and_zero_flag(acc);
    }

    fn tya(&mut self) {
        self.registers.acc = self.registers.iry;
        let acc = self.registers.acc;
        self.registers.set_sign_and_zero_flag(acc);
    }

    /// ## Stack Operations
    /// See http://wiki.nesdev.com/w/index.php/Status_flags regarding
    /// status flags and nuances with bytes 4 and 5
    fn tsx(&mut self) {
        self.registers.irx = self.registers.sp;
        let x = self.registers.irx;
        self.registers.set_sign_and_zero_flag(x);
    }

    fn txs(&mut self) {
        self.registers.sp = self.registers.irx;
    }

    fn pha(&mut self) {
        let acc = self.registers.acc;
        self.push_stack(acc);
    }

    fn php(&mut self) {
        let stat = self.registers.stat;
        // Break and unused bits are always set when pushing to stack
        self.push_stack(stat | FL_BRK | FL_UNUSED);
    }

    fn pla(&mut self) {
        let val = self.pop_stack();
        self.registers.set_acc(val);
    }

    fn plp(&mut self) {
        let val = self.pop_stack();
        let stat = self.registers.stat;
        // Break and unused bits are always ignored when pulling from stack
        self.registers.stat = val | (stat & (FL_BRK | FL_UNUSED));
    }

    /// ## Arithmetic

    fn adc(&mut self, rop: u8) {
        // See http://www.righto.com/2012/12/the-6502-overflow-flag-explained.html
        let lop = self.registers.acc;
        let carry = if self.registers.get_flag(FL_CARRY) {
            1
        } else {
            0
        };

        // add using the native word size
        let res = carry + lop as isize + rop as isize;

        // if the operation carries into the 8th bit, carry flag will be 1,
        // and zero othersize.
        let has_carry = res & 0x100 != 0;

        let res = res as u8;

        // Set the overflow flag when both operands have the same sign bit AND
        // the sign bit of the result differs from the two.
        let has_overflow = (lop ^ rop) & 0x80 == 0 && (lop ^ res) & 0x80 != 0;

        self.registers.set_flag(FL_CARRY, has_carry);
        self.registers.set_flag(FL_OVERFLOW, has_overflow);
        self.registers.set_acc(res);
    }

    fn sbc(&mut self, rop: u8) {
        let rop = !rop;
        self.adc(rop);
    }

    fn cmp_base(&mut self, lop: u8, rop: u8) {
        let res = lop as i32 - rop as i32;
        self.registers.set_flag(FL_CARRY, res & 0x100 == 0);
        self.registers.set_sign_and_zero_flag(res as u8);
    }

    fn cmp(&mut self, rop: u8) {
        let lop = self.registers.acc;
        self.cmp_base(lop, rop);
    }

    fn cpx(&mut self, rop: u8) {
        let lop = self.registers.irx;
        self.cmp_base(lop, rop);
    }

    fn cpy(&mut self, rop: u8) {
        let lop = self.registers.iry;
        self.cmp_base(lop, rop);
    }

    /// ## Increments and Decrements

    fn inc(&mut self, addr: u16) {
        let val = self.memory.inc(addr);
        self.registers.set_sign_and_zero_flag(val);
    }

    fn inx(&mut self) {
        self.registers.irx = (self.registers.irx as u16 + 1) as u8;
        let x = self.registers.irx;
        self.registers.set_sign_and_zero_flag(x);
    }

    fn iny(&mut self) {
        self.registers.iry = (self.registers.iry as u16 + 1) as u8;
        let y = self.registers.iry;
        self.registers.set_sign_and_zero_flag(y);
    }

    fn dec(&mut self, addr: u16) {
        let val = self.memory.dec(addr);
        self.registers.set_sign_and_zero_flag(val);
    }

    fn dex(&mut self) {
        self.registers.irx = (self.registers.irx as i16 - 1) as u8;
        let x = self.registers.irx;
        self.registers.set_sign_and_zero_flag(x);
    }

    fn dey(&mut self) {
        self.registers.iry = (self.registers.iry as i16 - 1) as u8;
        let y = self.registers.iry;
        self.registers.set_sign_and_zero_flag(y);
    }

    /// ## Shifts
    ///
    /// All shift operations return the shifted value.  It will be up to the
    /// instruction decoder to apply the value to the accumulator or memory
    /// location.

    fn shift_left(&mut self, val: u8, lsb: bool) -> u8 {
        let carry = (val & 0x80) != 0;
        let res = if lsb { (val << 1) | 0x1 } else { val << 1 };
        self.registers.set_flag(FL_CARRY, carry);
        self.registers.set_sign_and_zero_flag(res);
        res
    }

    fn shift_right(&mut self, val: u8, msb: bool) -> u8 {
        let carry = (val & 0x1) != 0;
        let res = if msb { (val >> 1) | 0x80 } else { val >> 1 };
        self.registers.set_flag(FL_CARRY, carry);
        self.registers.set_sign_and_zero_flag(res);
        res
    }

    fn asl(&mut self, val: u8) -> u8 {
        self.shift_left(val, false)
    }

    fn lsr(&mut self, val: u8) -> u8 {
        self.shift_right(val, false)
    }

    fn rol(&mut self, val: u8) -> u8 {
        let carry_set = self.registers.get_flag(FL_CARRY);
        self.shift_left(val, carry_set)
    }

    fn ror(&mut self, val: u8) -> u8 {
        let carry_set = self.registers.get_flag(FL_CARRY);
        self.shift_right(val, carry_set)
    }

    /// ## Jumps and Calls

    fn jmp(&mut self, loc: u16) {
        self.registers.pc = loc;
    }

    fn jsr(&mut self, loc: u16) {
        let pc = self.registers.pc;
        self.push_stack16(pc - 1);
        self.registers.pc = loc;
    }

    fn rts(&mut self) {
        self.registers.pc = self.pop_stack16() + 1;
    }

    /// ##  Branches

    fn branch(&mut self, condition: bool, rel_addr: i8) -> u8 {
        if condition {
            let old_pc = self.registers.pc;
            self.registers.pc = (self.registers.pc as i32 + rel_addr as i32) as u16;
            if self.registers.page_boundary_crossed(old_pc) {
                2
            } else {
                1
            }
        } else {
            0
        }
    }

    fn bcc(&mut self, rel_addr: i8) -> u8 {
        let carry_clear = !self.registers.get_flag(FL_CARRY);
        self.branch(carry_clear, rel_addr)
    }

    fn bcs(&mut self, rel_addr: i8) -> u8 {
        let carry_set = self.registers.get_flag(FL_CARRY);
        self.branch(carry_set, rel_addr)
    }

    fn beq(&mut self, rel_addr: i8) -> u8 {
        let zero_set = self.registers.get_flag(FL_ZERO);
        self.branch(zero_set, rel_addr)
    }

    fn bmi(&mut self, rel_addr: i8) -> u8 {
        let sign_set = self.registers.get_flag(FL_SIGN);
        self.branch(sign_set, rel_addr)
    }

    fn bne(&mut self, rel_addr: i8) -> u8 {
        let zero_clear = !self.registers.get_flag(FL_ZERO);
        self.branch(zero_clear, rel_addr)
    }

    fn bpl(&mut self, rel_addr: i8) -> u8 {
        let sign_clear = !self.registers.get_flag(FL_SIGN);
        self.branch(sign_clear, rel_addr)
    }

    fn bvc(&mut self, rel_addr: i8) -> u8 {
        let overflow_clear = !self.registers.get_flag(FL_OVERFLOW);
        self.branch(overflow_clear, rel_addr)
    }

    fn bvs(&mut self, rel_addr: i8) -> u8 {
        let overflow_set = self.registers.get_flag(FL_OVERFLOW);
        self.branch(overflow_set, rel_addr)
    }

    /// Status Flag Changes

    fn clc(&mut self) {
        self.registers.set_flag(FL_CARRY, false);
    }

    fn cld(&mut self) {
        self.registers.set_flag(FL_DECIMAL, false);
    }

    fn cli(&mut self) {
        self.registers.set_flag(FL_INTERRUPT_DISABLE, false);
    }

    fn clv(&mut self) {
        self.registers.set_flag(FL_OVERFLOW, false);
    }

    fn sec(&mut self) {
        self.registers.set_flag(FL_CARRY, true);
    }

    fn sed(&mut self) {
        self.registers.set_flag(FL_DECIMAL, true);
    }

    fn sei(&mut self) {
        self.registers.set_flag(FL_INTERRUPT_DISABLE, true);
    }

    /// ## Load/Store Operations

    fn lda(&mut self, val: u8) {
        self.registers.set_acc(val);
    }

    fn ldx(&mut self, val: u8) {
        self.registers.irx = val;
        self.registers.set_sign_and_zero_flag(val);
    }

    fn ldy(&mut self, val: u8) {
        self.registers.iry = val;
        self.registers.set_sign_and_zero_flag(val);
    }

    fn sta(&mut self, addr: u16) {
        self.memory.store(addr, self.registers.acc);
    }

    fn stx(&mut self, addr: u16) {
        self.memory.store(addr, self.registers.irx);
    }

    fn sty(&mut self, addr: u16) {
        self.memory.store(addr, self.registers.iry);
    }

    /// ## Logical (todo: tests)

    fn and(&mut self, rop: u8) {
        let lop = self.registers.acc;
        let res = lop & rop;
        self.registers.set_acc(res);
    }

    fn eor(&mut self, rop: u8) {
        let lop = self.registers.acc;
        let res = lop ^ rop;
        self.registers.set_acc(res);
    }

    fn ora(&mut self, rop: u8) {
        let lop = self.registers.acc;
        let res = lop | rop;
        self.registers.set_acc(res);
    }

    fn bit(&mut self, rop: u8) {
        let lop = self.registers.acc;
        let res = lop & rop;

        self.registers.set_flag(FL_ZERO, res == 0);
        self.registers.set_flag(FL_OVERFLOW, rop & 0x40 != 0);
        self.registers.set_flag(FL_SIGN, rop & 0x80 != 0);
    }

    /// ## System Functions (todo: tests)

    fn brk(&mut self) {
        let pc = self.registers.pc;
        let status = self.registers.stat;
        self.push_stack16(pc);
        self.push_stack(status);
        let irq_handler = self.memory.load16(BRK_VECTOR);
        self.registers.pc = irq_handler;
        self.registers.set_flag(FL_INTERRUPT_DISABLE, true);
    }

    fn nop(&mut self) {}

    fn rti(&mut self) {
        let stat = self.pop_stack();
        let pc = self.pop_stack16();
        self.registers.stat = stat;
        self.registers.pc = pc;
    }
}
