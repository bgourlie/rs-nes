mod addressing_mode;
mod branch_utils;
mod shift_utils;
mod adc;
mod dex;
mod inx;
mod iny;
mod dey;
mod tax;
mod tay;
mod txa;
mod tya;
mod txs;
mod tsx;
mod clc;
mod sec;
mod cli;
mod sei;
mod clv;
mod cld;
mod sed;
mod rts;
mod brk;
mod rti;
mod pha;
mod pla;
mod php;
mod plp;
mod nop;
mod bpl;
mod bmi;
mod bvc;
mod bvs;
mod bcc;
mod bcs;
mod bne;
mod beq;
mod lda;
mod ldx;
mod ldy;
mod sta;
mod stx;
mod sty;
mod sbc;
mod cmp;
mod cpx;
mod cpy;
mod and;
mod ora;
mod eor;
mod bit;
mod rol;
mod ror;
mod asl;
mod lsr;
mod inc;
mod dec;
mod jmp;
mod jsr;

use std::cell::Cell;

use cpu::Cpu;
use memory::Memory;

use self::addressing_mode::*;

trait OpCode {
    fn execute<M: Memory, AM: AddressingMode<M>, F: Fn(&Cpu<M>)>(_: &mut Cpu<M>, _: AM, _: &F) {
        unimplemented!()
    }

    /// An execute method that doesn't accept a tick_handler and returns the number of
    /// cycles executed.
    fn execute_cycles<M: Memory, AM: AddressingMode<M>>(cpu: &mut Cpu<M>, am: AM) -> usize {
        let cycles = Cell::new(0);
        Self::execute(cpu, am, &|_| cycles.set(cycles.get() + 1));
        cycles.get()
    }
}

pub fn execute<M: Memory, F: Fn(&Cpu<M>)>(cpu: &mut Cpu<M>, opcode: u8, tick_handler: &F) {
    match opcode {
        0xe8 => self::inx::Inx::execute(cpu, Implied, &tick_handler),
        0xca => self::dex::Dex::execute(cpu, Implied, &tick_handler),
        0xc8 => self::iny::Iny::execute(cpu, Implied, &tick_handler),
        0x88 => self::dey::Dey::execute(cpu, Implied, &tick_handler),
        0xaa => self::tax::Tax::execute(cpu, Implied, &tick_handler),
        0xa8 => self::tay::Tay::execute(cpu, Implied, &tick_handler),
        0x8a => self::txa::Txa::execute(cpu, Implied, &tick_handler),
        0x98 => self::tya::Tya::execute(cpu, Implied, &tick_handler),
        0x9a => self::txs::Txs::execute(cpu, Implied, &tick_handler),
        0xba => self::tsx::Tsx::execute(cpu, Implied, &tick_handler),
        0x18 => self::clc::Clc::execute(cpu, Implied, &tick_handler),
        0x38 => self::sec::Sec::execute(cpu, Implied, &tick_handler),
        0x58 => self::cli::Cli::execute(cpu, Implied, &tick_handler),
        0x78 => self::sei::Sei::execute(cpu, Implied, &tick_handler),
        0xb8 => self::clv::Clv::execute(cpu, Implied, &tick_handler),
        0xd8 => self::cld::Cld::execute(cpu, Implied, &tick_handler),
        0xf8 => self::sed::Sed::execute(cpu, Implied, &tick_handler),
        0x60 => self::rts::Rts::execute(cpu, Implied, &tick_handler),
        0x00 => self::brk::Brk::execute(cpu, Implied, &tick_handler),
        0x40 => self::rti::Rti::execute(cpu, Implied, &tick_handler),
        0x48 => self::pha::Pha::execute(cpu, Implied, &tick_handler),
        0x68 => self::pla::Pla::execute(cpu, Implied, &tick_handler),
        0x08 => self::php::Php::execute(cpu, Implied, &tick_handler),
        0x28 => self::plp::Plp::execute(cpu, Implied, &tick_handler),
        0xea => self::nop::Nop::execute(cpu, Implied, &tick_handler),
        0x10 => {
            let am = Relative::new(cpu, &tick_handler);
            self::bpl::Bpl::execute(cpu, am, &tick_handler)
        }
        0x30 => {
            let am = Relative::new(cpu, &tick_handler);
            self::bmi::Bmi::execute(cpu, am, &tick_handler)
        }
        0x50 => {
            let am = Relative::new(cpu, &tick_handler);
            self::bvc::Bvc::execute(cpu, am, &tick_handler)
        }
        0x70 => {
            let am = Relative::new(cpu, &tick_handler);
            self::bvs::Bvs::execute(cpu, am, &tick_handler)
        }
        0x90 => {
            let am = Relative::new(cpu, &tick_handler);
            self::bcc::Bcc::execute(cpu, am, &tick_handler)
        }
        0xb0 => {
            let am = Relative::new(cpu, &tick_handler);
            self::bcs::Bcs::execute(cpu, am, &tick_handler)
        }
        0xd0 => {
            let am = Relative::new(cpu, &tick_handler);
            self::bne::Bne::execute(cpu, am, &tick_handler)
        }
        0xf0 => {
            let am = Relative::new(cpu, &tick_handler);
            self::beq::Beq::execute(cpu, am, &tick_handler)
        }
        0xa1 => self::lda::Lda::execute(cpu, IndexedIndirect, &tick_handler),
        0xa5 => {
            let am = ZeroPage::new(cpu, &tick_handler);
            self::lda::Lda::execute(cpu, am, &tick_handler)
        }
        0xa9 => {
            let am = Immediate::new(cpu, &tick_handler);
            self::lda::Lda::execute(cpu, am, &tick_handler)
        }
        0xb1 => self::lda::Lda::execute(cpu, IndirectIndexed, &tick_handler),
        0xb5 => self::lda::Lda::execute(cpu, ZeroPageX, &tick_handler),
        0xad => {
            let am = Absolute::new(cpu, &tick_handler);
            self::lda::Lda::execute(cpu, am, &tick_handler)
        }
        0xb9 => self::lda::Lda::execute(cpu, AbsoluteY::default(), &tick_handler),
        0xbd => self::lda::Lda::execute(cpu, AbsoluteX::default(), &tick_handler),
        0xa2 => {
            let am = Immediate::new(cpu, &tick_handler);
            self::ldx::Ldx::execute(cpu, am, &tick_handler)
        }
        0xa6 => {
            let am = ZeroPage::new(cpu, &tick_handler);
            self::ldx::Ldx::execute(cpu, am, &tick_handler)
        }
        0xb6 => self::ldx::Ldx::execute(cpu, ZeroPageY, &tick_handler),
        0xae => {
            let am = Absolute::new(cpu, &tick_handler);
            self::ldx::Ldx::execute(cpu, am, &tick_handler)
        }
        0xbe => self::ldx::Ldx::execute(cpu, AbsoluteY::default(), &tick_handler),
        0xa0 => {
            let am = Immediate::new(cpu, &tick_handler);
            self::ldy::Ldy::execute(cpu, am, &tick_handler)
        }
        0xa4 => {
            let am = ZeroPage::new(cpu, &tick_handler);
            self::ldy::Ldy::execute(cpu, am, &tick_handler)
        }
        0xb4 => self::ldy::Ldy::execute(cpu, ZeroPageX, &tick_handler),
        0xac => {
            let am = Absolute::new(cpu, &tick_handler);
            self::ldy::Ldy::execute(cpu, am, &tick_handler)
        }
        0xbc => self::ldy::Ldy::execute(cpu, AbsoluteX::default(), &tick_handler),
        0x85 => {
            let am = ZeroPage::new(cpu, &tick_handler);
            self::sta::Sta::execute(cpu, am, &tick_handler)
        }
        0x95 => self::sta::Sta::execute(cpu, ZeroPageX, &tick_handler),
        0x81 => self::sta::Sta::execute(cpu, IndexedIndirect, &tick_handler),
        0x91 => self::sta::Sta::execute(cpu, IndirectIndexed, &tick_handler),
        0x8d => {
            let am = Absolute::new(cpu, &tick_handler);
            self::sta::Sta::execute(cpu, am, &tick_handler)
        }
        0x9d => self::sta::Sta::execute(cpu, AbsoluteX::default(), &tick_handler),
        0x99 => self::sta::Sta::execute(cpu, AbsoluteY::default(), &tick_handler),
        0x86 => {
            let am = ZeroPage::new(cpu, &tick_handler);
            self::stx::Stx::execute(cpu, am, &tick_handler)
        }
        0x96 => self::stx::Stx::execute(cpu, ZeroPageY, &tick_handler),
        0x8e => {
            let am = Absolute::new(cpu, &tick_handler);
            self::stx::Stx::execute(cpu, am, &tick_handler)
        }
        0x84 => {
            let am = ZeroPage::new(cpu, &tick_handler);
            self::sty::Sty::execute(cpu, am, &tick_handler)
        }
        0x94 => self::sty::Sty::execute(cpu, ZeroPageX, &tick_handler),
        0x8c => {
            let am = Absolute::new(cpu, &tick_handler);
            self::sty::Sty::execute(cpu, am, &tick_handler)
        }
        0x69 => {
            let am = Immediate::new(cpu, &tick_handler);
            self::adc::Adc::execute(cpu, am, &tick_handler)
        }
        0x65 => {
            let am = ZeroPage::new(cpu, &tick_handler);
            self::adc::Adc::execute(cpu, am, &tick_handler)
        }
        0x75 => self::adc::Adc::execute(cpu, ZeroPageX, &tick_handler),
        0x61 => self::adc::Adc::execute(cpu, IndexedIndirect, &tick_handler),
        0x71 => self::adc::Adc::execute(cpu, IndirectIndexed, &tick_handler),
        0x6d => {
            let am = Absolute::new(cpu, &tick_handler);
            self::adc::Adc::execute(cpu, am, &tick_handler)
        }
        0x7d => self::adc::Adc::execute(cpu, AbsoluteX::default(), &tick_handler),
        0x79 => self::adc::Adc::execute(cpu, AbsoluteY::default(), &tick_handler),
        0xe9 => {
            let am = Immediate::new(cpu, &tick_handler);
            self::sbc::Sbc::execute(cpu, am, &tick_handler)
        }
        0xe5 => {
            let am = ZeroPage::new(cpu, &tick_handler);
            self::sbc::Sbc::execute(cpu, am, &tick_handler)
        }
        0xf5 => self::sbc::Sbc::execute(cpu, ZeroPageX, &tick_handler),
        0xe1 => self::sbc::Sbc::execute(cpu, IndexedIndirect, &tick_handler),
        0xf1 => self::sbc::Sbc::execute(cpu, IndirectIndexed, &tick_handler),
        0xed => {
            let am = Absolute::new(cpu, &tick_handler);
            self::sbc::Sbc::execute(cpu, am, &tick_handler)
        }
        0xfd => self::sbc::Sbc::execute(cpu, AbsoluteX::default(), &tick_handler),
        0xf9 => self::sbc::Sbc::execute(cpu, AbsoluteY::default(), &tick_handler),
        0xc9 => {
            let am = Immediate::new(cpu, &tick_handler);
            self::cmp::Cmp::execute(cpu, am, &tick_handler)
        }
        0xc5 => {
            let am = ZeroPage::new(cpu, &tick_handler);
            self::cmp::Cmp::execute(cpu, am, &tick_handler)
        }
        0xd5 => self::cmp::Cmp::execute(cpu, ZeroPageX, &tick_handler),
        0xc1 => self::cmp::Cmp::execute(cpu, IndexedIndirect, &tick_handler),
        0xd1 => self::cmp::Cmp::execute(cpu, IndirectIndexed, &tick_handler),
        0xcd => {
            let am = Absolute::new(cpu, &tick_handler);
            self::cmp::Cmp::execute(cpu, am, &tick_handler)
        }
        0xdd => self::cmp::Cmp::execute(cpu, AbsoluteX::default(), &tick_handler),
        0xd9 => self::cmp::Cmp::execute(cpu, AbsoluteY::default(), &tick_handler),
        0xe0 => {
            let am = Immediate::new(cpu, &tick_handler);
            self::cpx::Cpx::execute(cpu, am, &tick_handler)
        }
        0xe4 => {
            let am = ZeroPage::new(cpu, &tick_handler);
            self::cpx::Cpx::execute(cpu, am, &tick_handler)
        }
        0xec => {
            let am = Absolute::new(cpu, &tick_handler);
            self::cpx::Cpx::execute(cpu, am, &tick_handler)
        }
        0xc0 => {
            let am = Immediate::new(cpu, &tick_handler);
            self::cpy::Cpy::execute(cpu, am, &tick_handler)
        }
        0xc4 => {
            let am = ZeroPage::new(cpu, &tick_handler);
            self::cpy::Cpy::execute(cpu, am, &tick_handler)
        }
        0xcc => {
            let am = Absolute::new(cpu, &tick_handler);
            self::cpy::Cpy::execute(cpu, am, &tick_handler)
        }
        0x29 => {
            let am = Immediate::new(cpu, &tick_handler);
            self::and::And::execute(cpu, am, &tick_handler)
        }
        0x25 => {
            let am = ZeroPage::new(cpu, &tick_handler);
            self::and::And::execute(cpu, am, &tick_handler)
        }
        0x35 => self::and::And::execute(cpu, ZeroPageX, &tick_handler),
        0x21 => self::and::And::execute(cpu, IndexedIndirect, &tick_handler),
        0x31 => self::and::And::execute(cpu, IndirectIndexed, &tick_handler),
        0x2d => {
            let am = Absolute::new(cpu, &tick_handler);
            self::and::And::execute(cpu, am, &tick_handler)
        }
        0x3d => self::and::And::execute(cpu, AbsoluteX::default(), &tick_handler),
        0x39 => self::and::And::execute(cpu, AbsoluteY::default(), &tick_handler),
        0x09 => {
            let am = Immediate::new(cpu, &tick_handler);
            self::ora::Ora::execute(cpu, am, &tick_handler)
        }
        0x05 => {
            let am = ZeroPage::new(cpu, &tick_handler);
            self::ora::Ora::execute(cpu, am, &tick_handler)
        }
        0x15 => self::ora::Ora::execute(cpu, ZeroPageX, &tick_handler),
        0x01 => self::ora::Ora::execute(cpu, IndexedIndirect, &tick_handler),
        0x11 => self::ora::Ora::execute(cpu, IndirectIndexed, &tick_handler),
        0x0d => {
            let am = Absolute::new(cpu, &tick_handler);
            self::ora::Ora::execute(cpu, am, &tick_handler)
        }
        0x1d => self::ora::Ora::execute(cpu, AbsoluteX::default(), &tick_handler),
        0x19 => self::ora::Ora::execute(cpu, AbsoluteY::default(), &tick_handler),
        0x49 => {
            let am = Immediate::new(cpu, &tick_handler);
            self::eor::Eor::execute(cpu, am, &tick_handler)
        }
        0x45 => {
            let am = ZeroPage::new(cpu, &tick_handler);
            self::eor::Eor::execute(cpu, am, &tick_handler)
        }
        0x55 => self::eor::Eor::execute(cpu, ZeroPageX, &tick_handler),
        0x41 => self::eor::Eor::execute(cpu, IndexedIndirect, &tick_handler),
        0x51 => self::eor::Eor::execute(cpu, IndirectIndexed, &tick_handler),
        0x4d => {
            let am = Absolute::new(cpu, &tick_handler);
            self::eor::Eor::execute(cpu, am, &tick_handler)
        }
        0x5d => self::eor::Eor::execute(cpu, AbsoluteX::default(), &tick_handler),
        0x59 => self::eor::Eor::execute(cpu, AbsoluteY::default(), &tick_handler),
        0x24 => {
            let am = ZeroPage::new(cpu, &tick_handler);
            self::bit::Bit::execute(cpu, am, &tick_handler)
        }
        0x2c => {
            let am = Absolute::new(cpu, &tick_handler);
            self::bit::Bit::execute(cpu, am, &tick_handler)
        }
        0x2a => self::rol::Rol::execute(cpu, Accumulator, &tick_handler),
        0x26 => {
            let am = ZeroPage::new(cpu, &tick_handler);
            self::rol::Rol::execute(cpu, am, &tick_handler)
        }
        0x36 => self::rol::Rol::execute(cpu, ZeroPageX, &tick_handler),
        0x2e => {
            let am = Absolute::new(cpu, &tick_handler);
            self::rol::Rol::execute(cpu, am, &tick_handler)
        }
        0x3e => self::rol::Rol::execute(cpu, AbsoluteX::default(), &tick_handler),
        0x6a => self::ror::Ror::execute(cpu, Accumulator, &tick_handler),
        0x66 => {
            let am = ZeroPage::new(cpu, &tick_handler);
            self::ror::Ror::execute(cpu, am, &tick_handler)
        }
        0x76 => self::ror::Ror::execute(cpu, ZeroPageX, &tick_handler),
        0x6e => {
            let am = Absolute::new(cpu, &tick_handler);
            self::ror::Ror::execute(cpu, am, &tick_handler)
        }
        0x7e => self::ror::Ror::execute(cpu, AbsoluteX::default(), &tick_handler),
        0x0a => self::asl::Asl::execute(cpu, Accumulator, &tick_handler),
        0x06 => {
            let am = ZeroPage::new(cpu, &tick_handler);
            self::asl::Asl::execute(cpu, am, &tick_handler)
        }
        0x16 => self::asl::Asl::execute(cpu, ZeroPageX, &tick_handler),
        0x0e => {
            let am = Absolute::new(cpu, &tick_handler);
            self::asl::Asl::execute(cpu, am, &tick_handler)
        }
        0x1e => self::asl::Asl::execute(cpu, AbsoluteX::default(), &tick_handler),
        0x4a => self::lsr::Lsr::execute(cpu, Accumulator, &tick_handler),
        0x46 => {
            let am = ZeroPage::new(cpu, &tick_handler);
            self::lsr::Lsr::execute(cpu, am, &tick_handler)
        }
        0x56 => self::lsr::Lsr::execute(cpu, ZeroPageX, &tick_handler),
        0x4e => {
            let am = Absolute::new(cpu, &tick_handler);
            self::lsr::Lsr::execute(cpu, am, &tick_handler)
        }
        0x5e => self::lsr::Lsr::execute(cpu, AbsoluteX::default(), &tick_handler),
        0xe6 => {
            let am = ZeroPage::new(cpu, &tick_handler);
            self::inc::Inc::execute(cpu, am, &tick_handler)
        }
        0xf6 => self::inc::Inc::execute(cpu, ZeroPageX, &tick_handler),
        0xee => {
            let am = Absolute::new(cpu, &tick_handler);
            self::inc::Inc::execute(cpu, am, &tick_handler)
        }
        0xfe => self::inc::Inc::execute(cpu, AbsoluteX::default(), &tick_handler),
        0xc6 => {
            let am = ZeroPage::new(cpu, &tick_handler);
            self::dec::Dec::execute(cpu, am, &tick_handler)
        }
        0xd6 => self::dec::Dec::execute(cpu, ZeroPageX, &tick_handler),
        0xce => {
            let am = Absolute::new(cpu, &tick_handler);
            self::dec::Dec::execute(cpu, am, &tick_handler)
        }
        0xde => self::dec::Dec::execute(cpu, AbsoluteX::default(), &tick_handler),
        0x4c => {
            let am = Absolute::new(cpu, &tick_handler);
            self::jmp::Jmp::execute(cpu, am, &tick_handler)
        }
        0x6c => self::jmp::Jmp::execute(cpu, Indirect, &tick_handler),
        0x20 => {
            let am = Absolute::new(cpu, &tick_handler);
            self::jsr::Jsr::execute(cpu, am, &tick_handler)
        }
        _ => {
            panic!("unexpected opcode encountered");
        }
    }
}
