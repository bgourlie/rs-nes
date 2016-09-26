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
mod addressing;

use std::num::Wrapping;

use self::registers::*;
use self::addressing::*;
use constants::*;
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

// TODO: consolidate logic with similar implementation in Register
fn page_crossed(val1: u16, val2: u16) -> bool {
    val1 & 0xFF00 != val2 & 0xFF00
}

pub struct Cpu<T: Memory> {
    pub cycles: u64,
    pub registers: Registers,
    pub memory: T,
}

impl<Mem: Memory> Cpu<Mem> {
    pub fn new(memory: Mem) -> Self {
        Cpu {
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

    pub fn step(&mut self) {
        let opcode = self.read_op();
        let mut cycles = CYCLE_TABLE[opcode as usize];

        match opcode {
            // ## Single Byte Instructions
            0x0a => {
                self.asl(Accumulator);
            }
            0x2a => {
                self.rol(Accumulator);
            }
            0x6a => {
                self.ror(Accumulator);
            }
            0x4a => {
                self.lsr(Accumulator);
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
                // program counter to accommodate this, which must be done *before*
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
            // ## Two Byte Instructions
            0xa1 => {
                // LDA Indirect,X
                let base_addr = self.read_op();
                let addr = self.indexed_indirect_addr(base_addr);
                self.lda(addr);
            }
            0xa5 => {
                // LDA Zero Page
                let addr = self.read_op() as u16;
                self.lda(addr);
            }
            0xa9 => {
                // LDA Immediate
                let val = self.read_op();
                self.lda(val);
            }
            0xb1 => {
                // LDA Indirect,X
                let base_addr = self.read_op();
                let target_addr = self.indirect_indexed_addr(base_addr);
                let page_crossed = page_crossed(base_addr as u16, target_addr);
                self.lda(target_addr);
                if page_crossed {
                    cycles += 1;
                }
            }
            0xb5 => {
                // LDA Zero Page,X
                let base_addr = self.read_op();
                let target_addr = self.zpx_addr(base_addr);
                self.lda(target_addr);
            }
            0xa2 => {
                // LDX Immediate
                let val = self.read_op();
                self.ldx(val);
            }
            0xa6 => {
                // LDX Zero Page
                let addr = self.read_op() as u16;
                self.ldx(addr);
            }
            0xb6 => {
                // LDX Zero Page,Y
                let base_addr = self.read_op();
                let val = self.zpy_addr(base_addr);
                self.ldx(val);
            }
            0xa0 => {
                // LDY Immediate
                let val = self.read_op();
                self.ldy(val);
            }
            0xa4 => {
                // LDY Zero Page
                let addr = self.read_op() as u16;
                self.ldy(addr);
            }
            0xb4 => {
                // LDY Zero Page,X
                let base_addr = self.read_op();
                let addr = self.zpx_addr(base_addr);
                self.ldy(addr);
            }
            0x85 => {
                let addr = self.read_op() as u16;
                self.sta(addr);
            }
            0x95 => {
                let base_addr = self.read_op();
                let addr = self.zpx_addr(base_addr);
                self.sta(addr);
            }
            0x81 => {
                let base_addr = self.read_op();
                let addr = self.indexed_indirect_addr(base_addr);
                self.sta(addr);
            }
            0x91 => {
                let base_addr = self.read_op();
                let addr = self.indirect_indexed_addr(base_addr);
                self.sta(addr);
            }
            0x86 => {
                let addr = self.read_op() as u16;
                self.stx(addr);
            }
            0x96 => {
                let base_addr = self.read_op();
                let addr = self.zpy_addr(base_addr);
                self.stx(addr);
            }
            0x84 => {
                let addr = self.read_op() as u16;
                self.sty(addr);
            }
            0x94 => {
                let base_addr = self.read_op();
                let addr = self.zpx_addr(base_addr);
                self.sty(addr);
            }
            0x69 => {
                let val = self.read_op();
                self.adc(val);
            }
            0x65 => {
                let addr = self.read_op() as u16;
                self.adc(addr);
            }
            0x75 => {
                let base_addr = self.read_op();
                let addr = self.zpx_addr(base_addr);
                self.adc(addr);
            }
            0x61 => {
                let base_addr = self.read_op();
                let addr = self.indexed_indirect_addr(base_addr);
                self.adc(addr);
            }
            0x71 => {
                let base_addr = self.read_op();
                let addr = self.indirect_indexed_addr(base_addr);
                let page_crossed = page_crossed(base_addr as u16, addr);
                self.adc(addr);
                if page_crossed {
                    cycles += 1;
                }
            }
            0xe9 => {
                let val = self.read_op();
                self.sbc(val);
            }
            0xe5 => {
                let addr = self.read_op() as u16;
                self.sbc(addr);
            }
            0xf5 => {
                let base_addr = self.read_op();
                let addr = self.zpx_addr(base_addr);
                self.sbc(addr);
            }
            0xe1 => {
                let base_addr = self.read_op();
                let addr = self.indexed_indirect_addr(base_addr);
                self.sbc(addr);
            }
            0xf1 => {
                let base_addr = self.read_op();
                let addr = self.indirect_indexed_addr(base_addr);
                let page_crossed = page_crossed(base_addr as u16, addr);
                self.sbc(addr);
                if page_crossed {
                    cycles += 1;
                }
            }
            0xc9 => {
                let val = self.read_op();
                self.cmp(val);
            }
            0xc5 => {
                let addr = self.read_op() as u16;
                self.cmp(addr);
            }
            0xd5 => {
                let base_addr = self.read_op();
                let addr = self.zpx_addr(base_addr);
                self.cmp(addr);
            }
            0xc1 => {
                let base_addr = self.read_op();
                let addr = self.indexed_indirect_addr(base_addr);
                self.cmp(addr);
            }
            0xd1 => {
                let base_addr = self.read_op();
                let (addr, page_crossed) = self.indy_addr(base_addr);
                self.cmp(addr);
                if page_crossed {
                    cycles += 1;
                }
            }
            0xe0 => {
                let val = self.read_op();
                self.cpx(val);
            }
            0xe4 => {
                let addr = self.read_op() as u16;
                self.cpx(addr);
            }
            0xc0 => {
                let val = self.read_op();
                self.cpy(val);
            }
            0xc4 => {
                let addr = self.read_op() as u16;
                self.cpy(addr);
            }
            0x29 => {
                let val = self.read_op();
                self.and(val);
            }
            0x25 => {
                let addr = self.read_op() as u16;
                self.and(addr);
            }
            0x35 => {
                let base_addr = self.read_op();
                let addr = self.zpx_addr(base_addr);
                self.and(addr);
            }
            0x21 => {
                let base_addr = self.read_op();
                let addr = self.indexed_indirect_addr(base_addr);
                self.and(addr);
            }
            0x31 => {
                let base_addr = self.read_op();
                let addr = self.indirect_indexed_addr(base_addr);
                let page_crossed = page_crossed(base_addr as u16, addr);
                self.and(addr);
                if page_crossed {
                    cycles += 1;
                }
            }
            0x09 => {
                let val = self.read_op();
                self.ora(val);
            }
            0x05 => {
                let addr = self.read_op() as u16;
                self.ora(addr);
            }
            0x15 => {
                let base_addr = self.read_op();
                let addr = self.zpx_addr(base_addr);
                self.ora(addr);
            }
            0x01 => {
                let base_addr = self.read_op();
                let addr = self.indexed_indirect_addr(base_addr);
                self.ora(addr);
            }
            0x11 => {
                let base_addr = self.read_op();
                let (addr, page_crossed) = self.indy_addr(base_addr);
                self.ora(addr);
                if page_crossed {
                    cycles += 1;
                }
            }
            0x49 => {
                let val = self.read_op();
                self.eor(val);
            }
            0x45 => {
                let addr = self.read_op() as u16;
                self.eor(addr);
            }
            0x55 => {
                let base_addr = self.read_op();
                let addr = self.zpx_addr(base_addr);
                self.eor(addr);
            }
            0x41 => {
                let base_addr = self.read_op();
                let addr = self.indexed_indirect_addr(base_addr);
                self.eor(addr);
            }
            0x51 => {
                let base_addr = self.read_op();
                let addr = self.indirect_indexed_addr(base_addr);
                let page_crossed = page_crossed(base_addr as u16, addr);
                self.eor(addr);
                if page_crossed {
                    cycles += 1;
                }
            }
            0x24 => {
                let addr = self.read_op() as u16;
                let val = self.memory.load(addr);
                self.bit(val);
            }
            // rol
            0x26 => {
                let addr = self.read_op() as u16;
                self.rol(addr);
            }
            0x36 => {
                let base_addr = self.read_op();
                let addr = self.zpx_addr(base_addr);
                self.rol(addr);
            }

            // ror
            0x66 => {
                let addr = self.read_op() as u16;
                self.ror(addr);
            }
            0x76 => {
                let base_addr = self.read_op();
                let addr = self.zpx_addr(base_addr);
                self.ror(addr);
            }
            0x06 => {
                let addr = self.read_op() as u16;
                self.asl(addr);
            }
            0x16 => {
                let base_addr = self.read_op();
                let addr = self.zpx_addr(base_addr);
                self.asl(addr);
            }
            0x46 => {
                let addr = self.read_op() as u16;
                self.lsr(addr);
            }
            0x56 => {
                let operand = self.read_op();
                let addr = self.zpx_addr(operand);
                self.lsr(addr);
            }
            0xe6 => {
                let addr = self.read_op() as u16;
                self.inc(addr);
            }
            0xf6 => {
                let base_addr = self.read_op();
                let addr = self.zpx_addr(base_addr);
                self.inc(addr);
            }
            0xc6 => {
                let addr = self.read_op() as u16;
                self.dec(addr);
            }
            0xd6 => {
                let base_addr = self.read_op();
                let addr = self.zpx_addr(base_addr);
                self.dec(addr);
            }
            0x10 => {
                let addr = self.read_op() as i8;
                cycles += self.bpl(addr);
            }
            0x30 => {
                let addr = self.read_op() as i8;
                cycles += self.bmi(addr);
            }
            0x50 => {
                let addr = self.read_op() as i8;
                cycles += self.bvc(addr);
            }
            0x70 => {
                let addr = self.read_op() as i8;
                cycles += self.bvs(addr);
            }
            0x90 => {
                let addr = self.read_op() as i8;
                cycles += self.bcc(addr);
            }
            0xb0 => {
                let addr = self.read_op() as i8;
                cycles += self.bcs(addr);
            }
            0xd0 => {
                let addr = self.read_op() as i8;
                cycles += self.bne(addr);
            }
            0xf0 => {
                let addr = self.read_op() as i8;
                cycles += self.beq(addr);
            }
            // ## Three byte instructions
            0xad => {
                // LDA Absolute
                let target_addr = self.read_op16();
                self.lda(target_addr);
            }
            0xb9 => {
                // LDA Absolute,Y
                let base_addr = self.read_op16();
                let (addr, page_crossed) = self.absy_addr(base_addr);
                self.lda(addr);
                if page_crossed {
                    cycles += 1;
                }
            }
            0xbd => {
                // LDA Absolute,X
                let base_addr = self.read_op16();
                let (addr, page_crossed) = self.absx_addr(base_addr);
                self.lda(addr);
                if page_crossed {
                    cycles += 1;
                }
            }

            0xae => {
                // LDX Absolute
                let addr = self.read_op16();
                let val = self.memory.load(addr);
                self.ldx(val);
            }
            0xbe => {
                // LDX Absolute,Y
                let base_addr = self.read_op16();
                let (addr, page_crossed) = self.absy_addr(base_addr);
                self.ldx(addr);
                if page_crossed {
                    cycles += 1;
                }
            }
            0xac => {
                // LDY Absolute
                let addr = self.read_op16();
                self.ldy(addr);
            }
            0xbc => {
                // LDY Absolute,X
                let base_addr = self.read_op16();
                let (addr, page_crossed) = self.absx_addr(base_addr);
                self.ldy(addr);
                if page_crossed {
                    cycles += 1;
                }
            }
            0x8d => {
                let addr = self.read_op16();
                self.sta(addr);
            }
            0x9d => {
                let base_addr = self.read_op16();
                let (addr, _) = self.absx_addr(base_addr);
                self.sta(addr);
            }
            0x99 => {
                let base_addr = self.read_op16();
                let (addr, _) = self.absy_addr(base_addr);
                self.sta(addr);
            }
            0x8e => {
                let addr = self.read_op16();
                self.stx(addr);
            }
            0x8c => {
                let addr = self.read_op16();
                self.sty(addr);
            }
            0x6d => {
                let addr = self.read_op16();
                self.adc(addr);
            }
            0x7d => {
                let base_addr = self.read_op16();
                let (addr, page_crossed) = self.absx_addr(base_addr);
                self.adc(addr);
                if page_crossed {
                    cycles += 1;
                }
            }
            0x79 => {
                let base_addr = self.read_op16();
                let (addr, page_crossed) = self.absy_addr(base_addr);
                self.adc(addr);
                if page_crossed {
                    cycles += 1
                }
            }
            0xed => {
                let addr = self.read_op16();
                self.sbc(addr);
            }
            0xfd => {
                let base_addr = self.read_op16();
                let (addr, page_crossed) = self.absx_addr(base_addr);
                self.sbc(addr);
                if page_crossed {
                    cycles += 1;
                }
            }
            0xf9 => {
                let base_addr = self.read_op16();
                let (addr, page_crossed) = self.absy_addr(base_addr);
                self.sbc(addr);
                if page_crossed {
                    cycles += 1;
                }
            }
            0xcd => {
                let addr = self.read_op16();
                self.cmp(addr);
            }
            0xdd => {
                let base_addr = self.read_op16();
                let (addr, page_crossed) = self.absx_addr(base_addr);
                self.cmp(addr);
                if page_crossed {
                    cycles += 1;
                }
            }
            0xd9 => {
                let base_addr = self.read_op16();
                let (addr, page_crossed) = self.absy_addr(base_addr);
                self.cmp(addr);
                if page_crossed {
                    cycles += 1;
                }
            }
            0x2d => {
                let addr = self.read_op16();
                let val = self.memory.load(addr);
                self.and(val);
            }
            0xec => {
                let addr = self.read_op16();
                self.cpx(addr);
            }
            0xcc => {
                let addr = self.read_op16();
                self.cpy(addr);
            }
            0x3d => {
                let base_addr = self.read_op16();
                let (addr, page_crossed) = self.absx_addr(base_addr);
                self.and(addr);
                if page_crossed {
                    cycles += 1;
                }
            }
            0x39 => {
                let base_addr = self.read_op16();
                let (addr, page_crossed) = self.absy_addr(base_addr);
                self.and(addr);
                if page_crossed {
                    cycles += 1;
                }
            }
            0x0d => {
                let addr = self.read_op16();
                self.ora(addr);
            }
            0x1d => {
                let base_addr = self.read_op16();
                let (addr, page_crossed) = self.absx_addr(base_addr);
                self.ora(addr);
                if page_crossed {
                    cycles += 1;
                }
            }
            0x19 => {
                let base_addr = self.read_op16();
                let (addr, page_crossed) = self.absy_addr(base_addr);
                self.ora(addr);
                if page_crossed {
                    cycles += 1;
                }
            }
            0x4d => {
                let addr = self.read_op16();
                self.eor(addr);
            }
            0x5d => {
                let base_addr = self.read_op16();
                let (addr, page_crossed) = self.absx_addr(base_addr);
                self.eor(addr);
                if page_crossed {
                    cycles += 1;
                }
            }
            0x59 => {
                let base_addr = self.read_op16();
                let (addr, page_crossed) = self.absy_addr(base_addr);
                self.eor(addr);
                if page_crossed {
                    cycles += 1;
                }
            }
            0x2c => {
                let addr = self.read_op16();
                let val = self.memory.load(addr);
                self.bit(val);
            }
            0x2e => {
                let addr = self.read_op16();
                self.rol(addr);
            }
            0x3e => {
                let base_addr = self.read_op16();
                let (addr, _) = self.absx_addr(base_addr);
                self.rol(addr);
            }
            0x6e => {
                let addr = self.read_op16();
                self.ror(addr);
            }
            0x7e => {
                let base_addr = self.read_op16();
                let (addr, _) = self.absx_addr(base_addr);
                self.ror(addr);
            }
            0x0e => {
                let addr = self.read_op16();
                self.asl(addr);
            }
            0x1e => {
                let base_addr = self.read_op16();
                let (addr, _) = self.absx_addr(base_addr);
                self.asl(addr);
            }
            0x4e => {
                let addr = self.read_op16();
                self.lsr(addr);
            }
            0x5e => {
                let base_addr = self.read_op16();
                let (addr, _) = self.absx_addr(base_addr);
                self.lsr(addr);
            }
            0xee => {
                let addr = self.read_op16();
                self.inc(addr);
            }
            0xfe => {
                let base_addr = self.read_op16();
                let (addr, _) = self.absx_addr(base_addr);
                self.inc(addr);
            }
            0xce => {
                let addr = self.read_op16();
                self.dec(addr);
            }
            0xde => {
                let base_addr = self.read_op16();
                let (addr, _) = self.absx_addr(base_addr);
                self.dec(addr);
            }
            0x4c => {
                let addr = self.read_op16();
                self.jmp(addr);
            }
            0x6c => {
                let addr = self.read_op16();
                self.indirect_jmp(addr);
            }
            0x20 => {
                let addr = self.read_op16();
                self.jsr(addr);
            }
            _ => {
                panic!("unexpected opcode encountered");
            }
        }

        self.cycles += cycles as u64;
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

    fn abs_indexed_addr(&self, base_addr: u16, index: u8) -> (u16, bool) {
        let target_addr = base_addr + index as u16;
        let page_crossed = page_crossed(base_addr, target_addr);
        (target_addr, page_crossed)
    }

    fn absx_addr(&self, val: u16) -> (u16, bool) {
        let x = self.registers.irx;
        self.abs_indexed_addr(val, x)
    }

    fn absy_addr(&self, val: u16) -> (u16, bool) {
        let y = self.registers.iry;
        self.abs_indexed_addr(val, y)
    }

    fn indy_addr(&self, base_addr: u8) -> (u16, bool) {
        let target_addr = self.indirect_indexed_addr(base_addr);
        let page_crossed = page_crossed(base_addr as u16, target_addr);
        (target_addr, page_crossed)
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

    fn adc_base(&mut self, rop: u8) {
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
        // and zero otherwise.
        let has_carry = res & 0x100 != 0;

        let res = res as u8;

        // Set the overflow flag when both operands have the same sign bit AND
        // the sign bit of the result differs from the two.
        let has_overflow = (lop ^ rop) & 0x80 == 0 && (lop ^ res) & 0x80 != 0;

        self.registers.set_flag(FL_CARRY, has_carry);
        self.registers.set_flag(FL_OVERFLOW, has_overflow);
        self.registers.set_acc(res);
    }

    fn adc<T: AddressReader<Mem>>(&mut self, val: T) {
        let rop = val.read(self);
        self.adc_base(rop);
    }

    fn sbc<T: AddressReader<Mem>>(&mut self, val: T) {
        let rop = val.read(self);
        let rop = !rop;
        self.adc_base(rop);
    }

    fn cmp_base(&mut self, lop: u8, rop: u8) {
        let res = lop as i32 - rop as i32;
        self.registers.set_flag(FL_CARRY, res & 0x100 == 0);
        self.registers.set_sign_and_zero_flag(res as u8);
    }

    fn cmp<T: AddressReader<Mem>>(&mut self, val: T) {
        let rop = val.read(self);
        let lop = self.registers.acc;
        self.cmp_base(lop, rop);
    }

    fn cpx<T: AddressReader<Mem>>(&mut self, val: T) {
        let rop = val.read(self);
        let lop = self.registers.irx;
        self.cmp_base(lop, rop);
    }

    fn cpy<T: AddressReader<Mem>>(&mut self, val: T) {
        let rop = val.read(self);
        let lop = self.registers.iry;
        self.cmp_base(lop, rop);
    }

    /// ## Increments and Decrements

    fn inc<T: AddressWriter<Mem>>(&mut self, addr: T) {
        let val = addr.read(self);
        let val = (Wrapping(val) + Wrapping(1)).0;
        addr.write(self, val);
        self.registers.set_sign_and_zero_flag(val);
    }

    fn inx(&mut self) {
        self.registers.irx = (Wrapping(self.registers.irx) + Wrapping(1)).0;
        let x = self.registers.irx;
        self.registers.set_sign_and_zero_flag(x);
    }

    fn iny(&mut self) {
        self.registers.iry = (Wrapping(self.registers.iry) + Wrapping(1)).0;
        let y = self.registers.iry;
        self.registers.set_sign_and_zero_flag(y);
    }

    fn dec<T: AddressWriter<Mem>>(&mut self, addr: T) {
        let val = addr.read(self);
        let val = (Wrapping(val) - Wrapping(1)).0;
        addr.write(self, val);
        self.registers.set_sign_and_zero_flag(val);
    }

    fn dex(&mut self) {
        self.registers.irx = (Wrapping(self.registers.irx) - Wrapping(1)).0;
        let x = self.registers.irx;
        self.registers.set_sign_and_zero_flag(x);
    }

    fn dey(&mut self) {
        self.registers.iry = (Wrapping(self.registers.iry) - Wrapping(1)).0;
        let y = self.registers.iry;
        self.registers.set_sign_and_zero_flag(y);
    }

    /// ## Shifts

    fn shift_left<T: AddressWriter<Mem>>(&mut self, addr: T, lsb: bool) {
        let val = addr.read(self);
        let carry = (val & 0x80) != 0;
        let res = if lsb { (val << 1) | 0x1 } else { val << 1 };
        self.registers.set_flag(FL_CARRY, carry);
        self.registers.set_sign_and_zero_flag(res);
        addr.write(self, res);
    }

    fn shift_right<T: AddressWriter<Mem>>(&mut self, addr: T, msb: bool) {
        let val = addr.read(self);
        let carry = (val & 0x1) != 0;
        let res = if msb { (val >> 1) | 0x80 } else { val >> 1 };
        self.registers.set_flag(FL_CARRY, carry);
        self.registers.set_sign_and_zero_flag(res);
        addr.write(self, res);
    }

    fn asl<T: AddressWriter<Mem>>(&mut self, addr: T) {
        self.shift_left(addr, false)
    }

    fn lsr<T: AddressWriter<Mem>>(&mut self, addr: T) {
        self.shift_right(addr, false)
    }

    fn rol<T: AddressWriter<Mem>>(&mut self, addr: T) {
        let carry_set = self.registers.get_flag(FL_CARRY);
        self.shift_left(addr, carry_set)
    }

    fn ror<T: AddressWriter<Mem>>(&mut self, addr: T) {
        let carry_set = self.registers.get_flag(FL_CARRY);
        self.shift_right(addr, carry_set)
    }

    /// ## Jumps and Calls

    fn jmp(&mut self, loc: u16) {
        self.registers.pc = loc;
    }

    fn indirect_jmp(&mut self, addr: u16) {
        // Recreate hardware bug specific to indirect jmp
        let lo_byte = self.memory.load(addr);

        // recreate indirect jump bug in nmos 6502
        let hi_byte = if addr & 0x00ff == 0x00ff {
            self.memory.load(addr & 0xff00)
        } else {
            self.memory.load(addr + 1)
        };

        let addr = (hi_byte as u16) << 8 | lo_byte as u16;
        self.jmp(addr);
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

    fn lda<T: AddressReader<Mem>>(&mut self, val: T) {
        let val = val.read(self);
        self.registers.set_acc(val);
    }

    fn ldx<T: AddressReader<Mem>>(&mut self, val: T) {
        let val = val.read(self);
        self.registers.irx = val;
        self.registers.set_sign_and_zero_flag(val);
    }

    fn ldy<T: AddressReader<Mem>>(&mut self, val: T) {
        let val = val.read(self);
        self.registers.iry = val;
        self.registers.set_sign_and_zero_flag(val);
    }

    fn sta<T: AddressWriter<Mem>>(&mut self, addr: T) {
        let acc = self.registers.acc;
        addr.write(self, acc);
    }

    fn stx<T: AddressWriter<Mem>>(&mut self, addr: T) {
        let x = self.registers.irx;
        addr.write(self, x);
    }

    fn sty<T: AddressWriter<Mem>>(&mut self, addr: T) {
        let y = self.registers.iry;
        addr.write(self, y);
    }

    /// ## Logical (todo: tests)

    fn and<T: AddressReader<Mem>>(&mut self, val: T) {
        let rop = val.read(self);
        let lop = self.registers.acc;
        let res = lop & rop;
        self.registers.set_acc(res);
    }

    fn eor<T: AddressReader<Mem>>(&mut self, val: T) {
        let rop = val.read(self);
        let lop = self.registers.acc;
        let res = lop ^ rop;
        self.registers.set_acc(res);
    }

    fn ora<T: AddressReader<Mem>>(&mut self, addr: T) {
        let rop = addr.read(self);
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
