mod addressing_mode;
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

use cpu::Cpu;
use memory::Memory;

use self::addressing_mode::*;

pub trait Instruction {
    fn execute<M: Memory, EC: AddressingMode<M>, F: Fn(&Cpu<M>)>(cpu: &mut Cpu<M>,
                                                                 context: EC,
                                                                 tick_handler: F);
}

pub fn execute<M: Memory, F: Fn(&Cpu<M>)>(cpu: &mut Cpu<M>, opcode: u8, tick_handler: F) {
    match opcode {
        0xe8 => self::inx::Inx::execute(cpu, Implied, tick_handler),
        0xca => self::dex::Dex::execute(cpu, Implied, tick_handler),
        0xc8 => self::iny::Iny::execute(cpu, Implied, tick_handler),
        0x88 => self::dey::Dey::execute(cpu, Implied, tick_handler),
        0xaa => self::tax::Tax::execute(cpu, Implied, tick_handler),
        0xa8 => self::tay::Tay::execute(cpu, Implied, tick_handler),
        0x8a => self::txa::Txa::execute(cpu, Implied, tick_handler),
        0x98 => self::tya::Tya::execute(cpu, Implied, tick_handler),
        0x9a => self::txs::Txs::execute(cpu, Implied, tick_handler),
        0xba => self::tsx::Tsx::execute(cpu, Implied, tick_handler),
        0x18 => self::clc::Clc::execute(cpu, Implied, tick_handler),
        0x38 => self::sec::Sec::execute(cpu, Implied, tick_handler),
        0x58 => self::cli::Cli::execute(cpu, Implied, tick_handler),
        0x78 => self::sei::Sei::execute(cpu, Implied, tick_handler),
        0xb8 => self::clv::Clv::execute(cpu, Implied, tick_handler),
        0xd8 => self::cld::Cld::execute(cpu, Implied, tick_handler),
        0xf8 => self::sed::Sed::execute(cpu, Implied, tick_handler),
        0x60 => self::rts::Rts::execute(cpu, Implied, tick_handler),
        0x00 => self::brk::Brk::execute(cpu, Implied, tick_handler),
        0x40 => self::rti::Rti::execute(cpu, Implied, tick_handler),
        0x48 => self::pha::Pha::execute(cpu, Implied, tick_handler),
        0x68 => self::pla::Pla::execute(cpu, Implied, tick_handler),
        0x08 => self::php::Php::execute(cpu, Implied, tick_handler),
        0x28 => self::plp::Plp::execute(cpu, Implied, tick_handler),
        0xea => self::nop::Nop::execute(cpu, Implied, tick_handler),
        0x10 => self::bpl::Bpl::execute(cpu, Relative, tick_handler),
        0x30 => self::bmi::Bmi::execute(cpu, Relative, tick_handler),
        0x50 => self::bvc::Bvc::execute(cpu, Relative, tick_handler),
        0x70 => self::bvs::Bvs::execute(cpu, Relative, tick_handler),
        0x90 => self::bcc::Bcc::execute(cpu, Relative, tick_handler),
        0xb0 => self::bcs::Bcs::execute(cpu, Relative, tick_handler),
        0xd0 => self::bne::Bne::execute(cpu, Relative, tick_handler),
        0xf0 => self::beq::Beq::execute(cpu, Relative, tick_handler),
        0xa1 => self::lda::Lda::execute(cpu, IndexedIndirect, tick_handler),
        0xa5 => self::lda::Lda::execute(cpu, ZeroPage, tick_handler),
        0xa9 => self::lda::Lda::execute(cpu, Immediate, tick_handler),
        0xb1 => self::lda::Lda::execute(cpu, IndirectIndexed, tick_handler),
        0xb5 => self::lda::Lda::execute(cpu, ZeroPageX, tick_handler),
        0xad => self::lda::Lda::execute(cpu, Absolute::default(), tick_handler),
        0xb9 => self::lda::Lda::execute(cpu, AbsoluteY::default(), tick_handler),
        0xbd => self::lda::Lda::execute(cpu, AbsoluteX::default(), tick_handler),
        0xa2 => self::ldx::Ldx::execute(cpu, Immediate, tick_handler),
        0xa6 => self::ldx::Ldx::execute(cpu, ZeroPage, tick_handler),
        0xb6 => self::ldx::Ldx::execute(cpu, ZeroPageY, tick_handler),
        0xae => self::ldx::Ldx::execute(cpu, Absolute::default(), tick_handler),
        0xbe => self::ldx::Ldx::execute(cpu, AbsoluteY::default(), tick_handler),
        0xa0 => self::ldy::Ldy::execute(cpu, Immediate, tick_handler),
        0xa4 => self::ldy::Ldy::execute(cpu, ZeroPage, tick_handler),
        0xb4 => self::ldy::Ldy::execute(cpu, ZeroPageX, tick_handler),
        0xac => self::ldy::Ldy::execute(cpu, Absolute::default(), tick_handler),
        0xbc => self::ldy::Ldy::execute(cpu, AbsoluteX::default(), tick_handler),
        0x85 => self::sta::Sta::execute(cpu, ZeroPage, tick_handler),
        0x95 => self::sta::Sta::execute(cpu, ZeroPageX, tick_handler),
        0x81 => self::sta::Sta::execute(cpu, IndexedIndirect, tick_handler),
        0x91 => self::sta::Sta::execute(cpu, IndirectIndexed, tick_handler),
        0x8d => self::sta::Sta::execute(cpu, Absolute::default(), tick_handler),
        0x9d => self::sta::Sta::execute(cpu, AbsoluteX::default(), tick_handler),
        0x99 => self::sta::Sta::execute(cpu, AbsoluteY::default(), tick_handler),
        0x86 => self::stx::Stx::execute(cpu, ZeroPage, tick_handler),
        0x96 => self::stx::Stx::execute(cpu, ZeroPageY, tick_handler),
        0x8e => self::stx::Stx::execute(cpu, Absolute::default(), tick_handler),
        0x84 => self::sty::Sty::execute(cpu, ZeroPage, tick_handler),
        0x94 => self::sty::Sty::execute(cpu, ZeroPageX, tick_handler),
        0x8c => self::sty::Sty::execute(cpu, Absolute::default(), tick_handler),
        0x69 => self::adc::Adc::execute(cpu, Immediate, tick_handler),
        0x65 => self::adc::Adc::execute(cpu, ZeroPage, tick_handler),
        0x75 => self::adc::Adc::execute(cpu, ZeroPageX, tick_handler),
        0x61 => self::adc::Adc::execute(cpu, IndexedIndirect, tick_handler),
        0x71 => self::adc::Adc::execute(cpu, IndirectIndexed, tick_handler),
        0x6d => self::adc::Adc::execute(cpu, Absolute::default(), tick_handler),
        0x7d => self::adc::Adc::execute(cpu, AbsoluteX::default(), tick_handler),
        0x79 => self::adc::Adc::execute(cpu, AbsoluteY::default(), tick_handler),
        0xe9 => self::sbc::Sbc::execute(cpu, Immediate, tick_handler),
        0xe5 => self::sbc::Sbc::execute(cpu, ZeroPage, tick_handler),
        0xf5 => self::sbc::Sbc::execute(cpu, ZeroPageX, tick_handler),
        0xe1 => self::sbc::Sbc::execute(cpu, IndexedIndirect, tick_handler),
        0xf1 => self::sbc::Sbc::execute(cpu, IndirectIndexed, tick_handler),
        0xed => self::sbc::Sbc::execute(cpu, Absolute::default(), tick_handler),
        0xfd => self::sbc::Sbc::execute(cpu, AbsoluteX::default(), tick_handler),
        0xf9 => self::sbc::Sbc::execute(cpu, AbsoluteY::default(), tick_handler),
        0xc9 => self::cmp::Cmp::execute(cpu, Immediate, tick_handler),
        0xc5 => self::cmp::Cmp::execute(cpu, ZeroPage, tick_handler),
        0xd5 => self::cmp::Cmp::execute(cpu, ZeroPageX, tick_handler),
        0xc1 => self::cmp::Cmp::execute(cpu, IndexedIndirect, tick_handler),
        0xd1 => self::cmp::Cmp::execute(cpu, IndirectIndexed, tick_handler),
        0xcd => self::cmp::Cmp::execute(cpu, Absolute::default(), tick_handler),
        0xdd => self::cmp::Cmp::execute(cpu, AbsoluteX::default(), tick_handler),
        0xd9 => self::cmp::Cmp::execute(cpu, AbsoluteY::default(), tick_handler),
        0xe0 => self::cpx::Cpx::execute(cpu, Immediate, tick_handler),
        0xe4 => self::cpx::Cpx::execute(cpu, ZeroPage, tick_handler),
        0xec => self::cpx::Cpx::execute(cpu, Absolute::default(), tick_handler),
        0xc0 => self::cpy::Cpy::execute(cpu, Immediate, tick_handler),
        0xc4 => self::cpy::Cpy::execute(cpu, ZeroPage, tick_handler),
        0xcc => self::cpy::Cpy::execute(cpu, Absolute::default(), tick_handler),
        0x29 => self::and::And::execute(cpu, Immediate, tick_handler),
        0x25 => self::and::And::execute(cpu, ZeroPage, tick_handler),
        0x35 => self::and::And::execute(cpu, ZeroPageX, tick_handler),
        0x21 => self::and::And::execute(cpu, IndexedIndirect, tick_handler),
        0x31 => self::and::And::execute(cpu, IndirectIndexed, tick_handler),
        0x2d => self::and::And::execute(cpu, Absolute::default(), tick_handler),
        0x3d => self::and::And::execute(cpu, AbsoluteX::default(), tick_handler),
        0x39 => self::and::And::execute(cpu, AbsoluteY::default(), tick_handler),
        0x09 => self::ora::Ora::execute(cpu, Immediate, tick_handler),
        0x05 => self::ora::Ora::execute(cpu, ZeroPage, tick_handler),
        0x15 => self::ora::Ora::execute(cpu, ZeroPageX, tick_handler),
        0x01 => self::ora::Ora::execute(cpu, IndexedIndirect, tick_handler),
        0x11 => self::ora::Ora::execute(cpu, IndirectIndexed, tick_handler),
        0x0d => self::ora::Ora::execute(cpu, Absolute::default(), tick_handler),
        0x1d => self::ora::Ora::execute(cpu, AbsoluteX::default(), tick_handler),
        0x19 => self::ora::Ora::execute(cpu, AbsoluteY::default(), tick_handler),
        0x49 => self::eor::Eor::execute(cpu, Immediate, tick_handler),
        0x45 => self::eor::Eor::execute(cpu, ZeroPage, tick_handler),
        0x55 => self::eor::Eor::execute(cpu, ZeroPageX, tick_handler),
        0x41 => self::eor::Eor::execute(cpu, IndexedIndirect, tick_handler),
        0x51 => self::eor::Eor::execute(cpu, IndirectIndexed, tick_handler),
        0x4d => self::eor::Eor::execute(cpu, Absolute::default(), tick_handler),
        0x5d => self::eor::Eor::execute(cpu, AbsoluteX::default(), tick_handler),
        0x59 => self::eor::Eor::execute(cpu, AbsoluteY::default(), tick_handler),
        0x24 => self::bit::Bit::execute(cpu, ZeroPage, tick_handler),
        0x2c => self::bit::Bit::execute(cpu, Absolute::default(), tick_handler),
        0x2a => self::rol::Rol::execute(cpu, Accumulator, tick_handler),
        0x26 => self::rol::Rol::execute(cpu, ZeroPage, tick_handler),
        0x36 => self::rol::Rol::execute(cpu, ZeroPageX, tick_handler),
        0x2e => self::rol::Rol::execute(cpu, Absolute::default(), tick_handler),
        0x3e => self::rol::Rol::execute(cpu, AbsoluteX::default(), tick_handler),
        0x6a => self::ror::Ror::execute(cpu, Accumulator, tick_handler),
        0x66 => self::ror::Ror::execute(cpu, ZeroPage, tick_handler),
        0x76 => self::ror::Ror::execute(cpu, ZeroPageX, tick_handler),
        0x6e => self::ror::Ror::execute(cpu, Absolute::default(), tick_handler),
        0x7e => self::ror::Ror::execute(cpu, AbsoluteX::default(), tick_handler),
        0x0a => self::asl::Asl::execute(cpu, Accumulator, tick_handler),
        0x06 => self::asl::Asl::execute(cpu, ZeroPage, tick_handler),
        0x16 => self::asl::Asl::execute(cpu, ZeroPageX, tick_handler),
        0x0e => self::asl::Asl::execute(cpu, Absolute::default(), tick_handler),
        0x1e => self::asl::Asl::execute(cpu, AbsoluteX::default(), tick_handler),
        0x4a => self::lsr::Lsr::execute(cpu, Accumulator, tick_handler),
        0x46 => self::lsr::Lsr::execute(cpu, ZeroPage, tick_handler),
        0x56 => self::lsr::Lsr::execute(cpu, ZeroPageX, tick_handler),
        0x4e => self::lsr::Lsr::execute(cpu, Absolute::default(), tick_handler),
        0x5e => self::lsr::Lsr::execute(cpu, AbsoluteX::default(), tick_handler),
        0xe6 => self::inc::Inc::execute(cpu, ZeroPage, tick_handler),
        0xf6 => self::inc::Inc::execute(cpu, ZeroPageX, tick_handler),
        0xee => self::inc::Inc::execute(cpu, Absolute::default(), tick_handler),
        0xfe => self::inc::Inc::execute(cpu, AbsoluteX::default(), tick_handler),
        0xc6 => self::dec::Dec::execute(cpu, ZeroPage, tick_handler),
        0xd6 => self::dec::Dec::execute(cpu, ZeroPageX, tick_handler),
        0xce => self::dec::Dec::execute(cpu, Absolute::default(), tick_handler),
        0xde => self::dec::Dec::execute(cpu, AbsoluteX::default(), tick_handler),
        0x4c => self::jmp::Jmp::execute(cpu, Absolute::default(), tick_handler),
        0x6c => self::jmp::Jmp::execute(cpu, Indirect, tick_handler),
        0x20 => self::jsr::Jsr::execute(cpu, Absolute::default(), tick_handler),
        _ => {
            panic!("unexpected opcode encountered");
        }
    }
}
