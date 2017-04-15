#[cfg(test)]
pub mod am_test_utils;

#[cfg(test)]
pub mod branch_tests_base;

#[cfg(test)]
pub mod compare_tests_base;

#[cfg(test)]
mod inc_dec_tests_base;

#[cfg(test)]
mod shift_tests_base;

#[cfg(test)]
mod adc_spec_tests;

#[cfg(test)]
mod sbc_spec_tests;

#[cfg(test)]
mod and_spec_tests;

#[cfg(test)]
mod asl_spec_tests;

#[cfg(test)]
mod rol_spec_tests;

#[cfg(test)]
mod ror_spec_tests;

#[cfg(test)]
mod lsr_spec_tests;

#[cfg(test)]
mod bcc_spec_tests;

#[cfg(test)]
mod bpl_spec_tests;

#[cfg(test)]
mod beq_spec_tests;

#[cfg(test)]
mod bmi_spec_tests;

#[cfg(test)]
mod bvc_spec_tests;

#[cfg(test)]
mod bvs_spec_tests;

#[cfg(test)]
mod bcs_spec_tests;

#[cfg(test)]
mod bne_spec_tests;

#[cfg(test)]
mod bit_spec_tests;

#[cfg(test)]
mod cmp_spec_tests;

#[cfg(test)]
mod cpx_spec_tests;

#[cfg(test)]
mod cpy_spec_tests;

#[cfg(test)]
mod brk_spec_tests;

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
mod rti;
mod pha;
mod pla;
mod php;
mod plp;
mod nop;
mod lda;
mod ldx;
mod ldy;
mod sta;
mod stx;
mod sty;
mod ora;
mod eor;
mod inc;
mod dec;
mod jmp;
mod jsr;

use byte_utils::{from_lo_hi, wrapping_add};
use cpu::Cpu;
use memory::Memory;
use screen::Screen;

const BRK_VECTOR: u16 = 0xfffe;

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
        0x00 => Brk::execute(cpu, Implied),
        0x40 => self::rti::Rti::execute(cpu, Implied),
        0x48 => self::pha::Pha::execute(cpu, Implied),
        0x68 => self::pla::Pla::execute(cpu, Implied),
        0x08 => self::php::Php::execute(cpu, Implied),
        0x28 => self::plp::Plp::execute(cpu, Implied),
        0xea => self::nop::Nop::execute(cpu, Implied),
        0x10 => {
            let am = Relative::init(cpu);
            Bpl::execute(cpu, am)
        }
        0x30 => {
            let am = Relative::init(cpu);
            Bmi::execute(cpu, am)
        }
        0x50 => {
            let am = Relative::init(cpu);
            Bvc::execute(cpu, am)
        }
        0x70 => {
            let am = Relative::init(cpu);
            Bvs::execute(cpu, am)
        }
        0x90 => {
            let am = Relative::init(cpu);
            Bcc::execute(cpu, am)
        }
        0xb0 => {
            let am = Relative::init(cpu);
            Bcs::execute(cpu, am)
        }
        0xd0 => {
            let am = Relative::init(cpu);
            Bne::execute(cpu, am)
        }
        0xf0 => {
            let am = Relative::init(cpu);
            Beq::execute(cpu, am)
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
            Cmp::execute(cpu, am)
        }
        0xc5 => {
            let am = ZeroPage::init(cpu);
            Cmp::execute(cpu, am)
        }
        0xd5 => {
            let am = ZeroPageX::init(cpu);
            Cmp::execute(cpu, am)
        }
        0xc1 => {
            let am = IndexedIndirect::init(cpu);
            Cmp::execute(cpu, am)
        }
        0xd1 => {
            let am = IndirectIndexed::init(cpu);
            Cmp::execute(cpu, am)
        }
        0xcd => {
            let am = Absolute::init(cpu);
            Cmp::execute(cpu, am)
        }
        0xdd => {
            let am = AbsoluteX::init(cpu);
            Cmp::execute(cpu, am)
        }
        0xd9 => {
            let am = AbsoluteY::init(cpu);
            Cmp::execute(cpu, am)
        }
        0xe0 => {
            let am = Immediate::init(cpu);
            Cpx::execute(cpu, am)
        }
        0xe4 => {
            let am = ZeroPage::init(cpu);
            Cpx::execute(cpu, am)
        }
        0xec => {
            let am = Absolute::init(cpu);
            Cpx::execute(cpu, am)
        }
        0xc0 => {
            let am = Immediate::init(cpu);
            Cpy::execute(cpu, am)
        }
        0xc4 => {
            let am = ZeroPage::init(cpu);
            Cpy::execute(cpu, am)
        }
        0xcc => {
            let am = Absolute::init(cpu);
            Cpy::execute(cpu, am)
        }
        0x29 => {
            let am = Immediate::init(cpu);
            And::execute(cpu, am)
        }
        0x25 => {
            let am = ZeroPage::init(cpu);
            And::execute(cpu, am)
        }
        0x35 => {
            let am = ZeroPageX::init(cpu);
            And::execute(cpu, am)
        }
        0x21 => {
            let am = IndexedIndirect::init(cpu);
            And::execute(cpu, am)
        }
        0x31 => {
            let am = IndirectIndexed::init(cpu);
            And::execute(cpu, am)
        }
        0x2d => {
            let am = Absolute::init(cpu);
            And::execute(cpu, am)
        }
        0x3d => {
            let am = AbsoluteX::init(cpu);
            And::execute(cpu, am)
        }
        0x39 => {
            let am = AbsoluteY::init(cpu);
            And::execute(cpu, am)
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
            Bit::execute(cpu, am)
        }
        0x2c => {
            let am = Absolute::init(cpu);
            Bit::execute(cpu, am)
        }
        0x2a => {
            let am = Accumulator::init(cpu);
            Rol::execute(cpu, am)
        }
        0x26 => {
            let am = ZeroPage::init(cpu);
            Rol::execute(cpu, am)
        }
        0x36 => {
            let am = ZeroPageX::init(cpu);
            Rol::execute(cpu, am)
        }
        0x2e => {
            let am = Absolute::init(cpu);
            Rol::execute(cpu, am)
        }
        0x3e => {
            let am = AbsoluteX::init_rmw(cpu);
            Rol::execute(cpu, am)
        }
        0x6a => {
            let am = Accumulator::init(cpu);
            Ror::execute(cpu, am)
        }
        0x66 => {
            let am = ZeroPage::init(cpu);
            Ror::execute(cpu, am)
        }
        0x76 => {
            let am = ZeroPageX::init(cpu);
            Ror::execute(cpu, am)
        }
        0x6e => {
            let am = Absolute::init(cpu);
            Ror::execute(cpu, am)
        }
        0x7e => {
            let am = AbsoluteX::init_rmw(cpu);
            Ror::execute(cpu, am)
        }
        0x0a => {
            let am = Accumulator::init(cpu);
            Asl::execute(cpu, am)
        }
        0x06 => {
            let am = ZeroPage::init(cpu);
            Asl::execute(cpu, am)
        }
        0x16 => {
            let am = ZeroPageX::init(cpu);
            Asl::execute(cpu, am)
        }
        0x0e => {
            let am = Absolute::init(cpu);
            Asl::execute(cpu, am)
        }
        0x1e => {
            let am = AbsoluteX::init_rmw(cpu);
            Asl::execute(cpu, am)
        }
        0x4a => {
            let am = Accumulator::init(cpu);
            Lsr::execute(cpu, am)
        }
        0x46 => {
            let am = ZeroPage::init(cpu);
            Lsr::execute(cpu, am)
        }
        0x56 => {
            let am = ZeroPageX::init(cpu);
            Lsr::execute(cpu, am)
        }
        0x4e => {
            let am = Absolute::init(cpu);
            Lsr::execute(cpu, am)
        }
        0x5e => {
            let am = AbsoluteX::init_rmw(cpu);
            Lsr::execute(cpu, am)
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

pub trait AddressingMode<S: Screen, M: Memory<S>> {
    type Output;
    fn read(&self) -> Self::Output;
    fn write(&self, _: &mut Cpu<S, M>, _: u8) {
        unimplemented!();
    }
}

pub struct Absolute {
    addr: u16,
    value: u8,
    is_store: bool,
}

impl Absolute {
    pub fn init<S: Screen, M: Memory<S>>(cpu: &mut Cpu<S, M>) -> Self {
        let addr = cpu.read_pc16();
        let value = cpu.read_memory(addr);

        Absolute {
            addr: addr,
            value: value,
            is_store: false,
        }
    }

    pub fn init_store<S: Screen, M: Memory<S>>(cpu: &mut Cpu<S, M>) -> Self {
        let addr = cpu.read_pc16();

        Absolute {
            addr: addr,
            value: 0, // Stores don't use the value and can cause illegal memory access if attempted
            is_store: true,
        }
    }
}

impl<S: Screen, M: Memory<S>> AddressingMode<S, M> for Absolute {
    type Output = u8;

    fn read(&self) -> Self::Output {
        self.value
    }

    fn write(&self, cpu: &mut Cpu<S, M>, value: u8) {
        if !self.is_store {
            // Dummy write cycle
            cpu.tick();
        }
        cpu.write_memory(self.addr, value)
    }
}

/// An absolute addressing mode for instructions that operate on the actually memory address, and
/// not the value at that address.
pub struct AbsoluteAddress {
    addr: u16,
}

impl AbsoluteAddress {
    pub fn init<S, M>(cpu: &mut Cpu<S, M>) -> Self
        where S: Screen,
              M: Memory<S>
    {
        AbsoluteAddress { addr: cpu.read_pc16() }
    }
}

impl<S: Screen, M: Memory<S>> AddressingMode<S, M> for AbsoluteAddress {
    type Output = u16;

    fn read(&self) -> Self::Output {
        self.addr
    }
}

pub struct AbsoluteX {
    addr: u16,
    value: u8,
    is_store: bool,
}


#[derive(PartialEq, Eq)]
enum Variant {
    Standard,
    ReadModifyWrite,
    Store,
}

impl AbsoluteX {
    pub fn init<S: Screen, M: Memory<S>>(cpu: &mut Cpu<S, M>) -> Self {
        Self::init_base(cpu, Variant::Standard)
    }

    pub fn init_store<S: Screen, M: Memory<S>>(cpu: &mut Cpu<S, M>) -> Self {
        Self::init_base(cpu, Variant::Store)
    }

    /// Init using special rules for cycle counting specific to read-modify-write instructions
    ///
    /// Read-modify-write instructions do not have a conditional page boundary cycle. For these
    /// instructions we always execute this cycle.
    pub fn init_rmw<S: Screen, M: Memory<S>>(cpu: &mut Cpu<S, M>) -> Self {
        Self::init_base(cpu, Variant::ReadModifyWrite)
    }

    fn init_base<S: Screen, M: Memory<S>>(cpu: &mut Cpu<S, M>, variant: Variant) -> Self {
        let base_addr = cpu.read_pc16();
        let target_addr = base_addr + cpu.registers.x as u16;

        // Conditional cycle if memory page crossed
        if variant != Variant::Store &&
           (variant == Variant::ReadModifyWrite || (base_addr & 0xff00 != target_addr & 0xff00)) {
            cpu.tick();
        }

        let val = if variant != Variant::Store {
            cpu.read_memory(target_addr)
        } else {
            cpu.tick();
            0x0 // Stores do not read memory and can cause illegal memory access if attempted
        };

        AbsoluteX {
            addr: target_addr,
            value: val,
            is_store: variant == Variant::Store,
        }
    }
}

impl<S: Screen, M: Memory<S>> AddressingMode<S, M> for AbsoluteX {
    type Output = u8;

    fn read(&self) -> Self::Output {
        self.value
    }

    fn write(&self, cpu: &mut Cpu<S, M>, value: u8) {
        if !self.is_store {
            // Dummy write cycle
            cpu.tick();
        }
        cpu.write_memory(self.addr, value)
    }
}

pub struct AbsoluteY {
    addr: u16,
    value: u8,
}

impl AbsoluteY {
    pub fn init<S: Screen, M: Memory<S>>(cpu: &mut Cpu<S, M>) -> Self {
        Self::init_base(cpu, false)
    }

    pub fn init_store<S: Screen, M: Memory<S>>(cpu: &mut Cpu<S, M>) -> Self {
        Self::init_base(cpu, true)
    }

    fn init_base<S: Screen, M: Memory<S>>(cpu: &mut Cpu<S, M>, is_store: bool) -> Self {
        let base_addr = cpu.read_pc16();
        let target_addr = base_addr + cpu.registers.y as u16;

        // Conditional cycle if memory page crossed
        if !is_store && base_addr & 0xff00 != target_addr & 0xff00 {
            cpu.tick()
        }

        let val = if !is_store {
            cpu.read_memory(target_addr)
        } else {
            cpu.tick();
            0x0 // Stores do not read memory and can cause illegal memory access if attempted
        };

        AbsoluteY {
            addr: target_addr,
            value: val,
        }
    }
}

impl<S: Screen, M: Memory<S>> AddressingMode<S, M> for AbsoluteY {
    type Output = u8;

    fn read(&self) -> Self::Output {
        self.value
    }

    fn write(&self, cpu: &mut Cpu<S, M>, value: u8) {
        cpu.write_memory(self.addr, value)
    }
}

pub struct Accumulator {
    value: u8,
}

impl Accumulator {
    pub fn init<S: Screen, M: Memory<S>>(cpu: &mut Cpu<S, M>) -> Self {
        // dummy read cycle
        cpu.tick();
        Accumulator { value: cpu.registers.acc }
    }
}

impl<S: Screen, M: Memory<S>> AddressingMode<S, M> for Accumulator {
    type Output = u8;

    fn read(&self) -> Self::Output {
        self.value
    }

    fn write(&self, cpu: &mut Cpu<S, M>, value: u8) {
        cpu.registers.acc = value;
    }
}

pub struct Immediate {
    value: u8,
}

impl Immediate {
    pub fn init<S: Screen, M: Memory<S>>(cpu: &mut Cpu<S, M>) -> Self {
        let val = cpu.read_pc();
        Immediate { value: val }
    }
}

impl<S: Screen, M: Memory<S>> AddressingMode<S, M> for Immediate {
    type Output = u8;

    fn read(&self) -> u8 {
        self.value
    }
}

pub struct Implied;

impl<S: Screen, M: Memory<S>> AddressingMode<S, M> for Implied {
    type Output = ();

    fn read(&self) -> Self::Output {
        ()
    }
}

pub struct IndexedIndirect {
    addr: u16,
    value: u8,
}

impl IndexedIndirect {
    pub fn init<S: Screen, M: Memory<S>>(cpu: &mut Cpu<S, M>) -> Self {
        Self::init_base(cpu, false)
    }

    pub fn init_store<S: Screen, M: Memory<S>>(cpu: &mut Cpu<S, M>) -> Self {
        Self::init_base(cpu, true)
    }

    fn init_base<S: Screen, M: Memory<S>>(cpu: &mut Cpu<S, M>, is_store: bool) -> Self {
        let operand = cpu.read_pc();
        let base_addr = wrapping_add(operand, cpu.registers.x) as u16;

        if !is_store {
            // Dummy read cycle
            cpu.tick();
        }

        let target_addr = cpu.read_memory16(base_addr);
        let value = cpu.read_memory(target_addr);

        IndexedIndirect {
            addr: target_addr,
            value: value,
        }
    }
}

impl<S: Screen, M: Memory<S>> AddressingMode<S, M> for IndexedIndirect {
    type Output = u8;

    fn read(&self) -> Self::Output {
        self.value
    }

    fn write(&self, cpu: &mut Cpu<S, M>, value: u8) {
        cpu.write_memory(self.addr, value)
    }
}

pub struct Indirect {
    addr: u16,
}

impl Indirect {
    pub fn init<S: Screen, M: Memory<S>>(cpu: &mut Cpu<S, M>) -> Self {
        let addr = cpu.read_pc16();

        // Recreate hardware bug specific to indirect jmp
        let lo_byte = cpu.read_memory(addr);

        // recreate indirect jump bug in nmos 6502
        let hi_byte = if addr & 0x00ff == 0x00ff {
            cpu.read_memory(addr & 0xff00)
        } else {
            cpu.read_memory(addr + 1)
        };

        let target_addr = from_lo_hi(lo_byte, hi_byte);
        Indirect { addr: target_addr }
    }
}

impl<S: Screen, M: Memory<S>> AddressingMode<S, M> for Indirect {
    type Output = u16;

    fn read(&self) -> Self::Output {
        self.addr
    }
}

pub struct IndirectIndexed {
    addr: u16,
    value: u8,
}

impl IndirectIndexed {
    pub fn init<S: Screen, M: Memory<S>>(cpu: &mut Cpu<S, M>) -> Self {
        Self::init_base(cpu, false)
    }

    pub fn init_store<S: Screen, M: Memory<S>>(cpu: &mut Cpu<S, M>) -> Self {
        Self::init_base(cpu, true)
    }

    fn init_base<S: Screen, M: Memory<S>>(cpu: &mut Cpu<S, M>, is_store: bool) -> Self {
        let addr = cpu.read_pc();
        let y = cpu.registers.y;
        let base_addr = cpu.read_memory16_zp(addr);
        let target_addr = base_addr + y as u16;

        // Conditional cycle if memory page crossed
        if !is_store && base_addr & 0xff00 != target_addr & 0xff00 {
            cpu.tick();
        }

        let val = cpu.read_memory(target_addr);
        IndirectIndexed {
            addr: target_addr,
            value: val,
        }
    }
}

impl<S: Screen, M: Memory<S>> AddressingMode<S, M> for IndirectIndexed {
    type Output = u8;

    fn read(&self) -> Self::Output {
        self.value
    }

    fn write(&self, cpu: &mut Cpu<S, M>, value: u8) {
        cpu.write_memory(self.addr, value)
    }
}

pub struct Relative {
    offset: i8,
}

impl Relative {
    pub fn init<S: Screen, M: Memory<S>>(cpu: &mut Cpu<S, M>) -> Self {
        let offset = cpu.read_pc() as i8;
        Relative { offset: offset }
    }
}

impl<S: Screen, M: Memory<S>> AddressingMode<S, M> for Relative {
    type Output = i8;

    fn read(&self) -> Self::Output {
        self.offset
    }
}

pub struct ZeroPage {
    addr: u16,
    value: u8,
    is_store: bool,
}

impl ZeroPage {
    pub fn init<S: Screen, M: Memory<S>>(cpu: &mut Cpu<S, M>) -> Self {
        let addr = cpu.read_pc() as u16;
        let val = cpu.read_memory(addr);

        ZeroPage {
            addr: addr,
            value: val,
            is_store: false,
        }
    }

    pub fn init_store<S: Screen, M: Memory<S>>(cpu: &mut Cpu<S, M>) -> Self {
        let addr = cpu.read_pc() as u16;

        ZeroPage {
            addr: addr,
            value: 0x0, // Stores don't read memory, can cause illegal memory access if attempted
            is_store: true,
        }
    }
}

impl<S: Screen, M: Memory<S>> AddressingMode<S, M> for ZeroPage {
    type Output = u8;

    fn read(&self) -> Self::Output {
        self.value
    }

    fn write(&self, cpu: &mut Cpu<S, M>, value: u8) {
        if !self.is_store {
            // Dummy write cycle
            cpu.tick();
        }
        cpu.write_memory(self.addr, value)
    }
}

pub struct ZeroPageX {
    addr: u16,
    value: u8,
    is_store: bool,
}

impl ZeroPageX {
    pub fn init<S: Screen, M: Memory<S>>(cpu: &mut Cpu<S, M>) -> Self {
        let base_addr = cpu.read_pc();
        let target_addr = wrapping_add(base_addr, cpu.registers.x) as u16;

        // Dummy read cycle
        cpu.tick();

        let val = cpu.read_memory(target_addr);

        ZeroPageX {
            addr: target_addr,
            value: val,
            is_store: false,
        }
    }

    pub fn init_store<S: Screen, M: Memory<S>>(cpu: &mut Cpu<S, M>) -> Self {
        let base_addr = cpu.read_pc();
        let target_addr = wrapping_add(base_addr, cpu.registers.x) as u16;

        let val = cpu.read_memory(target_addr);

        ZeroPageX {
            addr: target_addr,
            value: val,
            is_store: true,
        }
    }
}

impl<S: Screen, M: Memory<S>> AddressingMode<S, M> for ZeroPageX {
    type Output = u8;

    fn read(&self) -> Self::Output {
        self.value
    }

    fn write(&self, cpu: &mut Cpu<S, M>, value: u8) {
        if !self.is_store {
            // Dummy write cycle
            cpu.tick();
        }
        cpu.write_memory(self.addr, value)
    }
}

pub struct ZeroPageY {
    addr: u16,
    value: u8,
    is_store: bool,
}

impl ZeroPageY {
    pub fn init<S: Screen, M: Memory<S>>(cpu: &mut Cpu<S, M>) -> Self {
        Self::init_base(cpu, false)
    }

    pub fn init_store<S: Screen, M: Memory<S>>(cpu: &mut Cpu<S, M>) -> Self {
        Self::init_base(cpu, true)
    }

    fn init_base<S: Screen, M: Memory<S>>(cpu: &mut Cpu<S, M>, is_store: bool) -> Self {
        let base_addr = cpu.read_pc();
        let target_addr = wrapping_add(base_addr, cpu.registers.y) as u16;

        if !is_store {
            // Dummy read cycle
            cpu.tick();
        }

        let val = if !is_store {
            cpu.read_memory(target_addr)
        } else {
            cpu.tick();
            0x0 // Stores don't read memory, can cause illegal memory access if attempted
        };

        ZeroPageY {
            addr: target_addr,
            value: val,
            is_store: is_store,
        }
    }
}

impl<S: Screen, M: Memory<S>> AddressingMode<S, M> for ZeroPageY {
    type Output = u8;

    fn read(&self) -> Self::Output {
        self.value
    }

    fn write(&self, cpu: &mut Cpu<S, M>, value: u8) {
        if !self.is_store {
            // Dummy write cycle
            cpu.tick();
        }
        cpu.write_memory(self.addr, value)
    }
}

struct Adc;

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

struct Sbc;

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

fn adc_base<S: Screen, M: Memory<S>>(cpu: &mut Cpu<S, M>, lhs: u8, rhs: u8) {

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

struct And;

impl OpCode for And {
    type Input = u8;

fn execute<S: Screen, M: Memory<S>, AM: AddressingMode<S, M, Output = Self::Input>>(cpu: &mut Cpu<S, M>, am: AM){
        let lhs = cpu.registers.acc;
        let rhs = am.read();
        let res = lhs & rhs;
        cpu.registers.set_acc(res);
    }
}

fn shift_left<S: Screen, M: Memory<S>, AM: AddressingMode<S, M, Output = u8>>(cpu: &mut Cpu<S, M>,
                                                                              am: AM,
                                                                              lsb: bool) {
    let val = am.read();
    let carry = (val & 0x80) != 0;
    let res = if lsb { (val << 1) | 0x1 } else { val << 1 };
    cpu.registers.set_carry_flag(carry);
    cpu.registers.set_sign_and_zero_flag(res);
    am.write(cpu, res)
}

fn shift_right<S: Screen, M: Memory<S>, AM: AddressingMode<S, M, Output = u8>>(cpu: &mut Cpu<S,
                                                                                             M>,
                                                                               am: AM,
                                                                               msb: bool) {
    let val = am.read();
    let carry = (val & 0x1) != 0;
    let res = if msb { (val >> 1) | 0x80 } else { val >> 1 };
    cpu.registers.set_carry_flag(carry);
    cpu.registers.set_sign_and_zero_flag(res);
    am.write(cpu, res)
}

struct Asl;

impl OpCode for Asl {
    type Input = u8;

fn execute<S: Screen, M: Memory<S>, AM: AddressingMode<S, M, Output = Self::Input>>(cpu: &mut Cpu<S, M>, am: AM){
        shift_left(cpu, am, false)
    }
}

struct Rol;

impl OpCode for Rol {
    type Input = u8;

fn execute<S: Screen, M: Memory<S>, AM: AddressingMode<S, M, Output = Self::Input>>(cpu: &mut Cpu<S, M>, am: AM){
        let carry_set = cpu.registers.carry_flag();
        shift_left(cpu, am, carry_set)
    }
}

struct Ror;

impl OpCode for Ror {
    type Input = u8;

fn execute<S: Screen, M: Memory<S>, AM: AddressingMode<S, M, Output = Self::Input>>(cpu: &mut Cpu<S, M>, am: AM){
        let carry_set = cpu.registers.carry_flag();
        shift_right(cpu, am, carry_set)
    }
}

struct Lsr;

impl OpCode for Lsr {
    type Input = u8;

    fn execute<S, M, AM>(cpu: &mut Cpu<S, M>, am: AM)
        where S: Screen,
              M: Memory<S>,
              AM: AddressingMode<S, M, Output = Self::Input>
    {
        shift_right(cpu, am, false)
    }
}

fn branch<S: Screen, M: Memory<S>, AM: AddressingMode<S, M, Output = i8>>(cpu: &mut Cpu<S, M>,
                                                                          am: AM,
                                                                          condition: bool) {
    if condition {
        let rel_addr = am.read();
        let old_pc = cpu.registers.pc;
        cpu.registers.pc = (cpu.registers.pc as i32 + rel_addr as i32) as u16;
        cpu.tick();

        // Conditional cycle if pc crosses page boundary
        if old_pc & 0xFF00 != cpu.registers.pc & 0xFF00 {
            cpu.tick();
        }
    }
}

struct Bcc;

impl OpCode for Bcc {
    type Input = i8;

fn execute<S: Screen, M: Memory<S>, AM: AddressingMode<S, M, Output = Self::Input>>(cpu: &mut Cpu<S, M>, am: AM){
        let carry_clear = !cpu.registers.carry_flag();
        branch(cpu, am, carry_clear)
    }
}

pub struct Bpl;

impl OpCode for Bpl {
    type Input = i8;

fn execute<S: Screen, M: Memory<S>, AM: AddressingMode<S, M, Output = Self::Input>>(cpu: &mut Cpu<S, M>, am: AM){
        let sign_clear = !cpu.registers.sign_flag();
        branch(cpu, am, sign_clear)
    }
}

struct Beq;

impl OpCode for Beq {
    type Input = i8;

fn execute<S: Screen, M: Memory<S>, AM: AddressingMode<S, M, Output = Self::Input>>(cpu: &mut Cpu<S, M>, am: AM){
        let zero_set = cpu.registers.zero_flag();
        branch(cpu, am, zero_set)
    }
}

struct Bmi;

impl OpCode for Bmi {
    type Input = i8;

fn execute<S: Screen, M: Memory<S>, AM: AddressingMode<S, M, Output = Self::Input>>(cpu: &mut Cpu<S, M>, am: AM){
        let sign_set = cpu.registers.sign_flag();
        branch(cpu, am, sign_set)
    }
}

struct Bvc;

impl OpCode for Bvc {
    type Input = i8;

fn execute<S: Screen, M: Memory<S>, AM: AddressingMode<S, M, Output = Self::Input>>(cpu: &mut Cpu<S, M>, am: AM){
        let sign_clear = !cpu.registers.overflow_flag();
        branch(cpu, am, sign_clear)
    }
}

struct Bvs;

impl OpCode for Bvs {
    type Input = i8;

fn execute<S: Screen, M: Memory<S>, AM: AddressingMode<S, M, Output = Self::Input>>(cpu: &mut Cpu<S, M>, am: AM){
        let sign_clear = cpu.registers.overflow_flag();
        branch(cpu, am, sign_clear)
    }
}

struct Bcs;

impl OpCode for Bcs {
    type Input = i8;

fn execute<S: Screen, M: Memory<S>, AM: AddressingMode<S, M, Output = Self::Input>>(cpu: &mut Cpu<S, M>, am: AM){
        let carry_set = cpu.registers.carry_flag();
        branch(cpu, am, carry_set)
    }
}

struct Bne;

impl OpCode for Bne {
    type Input = i8;

fn execute<S: Screen, M: Memory<S>, AM: AddressingMode<S, M, Output = Self::Input>>(cpu: &mut Cpu<S, M>, am: AM){
        let zero_clear = !cpu.registers.zero_flag();
        branch(cpu, am, zero_clear)
    }
}

fn compare<S: Screen, M: Memory<S>, AM: AddressingMode<S, M, Output = u8>>(cpu: &mut Cpu<S, M>,
                                                                           am: AM,
                                                                           lhs: u8) {
    let rhs = am.read();
    let res = lhs as i32 - rhs as i32;
    cpu.registers.set_carry_flag(res & 0x100 == 0);
    cpu.registers.set_sign_and_zero_flag(res as u8);
}

struct Cmp;

impl OpCode for Cmp {
    type Input = u8;

fn execute<S: Screen, M: Memory<S>, AM: AddressingMode<S, M, Output = Self::Input>>(cpu: &mut Cpu<S, M>, am: AM){
        let val = cpu.registers.acc;
        compare(cpu, am, val);
    }
}

struct Cpx;

impl OpCode for Cpx {
    type Input = u8;

fn execute<S: Screen, M: Memory<S>, AM: AddressingMode<S, M, Output = Self::Input>>(cpu: &mut Cpu<S, M>, am: AM){
        let val = cpu.registers.x;
        compare(cpu, am, val);
    }
}

struct Cpy;

impl OpCode for Cpy {
    type Input = u8;

fn execute<S: Screen, M: Memory<S>, AM: AddressingMode<S, M, Output = Self::Input>>(cpu: &mut Cpu<S, M>, am: AM){
        let val = cpu.registers.y;
        compare(cpu, am, val);
    }
}

struct Bit;

impl OpCode for Bit {
    type Input = u8;

fn execute<S: Screen, M: Memory<S>, AM: AddressingMode<S, M, Output = Self::Input>>(cpu: &mut Cpu<S, M>, am: AM){
        let lhs = cpu.registers.acc;
        let rhs = am.read();
        let res = lhs & rhs;

        cpu.registers.set_zero_flag(res == 0);
        cpu.registers.set_overflow_flag(rhs & 0x40 != 0);
        cpu.registers.set_sign_flag(rhs & 0x80 != 0);
    }
}

struct Brk;

impl OpCode for Brk {
    type Input = ();

fn execute<S: Screen, M: Memory<S>, AM: AddressingMode<S, M, Output = Self::Input>>(cpu: &mut Cpu<S, M>, _: AM){
        cpu.registers.pc += 1;
        let pc = cpu.registers.pc;
        let status = cpu.registers.status;
        cpu.push_stack16(pc);
        cpu.push_stack(status);
        let irq_handler = cpu.read_memory16(BRK_VECTOR);
        cpu.registers.pc = irq_handler;
        cpu.registers.set_interrupt_disable_flag(true);
    }
}
