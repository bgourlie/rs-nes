mod adc;

use cpu::Cpu;
use memory::Memory;
use super::debugger::Debugger;
use super::addressing::AddressingMode;
use self::adc::Adc;

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

trait OpCode<M, D>
    where M: Memory,
          D: Debugger<M>
{
    fn execute(self, cpu: &mut Cpu<M, D>) -> usize;
}

fn decode_next<M, D, AM, OP>(cpu: &mut Cpu<M, D>) -> OP
    where M: Memory,
          D: Debugger<M>,
          OP: OpCode<M, D>
{

    let opcode = cpu.read_op();
    let mut base_cycles = CYCLE_TABLE[opcode as usize] as usize;

    match opcode {
        // ## Single Byte Instructions
        0x0a => {
            //            self.asl(Accumulator);
            unimplemented!()
        }
        0x2a => {
            //            self.rol(Accumulator);
            unimplemented!()
        }
        0x6a => {
            //            self.ror(Accumulator);
            unimplemented!()
        }
        0x4a => {
            //            self.lsr(Accumulator);
            unimplemented!()
        }
        0xe8 => {
            //            self.inx();
            unimplemented!()
        }
        0xca => {
            //            self.dex();
            unimplemented!()
        }
        0xc8 => {
            //            self.iny();
            unimplemented!()
        }
        0x88 => {
            //            self.dey();
            unimplemented!()
        }
        0xaa => {
            //            self.tax();
            unimplemented!()
        }
        0xa8 => {
            //            self.tay();
            unimplemented!()
        }
        0x8a => {
            //            self.txa();
            unimplemented!()
        }
        0x98 => {
            //            self.tya();
            unimplemented!()
        }
        0x9a => {
            //            self.txs();
            unimplemented!()
        }
        0xba => {
            //            self.tsx();
            unimplemented!()
        }
        0x18 => {
            //            self.clc();
            unimplemented!()
        }
        0x38 => {
            //            self.sec();
            unimplemented!()
        }
        0x58 => {
            //            self.cli();
            unimplemented!()
        }
        0x78 => {
            //            self.sei();
            unimplemented!()
        }
        0xb8 => {
            //            self.clv();
            unimplemented!()
        }
        0xd8 => {
            //            self.cld();
            unimplemented!()
        }
        0xf8 => {
            //            self.sed();
            unimplemented!()
        }
        0x60 => {
            //            self.rts();
            unimplemented!()
        }
        0x00 => {
            // The BRK instruction is actually encoded as 2 bytes, one for the
            // instruction, and an additional padding byte.  We increment the
            // program counter to accommodate this, which must be done *before*
            // invoking the brk instruction since it pushes the program counter
            // to the stack.
            cpu.registers.pc += 1;
            //            self.brk();
            unimplemented!()
        }

        0x40 => {
            //            self.rti();
            unimplemented!()
        }
        0x48 => {
            //            self.pha();
            unimplemented!()
        }
        0x68 => {
            //            self.pla();
            unimplemented!()
        }
        0x08 => {
            //            self.php();
            unimplemented!()
        }
        0x28 => {
            //            self.plp();
            unimplemented!()
        }
        0xea => {
            //            self.nop();
            unimplemented!()
        }
        // ## Two Byte Instructions
        0xa1 => {
            // LDA Indirect,X
            //            let base_addr = self.read_op();
            //            let addr = self.indexed_indirect_addr(base_addr);
            //            self.lda(addr);
            unimplemented!()
        }
        0xa5 => {
            // LDA Zero Page
            //            let addr = self.read_op() as u16;
            //            self.lda(addr);
            unimplemented!()
        }
        0xa9 => {
            // LDA Immediate
            //            let val = self.read_op();
            //            self.lda(val);
            unimplemented!()
        }
        0xb1 => {
            // LDA Indirect,X
            //            let base_addr = self.read_op();
            //            let target_addr = self.indirect_indexed_addr(base_addr);
            //            let page_crossed = page_crossed(base_addr as u16, target_addr);
            //            self.lda(target_addr);
            //            if page_crossed {
            //                cycles += 1;
            //            }
            unimplemented!()
        }
        0xb5 => {
            // LDA Zero Page,X
            //            let base_addr = self.read_op();
            //            let target_addr = self.zpx_addr(base_addr);
            //            self.lda(target_addr);
            unimplemented!()
        }
        0xa2 => {
            // LDX Immediate
            //            let val = self.read_op();
            //            self.ldx(val);
            unimplemented!()
        }
        0xa6 => {
            // LDX Zero Page
            //            let addr = self.read_op() as u16;
            //            self.ldx(addr);
            unimplemented!()
        }
        0xb6 => {
            // LDX Zero Page,Y
            //            let base_addr = self.read_op();
            //            let val = self.zpy_addr(base_addr);
            //            self.ldx(val);
            unimplemented!()
        }
        0xa0 => {
            // LDY Immediate
            //            let val = self.read_op();
            //            self.ldy(val);
            unimplemented!()
        }
        0xa4 => {
            // LDY Zero Page
            //            let addr = self.read_op() as u16;
            //            self.ldy(addr);
            unimplemented!()
        }
        0xb4 => {
            // LDY Zero Page,X
            //            let base_addr = self.read_op();
            //            let addr = self.zpx_addr(base_addr);
            //            self.ldy(addr);
            unimplemented!()
        }
        0x85 => {
            //            let addr = self.read_op() as u16;
            //            self.sta(addr);
            unimplemented!()
        }
        0x95 => {
            //            let base_addr = self.read_op();
            //            let addr = self.zpx_addr(base_addr);
            //            self.sta(addr);
            unimplemented!()
        }
        0x81 => {
            //            let base_addr = self.read_op();
            //            let addr = self.indexed_indirect_addr(base_addr);
            //            self.sta(addr);
            unimplemented!()
        }
        0x91 => {
            //            let base_addr = self.read_op();
            //            let addr = self.indirect_indexed_addr(base_addr);
            //            self.sta(addr);
            unimplemented!()
        }
        0x86 => {
            //            let addr = self.read_op() as u16;
            //            self.stx(addr);
            unimplemented!()
        }
        0x96 => {
            //            let base_addr = self.read_op();
            //            let addr = self.zpy_addr(base_addr);
            //            self.stx(addr);
            unimplemented!()
        }
        0x84 => {
            //            let addr = self.read_op() as u16;
            //            self.sty(addr);
            unimplemented!()
        }
        0x94 => {
            //            let base_addr = self.read_op();
            //            let addr = self.zpx_addr(base_addr);
            //            self.sty(addr);
            unimplemented!()
        }
        0x69 => {
            let val = cpu.read_op();
            let op: OP = Adc::new(base_cycles, val);
            op
        }
        0x65 => {
            let addr = cpu.read_op() as u16;
            Adc::new(base_cycles, addr)
        }
        0x75 => {
            let base_addr = cpu.read_op();
            let addr = cpu.zpx_addr(base_addr);
            Adc::new(base_cycles, addr)
        }
        0x61 => {
            let base_addr = cpu.read_op();
            let addr = cpu.indexed_indirect_addr(base_addr);
            Adc::new(base_cycles, addr)
        }
        0x71 => {
            let base_addr = cpu.read_op();
            let addr = cpu.indirect_indexed_addr(base_addr);
//            let page_crossed = page_crossed(base_addr as u16, addr);
            Adc::new(base_cycles, addr)
            //            if page_crossed {
            //                base_cycles += 1;
            //            }
        }
        0xe9 => {
            //            let val = self.read_op();
            //            self.sbc(val);
            unimplemented!()
        }
        0xe5 => {
            //            let addr = self.read_op() as u16;
            //            self.sbc(addr);
            unimplemented!()
        }
        0xf5 => {
            //            let base_addr = self.read_op();
            //            let addr = self.zpx_addr(base_addr);
            //            self.sbc(addr);
            unimplemented!()
        }
        0xe1 => {
            //            let base_addr = self.read_op();
            //            let addr = self.indexed_indirect_addr(base_addr);
            //            self.sbc(addr);
            unimplemented!()
        }
        0xf1 => {
            //            let base_addr = self.read_op();
            //            let addr = self.indirect_indexed_addr(base_addr);
            //            let page_crossed = page_crossed(base_addr as u16, addr);
            //            self.sbc(addr);
            //            if page_crossed {
            //                cycles += 1;
            //            }
            unimplemented!()
        }
        0xc9 => {
            //            let val = self.read_op();
            //            self.cmp(val);
            unimplemented!()
        }
        0xc5 => {
            //            let addr = self.read_op() as u16;
            //            self.cmp(addr);
            unimplemented!()
        }
        0xd5 => {
            //            let base_addr = self.read_op();
            //            let addr = self.zpx_addr(base_addr);
            //            self.cmp(addr);
            unimplemented!()
        }
        0xc1 => {
            //            let base_addr = self.read_op();
            //            let addr = self.indexed_indirect_addr(base_addr);
            //            self.cmp(addr);
            unimplemented!()
        }
        0xd1 => {
            //            let base_addr = self.read_op();
            //            let (addr, page_crossed) = self.indy_addr(base_addr);
            //            self.cmp(addr);
            //            if page_crossed {
            //                cycles += 1;
            //            }
            unimplemented!()
        }
        0xe0 => {
            //            let val = self.read_op();
            //            self.cpx(val);
            unimplemented!()
        }
        0xe4 => {
            //            let addr = self.read_op() as u16;
            //            self.cpx(addr);
            unimplemented!()
        }
        0xc0 => {
            //            let val = self.read_op();
            //            self.cpy(val);
            unimplemented!()
        }
        0xc4 => {
            //            let addr = self.read_op() as u16;
            //            self.cpy(addr);
            unimplemented!()
        }
        0x29 => {
            //            let val = self.read_op();
            //            self.and(val);
            unimplemented!()
        }
        0x25 => {
            //            let addr = self.read_op() as u16;
            //            self.and(addr);
            unimplemented!()
        }
        0x35 => {
            //            let base_addr = self.read_op();
            //            let addr = self.zpx_addr(base_addr);
            //            self.and(addr);
            unimplemented!()
        }
        0x21 => {
            //            let base_addr = self.read_op();
            //            let addr = self.indexed_indirect_addr(base_addr);
            //            self.and(addr);
            unimplemented!()
        }
        0x31 => {
            //            let base_addr = self.read_op();
            //            let addr = self.indirect_indexed_addr(base_addr);
            //            let page_crossed = page_crossed(base_addr as u16, addr);
            //            self.and(addr);
            //            if page_crossed {
            //                cycles += 1;
            //            }
            unimplemented!()
        }
        0x09 => {
            //            let val = self.read_op();
            //            self.ora(val);
            unimplemented!()
        }
        0x05 => {
            //            let addr = self.read_op() as u16;
            //            self.ora(addr);
            unimplemented!()
        }
        0x15 => {
            //            let base_addr = self.read_op();
            //            let addr = self.zpx_addr(base_addr);
            //            self.ora(addr);
            unimplemented!()
        }
        0x01 => {
            //            let base_addr = self.read_op();
            //            let addr = self.indexed_indirect_addr(base_addr);
            //            self.ora(addr);
            unimplemented!()
        }
        0x11 => {
            //            let base_addr = self.read_op();
            //            let (addr, page_crossed) = self.indy_addr(base_addr);
            //            self.ora(addr);
            //            if page_crossed {
            //                cycles += 1;
            //            }
            unimplemented!()
        }
        0x49 => {
            //            let val = self.read_op();
            //            self.eor(val);
            unimplemented!()
        }
        0x45 => {
            //            let addr = self.read_op() as u16;
            //            self.eor(addr);
            unimplemented!()
        }
        0x55 => {
            //            let base_addr = self.read_op();
            //            let addr = self.zpx_addr(base_addr);
            //            self.eor(addr);
            unimplemented!()
        }
        0x41 => {
            //            let base_addr = self.read_op();
            //            let addr = self.indexed_indirect_addr(base_addr);
            //            self.eor(addr);
            unimplemented!()
        }
        0x51 => {
            //            let base_addr = self.read_op();
            //            let addr = self.indirect_indexed_addr(base_addr);
            //            let page_crossed = page_crossed(base_addr as u16, addr);
            //            self.eor(addr);
            //            if page_crossed {
            //                cycles += 1;
            //            }
            unimplemented!()
        }
        0x24 => {
            //            let addr = self.read_op() as u16;
            //            self.bit(addr);
            unimplemented!()
        }
        // rol
        0x26 => {
            //            let addr = self.read_op() as u16;
            //            self.rol(addr);
            unimplemented!()
        }
        0x36 => {
            //            let base_addr = self.read_op();
            //            let addr = self.zpx_addr(base_addr);
            //            self.rol(addr);
            unimplemented!()
        }

        // ror
        0x66 => {
            //            let addr = self.read_op() as u16;
            //            self.ror(addr);
            unimplemented!()
        }
        0x76 => {
            //            let base_addr = self.read_op();
            //            let addr = self.zpx_addr(base_addr);
            //            self.ror(addr);
            unimplemented!()
        }
        0x06 => {
            //            let addr = self.read_op() as u16;
            //            self.asl(addr);
            unimplemented!()
        }
        0x16 => {
            //            let base_addr = self.read_op();
            //            let addr = self.zpx_addr(base_addr);
            //            self.asl(addr);
            unimplemented!()
        }
        0x46 => {
            //            let addr = self.read_op() as u16;
            //            self.lsr(addr);
            unimplemented!()
        }
        0x56 => {
            //            let operand = self.read_op();
            //            let addr = self.zpx_addr(operand);
            //            self.lsr(addr);
            unimplemented!()
        }
        0xe6 => {
            //            let addr = self.read_op() as u16;
            //            self.inc(addr);
            unimplemented!()
        }
        0xf6 => {
            //            let base_addr = self.read_op();
            //            let addr = self.zpx_addr(base_addr);
            //            self.inc(addr);
            unimplemented!()
        }
        0xc6 => {
            //            let addr = self.read_op() as u16;
            //            self.dec(addr);
            unimplemented!()
        }
        0xd6 => {
            //            let base_addr = self.read_op();
            //            let addr = self.zpx_addr(base_addr);
            //            self.dec(addr);
            unimplemented!()
        }
        0x10 => {
            //            let addr = self.read_op() as i8;
            //            cycles += self.bpl(addr);
            unimplemented!()
        }
        0x30 => {
            //            let addr = self.read_op() as i8;
            //            cycles += self.bmi(addr);
            unimplemented!()
        }
        0x50 => {
            //            let addr = self.read_op() as i8;
            //            cycles += self.bvc(addr);
            unimplemented!()
        }
        0x70 => {
            //            let addr = self.read_op() as i8;
            //            cycles += self.bvs(addr);
            unimplemented!()
        }
        0x90 => {
            //            let addr = self.read_op() as i8;
            //            cycles += self.bcc(addr);
            unimplemented!()
        }
        0xb0 => {
            //            let addr = self.read_op() as i8;
            //            cycles += self.bcs(addr);
            unimplemented!()
        }
        0xd0 => {
            //            let addr = self.read_op() as i8;
            //            cycles += self.bne(addr);
            unimplemented!()
        }
        0xf0 => {
            //            let addr = self.read_op() as i8;
            //            cycles += self.beq(addr);
            unimplemented!()
        }
        // ## Three byte instructions
        0xad => {
            //            // LDA Absolute
            //            let target_addr = self.read_op16();
            //            self.lda(target_addr);
            unimplemented!()
        }
        0xb9 => {
            // LDA Absolute,Y
            //            let base_addr = self.read_op16();
            //            let (addr, page_crossed) = self.absy_addr(base_addr);
            //            self.lda(addr);
            //            if page_crossed {
            //                cycles += 1;
            //            }
            unimplemented!()
        }
        0xbd => {
            // LDA Absolute,X
            //            let base_addr = self.read_op16();
            //            let (addr, page_crossed) = self.absx_addr(base_addr);
            //            self.lda(addr);
            //            if page_crossed {
            //                cycles += 1;
            //            }
            unimplemented!()
        }

        0xae => {
            // LDX Absolute
            //            let addr = self.read_op16();
            //            let val = self.memory.load(addr);
            //            self.ldx(val);
            unimplemented!()
        }
        0xbe => {
            // LDX Absolute,Y
            //            let base_addr = self.read_op16();
            //            let (addr, page_crossed) = self.absy_addr(base_addr);
            //            self.ldx(addr);
            //            if page_crossed {
            //                cycles += 1;
            //            }
            unimplemented!()
        }
        0xac => {
            // LDY Absolute
            //            let addr = self.read_op16();
            //            self.ldy(addr);
            unimplemented!()
        }
        0xbc => {
            // LDY Absolute,X
            //            let base_addr = self.read_op16();
            //            let (addr, page_crossed) = self.absx_addr(base_addr);
            //            self.ldy(addr);
            //            if page_crossed {
            //                cycles += 1;
            //            }
            unimplemented!()
        }
        0x8d => {
            //            let addr = self.read_op16();
            //            self.sta(addr);
            unimplemented!()
        }
        0x9d => {
            //            let base_addr = self.read_op16();
            //            let (addr, _) = self.absx_addr(base_addr);
            //            self.sta(addr);
            unimplemented!()
        }
        0x99 => {
            //            let base_addr = self.read_op16();
            //            let (addr, _) = self.absy_addr(base_addr);
            //            self.sta(addr);
            unimplemented!()
        }
        0x8e => {
            //            let addr = self.read_op16();
            //            self.stx(addr);
            unimplemented!()
        }
        0x8c => {
            //            let addr = self.read_op16();
            //            self.sty(addr);
            unimplemented!()
        }
        0x6d => {
            //            let addr = self.read_op16();
            //            self.adc(addr);
            unimplemented!()
        }
        0x7d => {
            //            let base_addr = self.read_op16();
            //            let (addr, page_crossed) = self.absx_addr(base_addr);
            //            self.adc(addr);
            //            if page_crossed {
            //                cycles += 1;
            //            }
            unimplemented!()
        }
        0x79 => {
            //            let base_addr = self.read_op16();
            //            let (addr, page_crossed) = self.absy_addr(base_addr);
            //            self.adc(addr);
            //            if page_crossed {
            //                cycles += 1
            //            }
            unimplemented!()
        }
        0xed => {
            //            let addr = self.read_op16();
            //            self.sbc(addr);
            unimplemented!()
        }
        0xfd => {
            //            let base_addr = self.read_op16();
            //            let (addr, page_crossed) = self.absx_addr(base_addr);
            //            self.sbc(addr);
            //            if page_crossed {
            //                cycles += 1;
            //            }
            unimplemented!()
        }
        0xf9 => {
            //            let base_addr = self.read_op16();
            //            let (addr, page_crossed) = self.absy_addr(base_addr);
            //            self.sbc(addr);
            //            if page_crossed {
            //                cycles += 1;
            //            }
            unimplemented!()
        }
        0xcd => {
            //            let addr = self.read_op16();
            //            self.cmp(addr);
            unimplemented!()
        }
        0xdd => {
            //            let base_addr = self.read_op16();
            //            let (addr, page_crossed) = self.absx_addr(base_addr);
            //            self.cmp(addr);
            //            if page_crossed {
            //                cycles += 1;
            //            }
            unimplemented!()
        }
        0xd9 => {
            //            let base_addr = self.read_op16();
            //            let (addr, page_crossed) = self.absy_addr(base_addr);
            //            self.cmp(addr);
            //            if page_crossed {
            //                cycles += 1;
            //            }
            unimplemented!()
        }
        0x2d => {
            //            let addr = self.read_op16();
            //            let val = self.memory.load(addr);
            //            self.and(val);
            unimplemented!()
        }
        0xec => {
            //            let addr = self.read_op16();
            //            self.cpx(addr);
            unimplemented!()
        }
        0xcc => {
            //            let addr = self.read_op16();
            //            self.cpy(addr);
            unimplemented!()
        }
        0x3d => {
            //            let base_addr = self.read_op16();
            //            let (addr, page_crossed) = self.absx_addr(base_addr);
            //            self.and(addr);
            //            if page_crossed {
            //                cycles += 1;
            //            }
            unimplemented!()
        }
        0x39 => {
            //            let base_addr = self.read_op16();
            //            let (addr, page_crossed) = self.absy_addr(base_addr);
            //            self.and(addr);
            //            if page_crossed {
            //                cycles += 1;
            //            }
            unimplemented!()
        }
        0x0d => {
            //            let addr = self.read_op16();
            //            self.ora(addr);
            unimplemented!()
        }
        0x1d => {
            //            let base_addr = self.read_op16();
            //            let (addr, page_crossed) = self.absx_addr(base_addr);
            //            self.ora(addr);
            //            if page_crossed {
            //                cycles += 1;
            //            }
            unimplemented!()
        }
        0x19 => {
            //            let base_addr = self.read_op16();
            //            let (addr, page_crossed) = self.absy_addr(base_addr);
            //            self.ora(addr);
            //            if page_crossed {
            //                cycles += 1;
            //            }
            unimplemented!()
        }
        0x4d => {
            //            let addr = self.read_op16();
            //            self.eor(addr);
            unimplemented!()
        }
        0x5d => {
            //            let base_addr = self.read_op16();
            //            let (addr, page_crossed) = self.absx_addr(base_addr);
            //            self.eor(addr);
            //            if page_crossed {
            //                cycles += 1;
            //            }
            unimplemented!()
        }
        0x59 => {
            //            let base_addr = self.read_op16();
            //            let (addr, page_crossed) = self.absy_addr(base_addr);
            //            self.eor(addr);
            //            if page_crossed {
            //                cycles += 1;
            //            }
            unimplemented!()
        }
        0x2c => {
            //            let addr = self.read_op16();
            //            self.bit(addr);
            unimplemented!()
        }
        0x2e => {
            //            let addr = self.read_op16();
            //            self.rol(addr);
            unimplemented!()
        }
        0x3e => {
            //            let base_addr = self.read_op16();
            //            let (addr, _) = self.absx_addr(base_addr);
            //            self.rol(addr);
            unimplemented!()
        }
        0x6e => {
            //            let addr = self.read_op16();
            //            self.ror(addr);
            unimplemented!()
        }
        0x7e => {
            //            let base_addr = self.read_op16();
            //            let (addr, _) = self.absx_addr(base_addr);
            //            self.ror(addr);
            unimplemented!()
        }
        0x0e => {
            //            let addr = self.read_op16();
            //            self.asl(addr);
            unimplemented!()
        }
        0x1e => {
            //            let base_addr = self.read_op16();
            //            let (addr, _) = self.absx_addr(base_addr);
            //            self.asl(addr);
            unimplemented!()
        }
        0x4e => {
            //            let addr = self.read_op16();
            //            self.lsr(addr);
            unimplemented!()
        }
        0x5e => {
            //            let base_addr = self.read_op16();
            //            let (addr, _) = self.absx_addr(base_addr);
            //            self.lsr(addr);
            unimplemented!()
        }
        0xee => {
            //            let addr = self.read_op16();
            //            self.inc(addr);
            unimplemented!()
        }
        0xfe => {
            //            let base_addr = self.read_op16();
            //            let (addr, _) = self.absx_addr(base_addr);
            //            self.inc(addr);
            unimplemented!()
        }
        0xce => {
            //            let addr = self.read_op16();
            //            self.dec(addr);
            unimplemented!()
        }
        0xde => {
            //            let base_addr = self.read_op16();
            //            let (addr, _) = self.absx_addr(base_addr);
            //            self.dec(addr);
            unimplemented!()
        }
        0x4c => {
            //            let addr = self.read_op16();
            //            self.jmp(addr);
            unimplemented!()
        }
        0x6c => {
            //            let addr = self.read_op16();
            //            self.indirect_jmp(addr);
            unimplemented!()
        }
        0x20 => {
            //            let addr = self.read_op16();
            //            self.jsr(addr);
            unimplemented!()
        }
        _ => {
            panic!("unexpected opcode encountered");
        }
    }
}
