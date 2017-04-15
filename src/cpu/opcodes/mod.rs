#[cfg(test)]
pub mod branch_tests_base;

#[cfg(test)]
pub mod compare_tests_base;

#[cfg(test)]
pub mod inc_dec_tests_base;

#[cfg(test)]
pub mod shift_tests_base;

#[cfg(test)]
pub mod adc_spec_tests;

#[cfg(test)]
pub mod sbc_spec_tests;

mod addressing;
mod branch_base;
mod shift_base;
mod compare_base;
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
use cpu::opcodes::addressing::*;
use memory::Memory;
use screen::Screen;

trait OpCode {
    type Input;
    fn execute<S: Screen, M: Memory<S>, AM: AddressingMode<S, M, Output = Self::Input>>(cpu: &mut Cpu<S, M>, am: AM);
}

pub fn execute<S: Screen, M: Memory<S>>(cpu: &mut Cpu<S, M>, opcode: u8) {
    match opcode {
        0xe8 => self::inx::Inx::execute(cpu, Implied),
        0xca => self::dex::Dex::execute(cpu, Implied),
        0xc8 => self::iny::Iny::execute(cpu, Implied),
        0x88 => self::dey::Dey::execute(cpu, Implied),
        0xaa => self::tax::Tax::execute(cpu, Implied),
        0xa8 => self::tay::Tay::execute(cpu, Implied),
        0x8a => self::txa::Txa::execute(cpu, Implied),
        0x98 => self::tya::Tya::execute(cpu, Implied),
        0x9a => self::txs::Txs::execute(cpu, Implied),
        0xba => self::tsx::Tsx::execute(cpu, Implied),
        0x18 => self::clc::Clc::execute(cpu, Implied),
        0x38 => self::sec::Sec::execute(cpu, Implied),
        0x58 => self::cli::Cli::execute(cpu, Implied),
        0x78 => self::sei::Sei::execute(cpu, Implied),
        0xb8 => self::clv::Clv::execute(cpu, Implied),
        0xd8 => self::cld::Cld::execute(cpu, Implied),
        0xf8 => self::sed::Sed::execute(cpu, Implied),
        0x60 => self::rts::Rts::execute(cpu, Implied),
        0x00 => self::brk::Brk::execute(cpu, Implied),
        0x40 => self::rti::Rti::execute(cpu, Implied),
        0x48 => self::pha::Pha::execute(cpu, Implied),
        0x68 => self::pla::Pla::execute(cpu, Implied),
        0x08 => self::php::Php::execute(cpu, Implied),
        0x28 => self::plp::Plp::execute(cpu, Implied),
        0xea => self::nop::Nop::execute(cpu, Implied),
        0x10 => {
            let am = Relative::init(cpu);
            self::bpl::Bpl::execute(cpu, am)
        }
        0x30 => {
            let am = Relative::init(cpu);
            self::bmi::Bmi::execute(cpu, am)
        }
        0x50 => {
            let am = Relative::init(cpu);
            self::bvc::Bvc::execute(cpu, am)
        }
        0x70 => {
            let am = Relative::init(cpu);
            self::bvs::Bvs::execute(cpu, am)
        }
        0x90 => {
            let am = Relative::init(cpu);
            self::bcc::Bcc::execute(cpu, am)
        }
        0xb0 => {
            let am = Relative::init(cpu);
            self::bcs::Bcs::execute(cpu, am)
        }
        0xd0 => {
            let am = Relative::init(cpu);
            self::bne::Bne::execute(cpu, am)
        }
        0xf0 => {
            let am = Relative::init(cpu);
            self::beq::Beq::execute(cpu, am)
        }
        0xa1 => {
            let am = IndexedIndirect::init(cpu);
            self::lda::Lda::execute(cpu, am)
        }
        0xa5 => {
            let am = ZeroPage::init(cpu);
            self::lda::Lda::execute(cpu, am)
        }
        0xa9 => {
            let am = Immediate::init(cpu);
            self::lda::Lda::execute(cpu, am)
        }
        0xb1 => {
            let am = IndirectIndexed::init(cpu);
            self::lda::Lda::execute(cpu, am)
        }
        0xb5 => {
            let am = ZeroPageX::init(cpu);
            self::lda::Lda::execute(cpu, am)
        }
        0xad => {
            let am = Absolute::init(cpu);
            self::lda::Lda::execute(cpu, am)
        }
        0xb9 => {
            let am = AbsoluteY::init(cpu);
            self::lda::Lda::execute(cpu, am)
        }
        0xbd => {
            let am = AbsoluteX::init(cpu);
            self::lda::Lda::execute(cpu, am)
        }
        0xa2 => {
            let am = Immediate::init(cpu);
            self::ldx::Ldx::execute(cpu, am)
        }
        0xa6 => {
            let am = ZeroPage::init(cpu);
            self::ldx::Ldx::execute(cpu, am)
        }
        0xb6 => {
            let am = ZeroPageY::init(cpu);
            self::ldx::Ldx::execute(cpu, am)
        }
        0xae => {
            let am = Absolute::init(cpu);
            self::ldx::Ldx::execute(cpu, am)
        }
        0xbe => {
            let am = AbsoluteY::init(cpu);
            self::ldx::Ldx::execute(cpu, am)
        }
        0xa0 => {
            let am = Immediate::init(cpu);
            self::ldy::Ldy::execute(cpu, am)
        }
        0xa4 => {
            let am = ZeroPage::init(cpu);
            self::ldy::Ldy::execute(cpu, am)
        }
        0xb4 => {
            let am = ZeroPageX::init(cpu);
            self::ldy::Ldy::execute(cpu, am)
        }
        0xac => {
            let am = Absolute::init(cpu);
            self::ldy::Ldy::execute(cpu, am)
        }
        0xbc => {
            let am = AbsoluteX::init(cpu);
            self::ldy::Ldy::execute(cpu, am)
        }
        0x85 => {
            let am = ZeroPage::init_store(cpu);
            self::sta::Sta::execute(cpu, am)
        }
        0x95 => {
            let am = ZeroPageX::init_store(cpu);
            self::sta::Sta::execute(cpu, am)
        }
        0x81 => {
            let am = IndexedIndirect::init_store(cpu);
            self::sta::Sta::execute(cpu, am)
        }
        0x91 => {
            let am = IndirectIndexed::init_store(cpu);
            self::sta::Sta::execute(cpu, am)
        }
        0x8d => {
            let am = Absolute::init_store(cpu);
            self::sta::Sta::execute(cpu, am)
        }
        0x9d => {
            let am = AbsoluteX::init_store(cpu);
            self::sta::Sta::execute(cpu, am)
        }
        0x99 => {
            let am = AbsoluteY::init_store(cpu);
            self::sta::Sta::execute(cpu, am)
        }
        0x86 => {
            let am = ZeroPage::init_store(cpu);
            self::stx::Stx::execute(cpu, am)
        }
        0x96 => {
            let am = ZeroPageY::init_store(cpu);
            self::stx::Stx::execute(cpu, am)
        }
        0x8e => {
            let am = Absolute::init_store(cpu);
            self::stx::Stx::execute(cpu, am)
        }
        0x84 => {
            let am = ZeroPage::init_store(cpu);
            self::sty::Sty::execute(cpu, am)
        }
        0x94 => {
            let am = ZeroPageX::init_store(cpu);
            self::sty::Sty::execute(cpu, am)
        }
        0x8c => {
            let am = Absolute::init_store(cpu);
            self::sty::Sty::execute(cpu, am)
        }
        0x69 => {
            let am = Immediate::init(cpu);
            Adc::execute(cpu, am)
        }
        0x65 => {
            let am = ZeroPage::init(cpu);
            Adc::execute(cpu, am)
        }
        0x75 => {
            let am = ZeroPageX::init(cpu);
            Adc::execute(cpu, am)
        }
        0x61 => {
            let am = IndexedIndirect::init(cpu);
            Adc::execute(cpu, am)
        }
        0x71 => {
            let am = IndirectIndexed::init(cpu);
            Adc::execute(cpu, am)
        }
        0x6d => {
            let am = Absolute::init(cpu);
            Adc::execute(cpu, am)
        }
        0x7d => {
            let am = AbsoluteX::init(cpu);
            Adc::execute(cpu, am)
        }
        0x79 => {
            let am = AbsoluteY::init(cpu);
            Adc::execute(cpu, am)
        }
        0xe9 => {
            let am = Immediate::init(cpu);
            Sbc::execute(cpu, am)
        }
        0xe5 => {
            let am = ZeroPage::init(cpu);
            Sbc::execute(cpu, am)
        }
        0xf5 => {
            let am = ZeroPageX::init(cpu);
            Sbc::execute(cpu, am)
        }
        0xe1 => {
            let am = IndexedIndirect::init(cpu);
            Sbc::execute(cpu, am)
        }
        0xf1 => {
            let am = IndirectIndexed::init(cpu);
            Sbc::execute(cpu, am)
        }
        0xed => {
            let am = Absolute::init(cpu);
            Sbc::execute(cpu, am)
        }
        0xfd => {
            let am = AbsoluteX::init(cpu);
            Sbc::execute(cpu, am)
        }
        0xf9 => {
            let am = AbsoluteY::init(cpu);
            Sbc::execute(cpu, am)
        }
        0xc9 => {
            let am = Immediate::init(cpu);
            self::cmp::Cmp::execute(cpu, am)
        }
        0xc5 => {
            let am = ZeroPage::init(cpu);
            self::cmp::Cmp::execute(cpu, am)
        }
        0xd5 => {
            let am = ZeroPageX::init(cpu);
            self::cmp::Cmp::execute(cpu, am)
        }
        0xc1 => {
            let am = IndexedIndirect::init(cpu);
            self::cmp::Cmp::execute(cpu, am)
        }
        0xd1 => {
            let am = IndirectIndexed::init(cpu);
            self::cmp::Cmp::execute(cpu, am)
        }
        0xcd => {
            let am = Absolute::init(cpu);
            self::cmp::Cmp::execute(cpu, am)
        }
        0xdd => {
            let am = AbsoluteX::init(cpu);
            self::cmp::Cmp::execute(cpu, am)
        }
        0xd9 => {
            let am = AbsoluteY::init(cpu);
            self::cmp::Cmp::execute(cpu, am)
        }
        0xe0 => {
            let am = Immediate::init(cpu);
            self::cpx::Cpx::execute(cpu, am)
        }
        0xe4 => {
            let am = ZeroPage::init(cpu);
            self::cpx::Cpx::execute(cpu, am)
        }
        0xec => {
            let am = Absolute::init(cpu);
            self::cpx::Cpx::execute(cpu, am)
        }
        0xc0 => {
            let am = Immediate::init(cpu);
            self::cpy::Cpy::execute(cpu, am)
        }
        0xc4 => {
            let am = ZeroPage::init(cpu);
            self::cpy::Cpy::execute(cpu, am)
        }
        0xcc => {
            let am = Absolute::init(cpu);
            self::cpy::Cpy::execute(cpu, am)
        }
        0x29 => {
            let am = Immediate::init(cpu);
            self::and::And::execute(cpu, am)
        }
        0x25 => {
            let am = ZeroPage::init(cpu);
            self::and::And::execute(cpu, am)
        }
        0x35 => {
            let am = ZeroPageX::init(cpu);
            self::and::And::execute(cpu, am)
        }
        0x21 => {
            let am = IndexedIndirect::init(cpu);
            self::and::And::execute(cpu, am)
        }
        0x31 => {
            let am = IndirectIndexed::init(cpu);
            self::and::And::execute(cpu, am)
        }
        0x2d => {
            let am = Absolute::init(cpu);
            self::and::And::execute(cpu, am)
        }
        0x3d => {
            let am = AbsoluteX::init(cpu);
            self::and::And::execute(cpu, am)
        }
        0x39 => {
            let am = AbsoluteY::init(cpu);
            self::and::And::execute(cpu, am)
        }
        0x09 => {
            let am = Immediate::init(cpu);
            self::ora::Ora::execute(cpu, am)
        }
        0x05 => {
            let am = ZeroPage::init(cpu);
            self::ora::Ora::execute(cpu, am)
        }
        0x15 => {
            let am = ZeroPageX::init(cpu);
            self::ora::Ora::execute(cpu, am)
        }
        0x01 => {
            let am = IndexedIndirect::init(cpu);
            self::ora::Ora::execute(cpu, am)
        }
        0x11 => {
            let am = IndirectIndexed::init(cpu);
            self::ora::Ora::execute(cpu, am)
        }
        0x0d => {
            let am = Absolute::init(cpu);
            self::ora::Ora::execute(cpu, am)
        }
        0x1d => {
            let am = AbsoluteX::init(cpu);
            self::ora::Ora::execute(cpu, am)
        }
        0x19 => {
            let am = AbsoluteY::init(cpu);
            self::ora::Ora::execute(cpu, am)
        }
        0x49 => {
            let am = Immediate::init(cpu);
            self::eor::Eor::execute(cpu, am)
        }
        0x45 => {
            let am = ZeroPage::init(cpu);
            self::eor::Eor::execute(cpu, am)
        }
        0x55 => {
            let am = ZeroPageX::init(cpu);
            self::eor::Eor::execute(cpu, am)
        }
        0x41 => {
            let am = IndexedIndirect::init(cpu);
            self::eor::Eor::execute(cpu, am)
        }
        0x51 => {
            let am = IndirectIndexed::init(cpu);
            self::eor::Eor::execute(cpu, am)
        }
        0x4d => {
            let am = Absolute::init(cpu);
            self::eor::Eor::execute(cpu, am)
        }
        0x5d => {
            let am = AbsoluteX::init(cpu);
            self::eor::Eor::execute(cpu, am)
        }
        0x59 => {
            let am = AbsoluteY::init(cpu);
            self::eor::Eor::execute(cpu, am)
        }
        0x24 => {
            let am = ZeroPage::init(cpu);
            self::bit::Bit::execute(cpu, am)
        }
        0x2c => {
            let am = Absolute::init(cpu);
            self::bit::Bit::execute(cpu, am)
        }
        0x2a => {
            let am = Accumulator::init(cpu);
            self::rol::Rol::execute(cpu, am)
        }
        0x26 => {
            let am = ZeroPage::init(cpu);
            self::rol::Rol::execute(cpu, am)
        }
        0x36 => {
            let am = ZeroPageX::init(cpu);
            self::rol::Rol::execute(cpu, am)
        }
        0x2e => {
            let am = Absolute::init(cpu);
            self::rol::Rol::execute(cpu, am)
        }
        0x3e => {
            let am = AbsoluteX::init_rmw(cpu);
            self::rol::Rol::execute(cpu, am)
        }
        0x6a => {
            let am = Accumulator::init(cpu);
            self::ror::Ror::execute(cpu, am)
        }
        0x66 => {
            let am = ZeroPage::init(cpu);
            self::ror::Ror::execute(cpu, am)
        }
        0x76 => {
            let am = ZeroPageX::init(cpu);
            self::ror::Ror::execute(cpu, am)
        }
        0x6e => {
            let am = Absolute::init(cpu);
            self::ror::Ror::execute(cpu, am)
        }
        0x7e => {
            let am = AbsoluteX::init_rmw(cpu);
            self::ror::Ror::execute(cpu, am)
        }
        0x0a => {
            let am = Accumulator::init(cpu);
            self::asl::Asl::execute(cpu, am)
        }
        0x06 => {
            let am = ZeroPage::init(cpu);
            self::asl::Asl::execute(cpu, am)
        }
        0x16 => {
            let am = ZeroPageX::init(cpu);
            self::asl::Asl::execute(cpu, am)
        }
        0x0e => {
            let am = Absolute::init(cpu);
            self::asl::Asl::execute(cpu, am)
        }
        0x1e => {
            let am = AbsoluteX::init_rmw(cpu);
            self::asl::Asl::execute(cpu, am)
        }
        0x4a => {
            let am = Accumulator::init(cpu);
            self::lsr::Lsr::execute(cpu, am)
        }
        0x46 => {
            let am = ZeroPage::init(cpu);
            self::lsr::Lsr::execute(cpu, am)
        }
        0x56 => {
            let am = ZeroPageX::init(cpu);
            self::lsr::Lsr::execute(cpu, am)
        }
        0x4e => {
            let am = Absolute::init(cpu);
            self::lsr::Lsr::execute(cpu, am)
        }
        0x5e => {
            let am = AbsoluteX::init_rmw(cpu);
            self::lsr::Lsr::execute(cpu, am)
        }
        0xe6 => {
            let am = ZeroPage::init(cpu);
            self::inc::Inc::execute(cpu, am)
        }
        0xf6 => {
            let am = ZeroPageX::init(cpu);
            self::inc::Inc::execute(cpu, am)
        }
        0xee => {
            let am = Absolute::init(cpu);
            self::inc::Inc::execute(cpu, am)
        }
        0xfe => {
            let am = AbsoluteX::init_rmw(cpu);
            self::inc::Inc::execute(cpu, am)
        }
        0xc6 => {
            let am = ZeroPage::init(cpu);
            self::dec::Dec::execute(cpu, am)
        }
        0xd6 => {
            let am = ZeroPageX::init(cpu);
            self::dec::Dec::execute(cpu, am)
        }
        0xce => {
            let am = Absolute::init(cpu);
            self::dec::Dec::execute(cpu, am)
        }
        0xde => {
            let am = AbsoluteX::init_rmw(cpu);
            self::dec::Dec::execute(cpu, am)
        }
        0x4c => {
            let am = AbsoluteAddress::init(cpu);
            self::jmp::Jmp::execute(cpu, am)
        }
        0x6c => {
            let am = Indirect::init(cpu);
            self::jmp::Jmp::execute(cpu, am)
        }
        0x20 => {
            let am = AbsoluteAddress::init(cpu);
            self::jsr::Jsr::execute(cpu, am)
        }
        _ => panic!("Unexpected opcode: {:0>2X}", opcode),
    }
}

pub struct Adc;

impl OpCode for Adc {
    type Input = u8;

    fn execute<S, M, AM>(cpu: &mut Cpu<S, M>, am: AM)
        where S: Screen,
              M: Memory<S>,
              AM: AddressingMode<S, M, Output = Self::Input>
    {
        let left = cpu.registers.acc;
        let right = am.read();
        adc_base(cpu, left, right)
    }
}

pub struct Sbc;

impl OpCode for Sbc {
    type Input = u8;

    fn execute<S, M, AM>(cpu: &mut Cpu<S, M>, am: AM)
        where S: Screen,
              M: Memory<S>,
              AM: AddressingMode<S, M, Output = Self::Input>
    {
        let lhs = cpu.registers.acc;
        let rhs = am.read();
        let rhs = !rhs;
        adc_base(cpu, lhs, rhs)
    }
}

pub fn adc_base<S: Screen, M: Memory<S>>(cpu: &mut Cpu<S, M>, lhs: u8, rhs: u8) {

    if cpu.registers.decimal_flag() {
        panic!("Attempted decimal mode arithmetic");
    } else {
        // See http://www.righto.com/2012/12/the-6502-overflow-flag-explained.html
        let carry = if cpu.registers.carry_flag() { 1 } else { 0 };

        // add using the native word size
        let res = carry + lhs as isize + rhs as isize;

        // if the operation carries into the 8th bit, carry flag will be 1,
        // and zero otherwise.
        let has_carry = res & 0x100 != 0;

        let res = res as u8;

        // Set the overflow flag when both operands have the same sign bit AND
        // the sign bit of the result differs from the two.
        let has_overflow = (lhs ^ rhs) & 0x80 == 0 && (lhs ^ res) & 0x80 != 0;

        cpu.registers.set_carry_flag(has_carry);
        cpu.registers.set_overflow_flag(has_overflow);
        cpu.registers.set_acc(res);
    }
}
