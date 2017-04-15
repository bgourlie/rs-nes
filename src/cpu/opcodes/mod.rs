#[cfg(test)]
pub mod am_test_utils;

#[cfg(test)]
mod arithmetic_instr_spec_tests;

#[cfg(test)]
mod bitwise_and_shift_instr_spec_tests;

#[cfg(test)]
mod branch_spec_tests;

#[cfg(test)]
mod compare_spec_tests;

#[cfg(test)]
mod reg_transfer_spec_tests;

#[cfg(test)]
mod flag_instr_spec_tests;

#[cfg(test)]
mod loads_and_stores_spec_tests;

#[cfg(test)]
mod inc_dec_spec_tests;

#[cfg(test)]
mod jump_and_returns_instr_spec_tests;

use byte_utils::*;
use cpu::Cpu;
use input::Input;
use memory::Memory;
use screen::Screen;

const BRK_VECTOR: u16 = 0xfffe;

trait OpCode {
    type Input;
    fn execute<S, I, M, AM>(cpu: &mut Cpu<S, I, M>, am: AM)
        where S: Screen,
              I: Input,
              M: Memory<I, S>,
              AM: AddressingMode<S, I, M, Output = Self::Input>;
}

pub fn execute<S: Screen, I: Input, M: Memory<I, S>>(cpu: &mut Cpu<S, I, M>, opcode: u8) {
    match opcode {
        0xe8 => Inx::execute(cpu, Implied),
        0xca => Dex::execute(cpu, Implied),
        0xc8 => Iny::execute(cpu, Implied),
        0x88 => Dey::execute(cpu, Implied),
        0xaa => Tax::execute(cpu, Implied),
        0xa8 => Tay::execute(cpu, Implied),
        0x8a => Txa::execute(cpu, Implied),
        0x98 => Tya::execute(cpu, Implied),
        0x9a => Txs::execute(cpu, Implied),
        0xba => Tsx::execute(cpu, Implied),
        0x18 => Clc::execute(cpu, Implied),
        0x38 => Sec::execute(cpu, Implied),
        0x58 => Cli::execute(cpu, Implied),
        0x78 => Sei::execute(cpu, Implied),
        0xb8 => Clv::execute(cpu, Implied),
        0xd8 => Cld::execute(cpu, Implied),
        0xf8 => Sed::execute(cpu, Implied),
        0x60 => Rts::execute(cpu, Implied),
        0x00 => Brk::execute(cpu, Implied),
        0x40 => Rti::execute(cpu, Implied),
        0x48 => Pha::execute(cpu, Implied),
        0x68 => Pla::execute(cpu, Implied),
        0x08 => Php::execute(cpu, Implied),
        0x28 => Plp::execute(cpu, Implied),
        0xea => Nop::execute(cpu, Implied),
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
            Lda::execute(cpu, am)
        }
        0xa5 => {
            let am = ZeroPage::init(cpu);
            Lda::execute(cpu, am)
        }
        0xa9 => {
            let am = Immediate::init(cpu);
            Lda::execute(cpu, am)
        }
        0xb1 => {
            let am = IndirectIndexed::init(cpu);
            Lda::execute(cpu, am)
        }
        0xb5 => {
            let am = ZeroPageX::init(cpu);
            Lda::execute(cpu, am)
        }
        0xad => {
            let am = Absolute::init(cpu);
            Lda::execute(cpu, am)
        }
        0xb9 => {
            let am = AbsoluteY::init(cpu);
            Lda::execute(cpu, am)
        }
        0xbd => {
            let am = AbsoluteX::init(cpu);
            Lda::execute(cpu, am)
        }
        0xa2 => {
            let am = Immediate::init(cpu);
            Ldx::execute(cpu, am)
        }
        0xa6 => {
            let am = ZeroPage::init(cpu);
            Ldx::execute(cpu, am)
        }
        0xb6 => {
            let am = ZeroPageY::init(cpu);
            Ldx::execute(cpu, am)
        }
        0xae => {
            let am = Absolute::init(cpu);
            Ldx::execute(cpu, am)
        }
        0xbe => {
            let am = AbsoluteY::init(cpu);
            Ldx::execute(cpu, am)
        }
        0xa0 => {
            let am = Immediate::init(cpu);
            Ldy::execute(cpu, am)
        }
        0xa4 => {
            let am = ZeroPage::init(cpu);
            Ldy::execute(cpu, am)
        }
        0xb4 => {
            let am = ZeroPageX::init(cpu);
            Ldy::execute(cpu, am)
        }
        0xac => {
            let am = Absolute::init(cpu);
            Ldy::execute(cpu, am)
        }
        0xbc => {
            let am = AbsoluteX::init(cpu);
            Ldy::execute(cpu, am)
        }
        0x85 => {
            let am = ZeroPage::init_store(cpu);
            Sta::execute(cpu, am)
        }
        0x95 => {
            let am = ZeroPageX::init_store(cpu);
            Sta::execute(cpu, am)
        }
        0x81 => {
            let am = IndexedIndirect::init_store(cpu);
            Sta::execute(cpu, am)
        }
        0x91 => {
            let am = IndirectIndexed::init_store(cpu);
            Sta::execute(cpu, am)
        }
        0x8d => {
            let am = Absolute::init_store(cpu);
            Sta::execute(cpu, am)
        }
        0x9d => {
            let am = AbsoluteX::init_store(cpu);
            Sta::execute(cpu, am)
        }
        0x99 => {
            let am = AbsoluteY::init_store(cpu);
            Sta::execute(cpu, am)
        }
        0x86 => {
            let am = ZeroPage::init_store(cpu);
            Stx::execute(cpu, am)
        }
        0x96 => {
            let am = ZeroPageY::init_store(cpu);
            Stx::execute(cpu, am)
        }
        0x8e => {
            let am = Absolute::init_store(cpu);
            Stx::execute(cpu, am)
        }
        0x84 => {
            let am = ZeroPage::init_store(cpu);
            Sty::execute(cpu, am)
        }
        0x94 => {
            let am = ZeroPageX::init_store(cpu);
            Sty::execute(cpu, am)
        }
        0x8c => {
            let am = Absolute::init_store(cpu);
            Sty::execute(cpu, am)
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
            Ora::execute(cpu, am)
        }
        0x05 => {
            let am = ZeroPage::init(cpu);
            Ora::execute(cpu, am)
        }
        0x15 => {
            let am = ZeroPageX::init(cpu);
            Ora::execute(cpu, am)
        }
        0x01 => {
            let am = IndexedIndirect::init(cpu);
            Ora::execute(cpu, am)
        }
        0x11 => {
            let am = IndirectIndexed::init(cpu);
            Ora::execute(cpu, am)
        }
        0x0d => {
            let am = Absolute::init(cpu);
            Ora::execute(cpu, am)
        }
        0x1d => {
            let am = AbsoluteX::init(cpu);
            Ora::execute(cpu, am)
        }
        0x19 => {
            let am = AbsoluteY::init(cpu);
            Ora::execute(cpu, am)
        }
        0x49 => {
            let am = Immediate::init(cpu);
            Eor::execute(cpu, am)
        }
        0x45 => {
            let am = ZeroPage::init(cpu);
            Eor::execute(cpu, am)
        }
        0x55 => {
            let am = ZeroPageX::init(cpu);
            Eor::execute(cpu, am)
        }
        0x41 => {
            let am = IndexedIndirect::init(cpu);
            Eor::execute(cpu, am)
        }
        0x51 => {
            let am = IndirectIndexed::init(cpu);
            Eor::execute(cpu, am)
        }
        0x4d => {
            let am = Absolute::init(cpu);
            Eor::execute(cpu, am)
        }
        0x5d => {
            let am = AbsoluteX::init(cpu);
            Eor::execute(cpu, am)
        }
        0x59 => {
            let am = AbsoluteY::init(cpu);
            Eor::execute(cpu, am)
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
            Inc::execute(cpu, am)
        }
        0xf6 => {
            let am = ZeroPageX::init(cpu);
            Inc::execute(cpu, am)
        }
        0xee => {
            let am = Absolute::init(cpu);
            Inc::execute(cpu, am)
        }
        0xfe => {
            let am = AbsoluteX::init_rmw(cpu);
            Inc::execute(cpu, am)
        }
        0xc6 => {
            let am = ZeroPage::init(cpu);
            Dec::execute(cpu, am)
        }
        0xd6 => {
            let am = ZeroPageX::init(cpu);
            Dec::execute(cpu, am)
        }
        0xce => {
            let am = Absolute::init(cpu);
            Dec::execute(cpu, am)
        }
        0xde => {
            let am = AbsoluteX::init_rmw(cpu);
            Dec::execute(cpu, am)
        }
        0x4c => {
            let am = AbsoluteAddress::init(cpu);
            Jmp::execute(cpu, am)
        }
        0x6c => {
            let am = Indirect::init(cpu);
            Jmp::execute(cpu, am)
        }
        0x20 => {
            let am = AbsoluteAddress::init(cpu);
            Jsr::execute(cpu, am)
        }
        _ => panic!("Unexpected opcode: {:0>2X}", opcode),
    }
}

pub trait AddressingMode<S: Screen, I: Input, M: Memory<I, S>> {
    type Output;
    fn read(&self) -> Self::Output;
    fn write(&self, _: &mut Cpu<S, I, M>, _: u8) {
        unimplemented!();
    }
}

pub struct Absolute {
    addr: u16,
    value: u8,
    is_store: bool,
}

impl Absolute {
    pub fn init<S: Screen, I: Input, M: Memory<I, S>>(cpu: &mut Cpu<S, I, M>) -> Self {
        let addr = cpu.read_pc16();
        let value = cpu.read_memory(addr);

        Absolute {
            addr: addr,
            value: value,
            is_store: false,
        }
    }

    pub fn init_store<S: Screen, I: Input, M: Memory<I, S>>(cpu: &mut Cpu<S, I, M>) -> Self {
        let addr = cpu.read_pc16();

        Absolute {
            addr: addr,
            value: 0, // Stores don't use the value and can cause illegal memory access if attempted
            is_store: true,
        }
    }
}

impl<S: Screen, I: Input, M: Memory<I, S>> AddressingMode<S, I, M> for Absolute {
    type Output = u8;

    fn read(&self) -> Self::Output {
        self.value
    }

    fn write(&self, cpu: &mut Cpu<S, I, M>, value: u8) {
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
    pub fn init<S, I, M>(cpu: &mut Cpu<S, I, M>) -> Self
        where S: Screen,
              I: Input,
              M: Memory<I, S>
    {
        AbsoluteAddress { addr: cpu.read_pc16() }
    }
}

impl<S: Screen, I: Input, M: Memory<I, S>> AddressingMode<S, I, M> for AbsoluteAddress {
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
    pub fn init<S: Screen, I: Input, M: Memory<I, S>>(cpu: &mut Cpu<S, I, M>) -> Self {
        Self::init_base(cpu, Variant::Standard)
    }

    pub fn init_store<S: Screen, I: Input, M: Memory<I, S>>(cpu: &mut Cpu<S, I, M>) -> Self {
        Self::init_base(cpu, Variant::Store)
    }

    /// Init using special rules for cycle counting specific to read-modify-write instructions
    ///
    /// Read-modify-write instructions do not have a conditional page boundary cycle. For these
    /// instructions we always execute this cycle.
    pub fn init_rmw<S: Screen, I: Input, M: Memory<I, S>>(cpu: &mut Cpu<S, I, M>) -> Self {
        Self::init_base(cpu, Variant::ReadModifyWrite)
    }

    fn init_base<S: Screen, I: Input, M: Memory<I, S>>(cpu: &mut Cpu<S, I, M>,
                                                       variant: Variant)
                                                       -> Self {
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

impl<S: Screen, I: Input, M: Memory<I, S>> AddressingMode<S, I, M> for AbsoluteX {
    type Output = u8;

    fn read(&self) -> Self::Output {
        self.value
    }

    fn write(&self, cpu: &mut Cpu<S, I, M>, value: u8) {
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
    pub fn init<S: Screen, I: Input, M: Memory<I, S>>(cpu: &mut Cpu<S, I, M>) -> Self {
        Self::init_base(cpu, false)
    }

    pub fn init_store<S: Screen, I: Input, M: Memory<I, S>>(cpu: &mut Cpu<S, I, M>) -> Self {
        Self::init_base(cpu, true)
    }

    fn init_base<S, I, M>(cpu: &mut Cpu<S, I, M>, is_store: bool) -> Self
        where S: Screen,
              I: Input,
              M: Memory<I, S>
    {
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

impl<S: Screen, I: Input, M: Memory<I, S>> AddressingMode<S, I, M> for AbsoluteY {
    type Output = u8;

    fn read(&self) -> Self::Output {
        self.value
    }

    fn write(&self, cpu: &mut Cpu<S, I, M>, value: u8) {
        cpu.write_memory(self.addr, value)
    }
}

pub struct Accumulator {
    value: u8,
}

impl Accumulator {
    pub fn init<S: Screen, I: Input, M: Memory<I, S>>(cpu: &mut Cpu<S, I, M>) -> Self {
        // dummy read cycle
        cpu.tick();
        Accumulator { value: cpu.registers.acc }
    }
}

impl<S: Screen, I: Input, M: Memory<I, S>> AddressingMode<S, I, M> for Accumulator {
    type Output = u8;

    fn read(&self) -> Self::Output {
        self.value
    }

    fn write(&self, cpu: &mut Cpu<S, I, M>, value: u8) {
        cpu.registers.acc = value;
    }
}

pub struct Immediate {
    value: u8,
}

impl Immediate {
    pub fn init<S: Screen, I: Input, M: Memory<I, S>>(cpu: &mut Cpu<S, I, M>) -> Self {
        let val = cpu.read_pc();
        Immediate { value: val }
    }
}

impl<S: Screen, I: Input, M: Memory<I, S>> AddressingMode<S, I, M> for Immediate {
    type Output = u8;

    fn read(&self) -> u8 {
        self.value
    }
}

pub struct Implied;

impl<S: Screen, I: Input, M: Memory<I, S>> AddressingMode<S, I, M> for Implied {
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
    pub fn init<S: Screen, I: Input, M: Memory<I, S>>(cpu: &mut Cpu<S, I, M>) -> Self {
        Self::init_base(cpu, false)
    }

    pub fn init_store<S: Screen, I: Input, M: Memory<I, S>>(cpu: &mut Cpu<S, I, M>) -> Self {
        Self::init_base(cpu, true)
    }

    fn init_base<S, I, M>(cpu: &mut Cpu<S, I, M>, is_store: bool) -> Self
        where S: Screen,
              I: Input,
              M: Memory<I, S>
    {
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

impl<S: Screen, I: Input, M: Memory<I, S>> AddressingMode<S, I, M> for IndexedIndirect {
    type Output = u8;

    fn read(&self) -> Self::Output {
        self.value
    }

    fn write(&self, cpu: &mut Cpu<S, I, M>, value: u8) {
        cpu.write_memory(self.addr, value)
    }
}

pub struct Indirect {
    addr: u16,
}

impl Indirect {
    pub fn init<S: Screen, I: Input, M: Memory<I, S>>(cpu: &mut Cpu<S, I, M>) -> Self {
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

impl<S: Screen, I: Input, M: Memory<I, S>> AddressingMode<S, I, M> for Indirect {
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
    pub fn init<S, I, M>(cpu: &mut Cpu<S, I, M>) -> Self
        where S: Screen,
              I: Input,
              M: Memory<I, S>
    {
        Self::init_base(cpu, false)
    }

    pub fn init_store<S, I, M>(cpu: &mut Cpu<S, I, M>) -> Self
        where S: Screen,
              I: Input,
              M: Memory<I, S>
    {
        Self::init_base(cpu, true)
    }

    fn init_base<S, I, M>(cpu: &mut Cpu<S, I, M>, is_store: bool) -> Self
        where S: Screen,
              I: Input,
              M: Memory<I, S>
    {
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

impl<S: Screen, I: Input, M: Memory<I, S>> AddressingMode<S, I, M> for IndirectIndexed {
    type Output = u8;

    fn read(&self) -> Self::Output {
        self.value
    }

    fn write(&self, cpu: &mut Cpu<S, I, M>, value: u8) {
        cpu.write_memory(self.addr, value)
    }
}

pub struct Relative {
    offset: i8,
}

impl Relative {
    pub fn init<S: Screen, I: Input, M: Memory<I, S>>(cpu: &mut Cpu<S, I, M>) -> Self {
        let offset = cpu.read_pc() as i8;
        Relative { offset: offset }
    }
}

impl<S: Screen, I: Input, M: Memory<I, S>> AddressingMode<S, I, M> for Relative {
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
    pub fn init<S: Screen, I: Input, M: Memory<I, S>>(cpu: &mut Cpu<S, I, M>) -> Self {
        let addr = cpu.read_pc() as u16;
        let val = cpu.read_memory(addr);

        ZeroPage {
            addr: addr,
            value: val,
            is_store: false,
        }
    }

    pub fn init_store<S: Screen, I: Input, M: Memory<I, S>>(cpu: &mut Cpu<S, I, M>) -> Self {
        let addr = cpu.read_pc() as u16;

        ZeroPage {
            addr: addr,
            value: 0x0, // Stores don't read memory, can cause illegal memory access if attempted
            is_store: true,
        }
    }
}

impl<S: Screen, I: Input, M: Memory<I, S>> AddressingMode<S, I, M> for ZeroPage {
    type Output = u8;

    fn read(&self) -> Self::Output {
        self.value
    }

    fn write(&self, cpu: &mut Cpu<S, I, M>, value: u8) {
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
    pub fn init<S: Screen, I: Input, M: Memory<I, S>>(cpu: &mut Cpu<S, I, M>) -> Self {
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

    pub fn init_store<S: Screen, I: Input, M: Memory<I, S>>(cpu: &mut Cpu<S, I, M>) -> Self {
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

impl<S: Screen, I: Input, M: Memory<I, S>> AddressingMode<S, I, M> for ZeroPageX {
    type Output = u8;

    fn read(&self) -> Self::Output {
        self.value
    }

    fn write(&self, cpu: &mut Cpu<S, I, M>, value: u8) {
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
    pub fn init<S, I, M>(cpu: &mut Cpu<S, I, M>) -> Self
        where S: Screen,
              I: Input,
              M: Memory<I, S>
    {
        Self::init_base(cpu, false)
    }

    pub fn init_store<S, I, M>(cpu: &mut Cpu<S, I, M>) -> Self
        where S: Screen,
              I: Input,
              M: Memory<I, S>
    {
        Self::init_base(cpu, true)
    }

    fn init_base<S, I, M>(cpu: &mut Cpu<S, I, M>, is_store: bool) -> Self
        where S: Screen,
              I: Input,
              M: Memory<I, S>
    {
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

impl<S: Screen, I: Input, M: Memory<I, S>> AddressingMode<S, I, M> for ZeroPageY {
    type Output = u8;

    fn read(&self) -> Self::Output {
        self.value
    }

    fn write(&self, cpu: &mut Cpu<S, I, M>, value: u8) {
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

    fn execute<S, I, M, AM>(cpu: &mut Cpu<S, I, M>, am: AM)
        where S: Screen,
              I: Input,
              M: Memory<I, S>,
              AM: AddressingMode<S, I, M, Output = Self::Input>
    {
        let left = cpu.registers.acc;
        let right = am.read();
        adc_base(cpu, left, right)
    }
}

struct Sbc;

impl OpCode for Sbc {
    type Input = u8;

    fn execute<S, I, M, AM>(cpu: &mut Cpu<S, I, M>, am: AM)
        where S: Screen,
              I: Input,
              M: Memory<I, S>,
              AM: AddressingMode<S, I, M, Output = Self::Input>
    {
        let lhs = cpu.registers.acc;
        let rhs = am.read();
        let rhs = !rhs;
        adc_base(cpu, lhs, rhs)
    }
}

fn adc_base<S: Screen, I: Input, M: Memory<I, S>>(cpu: &mut Cpu<S, I, M>, lhs: u8, rhs: u8) {

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

    fn execute<S, I, M, AM>(cpu: &mut Cpu<S, I, M>, am: AM)
        where S: Screen,
              I: Input,
              M: Memory<I, S>,
              AM: AddressingMode<S, I, M, Output = Self::Input>
    {
        let lhs = cpu.registers.acc;
        let rhs = am.read();
        let res = lhs & rhs;
        cpu.registers.set_acc(res);
    }
}

fn shift_left<S, I, M, AM>(cpu: &mut Cpu<S, I, M>, am: AM, lsb: bool)
    where S: Screen,
          I: Input,
          M: Memory<I, S>,
          AM: AddressingMode<S, I, M, Output = u8>
{
    let val = am.read();
    let carry = (val & 0x80) != 0;
    let res = if lsb { (val << 1) | 0x1 } else { val << 1 };
    cpu.registers.set_carry_flag(carry);
    cpu.registers.set_sign_and_zero_flag(res);
    am.write(cpu, res)
}

fn shift_right<S, I, M, AM>(cpu: &mut Cpu<S, I, M>, am: AM, msb: bool)
    where S: Screen,
          I: Input,
          M: Memory<I, S>,
          AM: AddressingMode<S, I, M, Output = u8>
{
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

    fn execute<S, I, M, AM>(cpu: &mut Cpu<S, I, M>, am: AM)
        where S: Screen,
              I: Input,
              M: Memory<I, S>,
              AM: AddressingMode<S, I, M, Output = Self::Input>
    {
        shift_left(cpu, am, false)
    }
}

struct Rol;

impl OpCode for Rol {
    type Input = u8;

    fn execute<S, I, M, AM>(cpu: &mut Cpu<S, I, M>, am: AM)
        where S: Screen,
              I: Input,
              M: Memory<I, S>,
              AM: AddressingMode<S, I, M, Output = Self::Input>
    {
        let carry_set = cpu.registers.carry_flag();
        shift_left(cpu, am, carry_set)
    }
}

struct Ror;

impl OpCode for Ror {
    type Input = u8;

    fn execute<S, I, M, AM>(cpu: &mut Cpu<S, I, M>, am: AM)
        where S: Screen,
              I: Input,
              M: Memory<I, S>,
              AM: AddressingMode<S, I, M, Output = Self::Input>
    {
        let carry_set = cpu.registers.carry_flag();
        shift_right(cpu, am, carry_set)
    }
}

struct Lsr;

impl OpCode for Lsr {
    type Input = u8;

    fn execute<S, I, M, AM>(cpu: &mut Cpu<S, I, M>, am: AM)
        where S: Screen,
              I: Input,
              M: Memory<I, S>,
              AM: AddressingMode<S, I, M, Output = Self::Input>
    {
        shift_right(cpu, am, false)
    }
}

fn branch<S, I, M, AM>(cpu: &mut Cpu<S, I, M>, am: AM, condition: bool)
    where S: Screen,
          I: Input,
          M: Memory<I, S>,
          AM: AddressingMode<S, I, M, Output = i8>
{
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

    fn execute<S, I, M, AM>(cpu: &mut Cpu<S, I, M>, am: AM)
        where S: Screen,
              I: Input,
              M: Memory<I, S>,
              AM: AddressingMode<S, I, M, Output = Self::Input>
    {
        let carry_clear = !cpu.registers.carry_flag();
        branch(cpu, am, carry_clear)
    }
}

pub struct Bpl;

impl OpCode for Bpl {
    type Input = i8;

    fn execute<S, I, M, AM>(cpu: &mut Cpu<S, I, M>, am: AM)
        where S: Screen,
              I: Input,
              M: Memory<I, S>,
              AM: AddressingMode<S, I, M, Output = Self::Input>
    {
        let sign_clear = !cpu.registers.sign_flag();
        branch(cpu, am, sign_clear)
    }
}

struct Beq;

impl OpCode for Beq {
    type Input = i8;

    fn execute<S, I, M, AM>(cpu: &mut Cpu<S, I, M>, am: AM)
        where S: Screen,
              I: Input,
              M: Memory<I, S>,
              AM: AddressingMode<S, I, M, Output = Self::Input>
    {
        let zero_set = cpu.registers.zero_flag();
        branch(cpu, am, zero_set)
    }
}

struct Bmi;

impl OpCode for Bmi {
    type Input = i8;

    fn execute<S, I, M, AM>(cpu: &mut Cpu<S, I, M>, am: AM)
        where S: Screen,
              I: Input,
              M: Memory<I, S>,
              AM: AddressingMode<S, I, M, Output = Self::Input>
    {
        let sign_set = cpu.registers.sign_flag();
        branch(cpu, am, sign_set)
    }
}

struct Bvc;

impl OpCode for Bvc {
    type Input = i8;

    fn execute<S, I, M, AM>(cpu: &mut Cpu<S, I, M>, am: AM)
        where S: Screen,
              I: Input,
              M: Memory<I, S>,
              AM: AddressingMode<S, I, M, Output = Self::Input>
    {
        let sign_clear = !cpu.registers.overflow_flag();
        branch(cpu, am, sign_clear)
    }
}

struct Bvs;

impl OpCode for Bvs {
    type Input = i8;

    fn execute<S, I, M, AM>(cpu: &mut Cpu<S, I, M>, am: AM)
        where S: Screen,
              I: Input,
              M: Memory<I, S>,
              AM: AddressingMode<S, I, M, Output = Self::Input>
    {
        let sign_clear = cpu.registers.overflow_flag();
        branch(cpu, am, sign_clear)
    }
}

struct Bcs;

impl OpCode for Bcs {
    type Input = i8;

    fn execute<S, I, M, AM>(cpu: &mut Cpu<S, I, M>, am: AM)
        where S: Screen,
              I: Input,
              M: Memory<I, S>,
              AM: AddressingMode<S, I, M, Output = Self::Input>
    {
        let carry_set = cpu.registers.carry_flag();
        branch(cpu, am, carry_set)
    }
}

struct Bne;

impl OpCode for Bne {
    type Input = i8;

    fn execute<S, I, M, AM>(cpu: &mut Cpu<S, I, M>, am: AM)
        where S: Screen,
              I: Input,
              M: Memory<I, S>,
              AM: AddressingMode<S, I, M, Output = Self::Input>
    {
        let zero_clear = !cpu.registers.zero_flag();
        branch(cpu, am, zero_clear)
    }
}

fn compare<S, I, M, AM>(cpu: &mut Cpu<S, I, M>, am: AM, lhs: u8)
    where S: Screen,
          I: Input,
          M: Memory<I, S>,
          AM: AddressingMode<S, I, M, Output = u8>
{
    let rhs = am.read();
    let res = lhs as i32 - rhs as i32;
    cpu.registers.set_carry_flag(res & 0x100 == 0);
    cpu.registers.set_sign_and_zero_flag(res as u8);
}

struct Cmp;

impl OpCode for Cmp {
    type Input = u8;

    fn execute<S, I, M, AM>(cpu: &mut Cpu<S, I, M>, am: AM)
        where S: Screen,
              I: Input,
              M: Memory<I, S>,
              AM: AddressingMode<S, I, M, Output = Self::Input>
    {
        let val = cpu.registers.acc;
        compare(cpu, am, val);
    }
}

struct Cpx;

impl OpCode for Cpx {
    type Input = u8;

    fn execute<S, I, M, AM>(cpu: &mut Cpu<S, I, M>, am: AM)
        where S: Screen,
              I: Input,
              M: Memory<I, S>,
              AM: AddressingMode<S, I, M, Output = Self::Input>
    {
        let val = cpu.registers.x;
        compare(cpu, am, val);
    }
}

struct Cpy;

impl OpCode for Cpy {
    type Input = u8;

    fn execute<S, I, M, AM>(cpu: &mut Cpu<S, I, M>, am: AM)
        where S: Screen,
              I: Input,
              M: Memory<I, S>,
              AM: AddressingMode<S, I, M, Output = Self::Input>
    {
        let val = cpu.registers.y;
        compare(cpu, am, val);
    }
}

struct Bit;

impl OpCode for Bit {
    type Input = u8;

    fn execute<S, I, M, AM>(cpu: &mut Cpu<S, I, M>, am: AM)
        where S: Screen,
              I: Input,
              M: Memory<I, S>,
              AM: AddressingMode<S, I, M, Output = Self::Input>
    {
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

    fn execute<S, I, M, AM>(cpu: &mut Cpu<S, I, M>, _: AM)
        where S: Screen,
              I: Input,
              M: Memory<I, S>,
              AM: AddressingMode<S, I, M, Output = Self::Input>
    {
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

struct Clc;

impl OpCode for Clc {
    type Input = ();

    fn execute<S, I, M, AM>(cpu: &mut Cpu<S, I, M>, _: AM)
        where S: Screen,
              I: Input,
              M: Memory<I, S>,
              AM: AddressingMode<S, I, M, Output = Self::Input>
    {
        cpu.registers.set_carry_flag(false);
        cpu.tick()
    }
}

struct Cld;

impl OpCode for Cld {
    type Input = ();

    fn execute<S, I, M, AM>(cpu: &mut Cpu<S, I, M>, _: AM)
        where S: Screen,
              I: Input,
              M: Memory<I, S>,
              AM: AddressingMode<S, I, M, Output = Self::Input>
    {
        cpu.registers.set_decimal_flag(false);
        cpu.tick()
    }
}

struct Cli;

impl OpCode for Cli {
    type Input = ();

    fn execute<S, I, M, AM>(cpu: &mut Cpu<S, I, M>, _: AM)
        where S: Screen,
              I: Input,
              M: Memory<I, S>,
              AM: AddressingMode<S, I, M, Output = Self::Input>
    {
        cpu.registers.set_interrupt_disable_flag(false);
        cpu.tick()
    }
}

struct Clv;

impl OpCode for Clv {
    type Input = ();

    fn execute<S, I, M, AM>(cpu: &mut Cpu<S, I, M>, _: AM)
        where S: Screen,
              I: Input,
              M: Memory<I, S>,
              AM: AddressingMode<S, I, M, Output = Self::Input>
    {
        cpu.registers.set_overflow_flag(false);
        cpu.tick()
    }
}

struct Dec;

impl OpCode for Dec {
    type Input = u8;

    fn execute<S, I, M, AM>(cpu: &mut Cpu<S, I, M>, am: AM)
        where S: Screen,
              I: Input,
              M: Memory<I, S>,
              AM: AddressingMode<S, I, M, Output = Self::Input>
    {
        let val = wrapping_dec(am.read());
        am.write(cpu, val);
        cpu.registers.set_sign_and_zero_flag(val);
    }
}

struct Dex;

impl OpCode for Dex {
    type Input = ();

    fn execute<S, I, M, AM>(cpu: &mut Cpu<S, I, M>, _: AM)
        where S: Screen,
              I: Input,
              M: Memory<I, S>,
              AM: AddressingMode<S, I, M, Output = Self::Input>
    {
        let val = wrapping_dec(cpu.registers.x);
        cpu.registers.x = val;
        cpu.registers.set_sign_and_zero_flag(val);
        cpu.tick()
    }
}

struct Dey;

impl OpCode for Dey {
    type Input = ();

    fn execute<S, I, M, AM>(cpu: &mut Cpu<S, I, M>, _: AM)
        where S: Screen,
              I: Input,
              M: Memory<I, S>,
              AM: AddressingMode<S, I, M, Output = Self::Input>
    {
        let val = wrapping_dec(cpu.registers.y);
        cpu.registers.y = val;
        cpu.registers.set_sign_and_zero_flag(val);
        cpu.tick()
    }
}

struct Eor;

impl OpCode for Eor {
    type Input = u8;

    fn execute<S, I, M, AM>(cpu: &mut Cpu<S, I, M>, am: AM)
        where S: Screen,
              I: Input,
              M: Memory<I, S>,
              AM: AddressingMode<S, I, M, Output = Self::Input>
    {
        let rhs = am.read();
        let lhs = cpu.registers.acc;
        let res = lhs ^ rhs;
        cpu.registers.set_acc(res);
    }
}

struct Inc;

impl OpCode for Inc {
    type Input = u8;

    fn execute<S, I, M, AM>(cpu: &mut Cpu<S, I, M>, am: AM)
        where S: Screen,
              I: Input,
              M: Memory<I, S>,
              AM: AddressingMode<S, I, M, Output = Self::Input>
    {
        let val = wrapping_inc(am.read());
        am.write(cpu, val);
        cpu.registers.set_sign_and_zero_flag(val);
    }
}

struct Inx;

impl OpCode for Inx {
    type Input = ();

    fn execute<S, I, M, AM>(cpu: &mut Cpu<S, I, M>, _: AM)
        where S: Screen,
              I: Input,
              M: Memory<I, S>,
              AM: AddressingMode<S, I, M, Output = Self::Input>
    {
        let val = wrapping_inc(cpu.registers.x);
        cpu.registers.x = val;
        cpu.registers.set_sign_and_zero_flag(val);
        cpu.tick()
    }
}

struct Iny;

impl OpCode for Iny {
    type Input = ();

    fn execute<S, I, M, AM>(cpu: &mut Cpu<S, I, M>, _: AM)
        where S: Screen,
              I: Input,
              M: Memory<I, S>,
              AM: AddressingMode<S, I, M, Output = Self::Input>
    {
        let val = wrapping_inc(cpu.registers.y);
        cpu.registers.y = val;
        cpu.registers.set_sign_and_zero_flag(val);
        cpu.tick()
    }
}

struct Jmp;

impl OpCode for Jmp {
    type Input = u16;

    fn execute<S, I, M, AM>(cpu: &mut Cpu<S, I, M>, am: AM)
        where S: Screen,
              I: Input,
              M: Memory<I, S>,
              AM: AddressingMode<S, I, M, Output = Self::Input>
    {
        cpu.registers.pc = am.read();
    }
}

struct Jsr;

impl OpCode for Jsr {
    type Input = u16;

    fn execute<S, I, M, AM>(cpu: &mut Cpu<S, I, M>, am: AM)
        where S: Screen,
              I: Input,
              M: Memory<I, S>,
              AM: AddressingMode<S, I, M, Output = Self::Input>
    {
        let loc = am.read();
        let pc = cpu.registers.pc;
        cpu.push_stack16(pc - 1);
        cpu.registers.pc = loc;
        cpu.tick()
    }
}

struct Lda;

impl OpCode for Lda {
    type Input = u8;

    fn execute<S, I, M, AM>(cpu: &mut Cpu<S, I, M>, am: AM)
        where S: Screen,
              I: Input,
              M: Memory<I, S>,
              AM: AddressingMode<S, I, M, Output = Self::Input>
    {
        let val = am.read();
        cpu.registers.set_acc(val);
    }
}

struct Ldx;

impl OpCode for Ldx {
    type Input = u8;

    fn execute<S, I, M, AM>(cpu: &mut Cpu<S, I, M>, am: AM)
        where S: Screen,
              I: Input,
              M: Memory<I, S>,
              AM: AddressingMode<S, I, M, Output = Self::Input>
    {
        let val = am.read();
        cpu.registers.x = val;
        cpu.registers.set_sign_and_zero_flag(val);
    }
}

struct Ldy;

impl OpCode for Ldy {
    type Input = u8;

    fn execute<S, I, M, AM>(cpu: &mut Cpu<S, I, M>, am: AM)
        where S: Screen,
              I: Input,
              M: Memory<I, S>,
              AM: AddressingMode<S, I, M, Output = Self::Input>
    {
        let val = am.read();
        cpu.registers.y = val;
        cpu.registers.set_sign_and_zero_flag(val);
    }
}

struct Nop;

impl OpCode for Nop {
    type Input = ();

    fn execute<S, I, M, AM>(cpu: &mut Cpu<S, I, M>, _: AM)
        where S: Screen,
              I: Input,
              M: Memory<I, S>,
              AM: AddressingMode<S, I, M, Output = Self::Input>
    {
        cpu.tick()
    }
}

struct Ora;

impl OpCode for Ora {
    type Input = u8;

    fn execute<S, I, M, AM>(cpu: &mut Cpu<S, I, M>, am: AM)
        where S: Screen,
              I: Input,
              M: Memory<I, S>,
              AM: AddressingMode<S, I, M, Output = Self::Input>
    {
        let lhs = cpu.registers.acc;
        let rhs = am.read();
        let res = lhs | rhs;
        cpu.registers.set_acc(res);
    }
}

struct Pha;

impl OpCode for Pha {
    type Input = ();

    fn execute<S, I, M, AM>(cpu: &mut Cpu<S, I, M>, _: AM)
        where S: Screen,
              I: Input,
              M: Memory<I, S>,
              AM: AddressingMode<S, I, M, Output = Self::Input>
    {
        // Dummy read
        cpu.tick();

        let acc = cpu.registers.acc;
        cpu.push_stack(acc)
    }
}

struct Php;

impl OpCode for Php {
    type Input = ();

    fn execute<S, I, M, AM>(cpu: &mut Cpu<S, I, M>, _: AM)
        where S: Screen,
              I: Input,
              M: Memory<I, S>,
              AM: AddressingMode<S, I, M, Output = Self::Input>
    {
        // Dummy read
        cpu.tick();

        let stat = cpu.registers.status;
        cpu.push_stack(stat)
    }
}

struct Pla;

impl OpCode for Pla {
    type Input = ();

    fn execute<S, I, M, AM>(cpu: &mut Cpu<S, I, M>, _: AM)
        where S: Screen,
              I: Input,
              M: Memory<I, S>,
              AM: AddressingMode<S, I, M, Output = Self::Input>
    {
        // Dummy read
        cpu.tick();

        // Stack pointer inc cycle
        cpu.tick();

        let val = cpu.pop_stack();
        cpu.registers.set_acc(val);
    }
}

struct Plp;

impl OpCode for Plp {
    type Input = ();

    fn execute<S, I, M, AM>(cpu: &mut Cpu<S, I, M>, _: AM)
        where S: Screen,
              I: Input,
              M: Memory<I, S>,
              AM: AddressingMode<S, I, M, Output = Self::Input>
    {
        // Dummy read
        cpu.tick();

        // Stack pointer inc cycle
        cpu.tick();

        let val = cpu.pop_stack();
        cpu.registers.set_status_from_stack(val);

    }
}

struct Rti;

impl OpCode for Rti {
    type Input = ();

    fn execute<S, I, M, AM>(cpu: &mut Cpu<S, I, M>, _: AM)
        where S: Screen,
              I: Input,
              M: Memory<I, S>,
              AM: AddressingMode<S, I, M, Output = Self::Input>
    {
        // Dummy read cycle
        cpu.tick();

        // Increment stack pointer cycle
        cpu.tick();

        let stat = cpu.pop_stack();
        let pc = cpu.pop_stack16();
        cpu.registers.set_status_from_stack(stat);
        cpu.registers.pc = pc;

    }
}

struct Rts;

impl OpCode for Rts {
    type Input = ();

    fn execute<S, I, M, AM>(cpu: &mut Cpu<S, I, M>, _: AM)
        where S: Screen,
              I: Input,
              M: Memory<I, S>,
              AM: AddressingMode<S, I, M, Output = Self::Input>
    {
        // Dummy read cycle
        cpu.tick();

        // Stack increment cycle
        cpu.tick();

        let pc = cpu.pop_stack16();
        cpu.registers.pc = pc + 1;

        // increment PC cycle
        cpu.tick()
    }
}

struct Sec;

impl OpCode for Sec {
    type Input = ();

    fn execute<S, I, M, AM>(cpu: &mut Cpu<S, I, M>, _: AM)
        where S: Screen,
              I: Input,
              M: Memory<I, S>,
              AM: AddressingMode<S, I, M, Output = Self::Input>
    {
        cpu.registers.set_carry_flag(true);
        cpu.tick()
    }
}

struct Sed;

impl OpCode for Sed {
    type Input = ();

    fn execute<S, I, M, AM>(cpu: &mut Cpu<S, I, M>, _: AM)
        where S: Screen,
              I: Input,
              M: Memory<I, S>,
              AM: AddressingMode<S, I, M, Output = Self::Input>
    {
        cpu.registers.set_decimal_flag(true);
        cpu.tick()
    }
}

struct Sei;

impl OpCode for Sei {
    type Input = ();

    fn execute<S, I, M, AM>(cpu: &mut Cpu<S, I, M>, _: AM)
        where S: Screen,
              I: Input,
              M: Memory<I, S>,
              AM: AddressingMode<S, I, M, Output = Self::Input>
    {
        cpu.registers.set_interrupt_disable_flag(true);
        cpu.tick()
    }
}

struct Sta;

impl OpCode for Sta {
    // TODO: STA doesn't actually have an input
    // Is there a compelling reason to have write-only addressing implementations?
    type Input = u8;

    fn execute<S, I, M, AM>(cpu: &mut Cpu<S, I, M>, am: AM)
        where S: Screen,
              I: Input,
              M: Memory<I, S>,
              AM: AddressingMode<S, I, M, Output = Self::Input>
    {
        let acc = cpu.registers.acc;
        am.write(cpu, acc)
    }
}

struct Stx;

impl OpCode for Stx {
    // TODO: STX doesn't actually have an input
    // Is there a compelling reason to have write-only addressing implementations?
    type Input = u8;

    fn execute<S, I, M, AM>(cpu: &mut Cpu<S, I, M>, am: AM)
        where S: Screen,
              I: Input,
              M: Memory<I, S>,
              AM: AddressingMode<S, I, M, Output = Self::Input>
    {
        let x = cpu.registers.x;
        am.write(cpu, x)
    }
}

struct Sty;

impl OpCode for Sty {
    // TODO: STY doesn't actually have an input
    // Is there a compelling reason to have write-only addressing implementations?
    type Input = u8;

    fn execute<S, I, M, AM>(cpu: &mut Cpu<S, I, M>, am: AM)
        where S: Screen,
              I: Input,
              M: Memory<I, S>,
              AM: AddressingMode<S, I, M, Output = Self::Input>
    {
        let y = cpu.registers.y;
        am.write(cpu, y)
    }
}

struct Tax;

impl OpCode for Tax {
    type Input = ();

    fn execute<S, I, M, AM>(cpu: &mut Cpu<S, I, M>, _: AM)
        where S: Screen,
              I: Input,
              M: Memory<I, S>,
              AM: AddressingMode<S, I, M, Output = Self::Input>
    {
        cpu.registers.x = cpu.registers.acc;
        let x = cpu.registers.x;
        cpu.registers.set_sign_and_zero_flag(x);
        cpu.tick()
    }
}

struct Tay;

impl OpCode for Tay {
    type Input = ();

    fn execute<S, I, M, AM>(cpu: &mut Cpu<S, I, M>, _: AM)
        where S: Screen,
              I: Input,
              M: Memory<I, S>,
              AM: AddressingMode<S, I, M, Output = Self::Input>
    {
        cpu.registers.y = cpu.registers.acc;
        let y = cpu.registers.y;
        cpu.registers.set_sign_and_zero_flag(y);
        cpu.tick()
    }
}

struct Tsx;

impl OpCode for Tsx {
    type Input = ();

    fn execute<S, I, M, AM>(cpu: &mut Cpu<S, I, M>, _: AM)
        where S: Screen,
              I: Input,
              M: Memory<I, S>,
              AM: AddressingMode<S, I, M, Output = Self::Input>
    {
        cpu.registers.x = cpu.registers.sp;
        let x = cpu.registers.x;
        cpu.registers.set_sign_and_zero_flag(x);
        cpu.tick()
    }
}

struct Txa;

impl OpCode for Txa {
    type Input = ();

    fn execute<S, I, M, AM>(cpu: &mut Cpu<S, I, M>, _: AM)
        where S: Screen,
              I: Input,
              M: Memory<I, S>,
              AM: AddressingMode<S, I, M, Output = Self::Input>
    {
        cpu.registers.acc = cpu.registers.x;
        let acc = cpu.registers.acc;
        cpu.registers.set_sign_and_zero_flag(acc);
        cpu.tick()
    }
}

struct Txs;

impl OpCode for Txs {
    type Input = ();

    fn execute<S, I, M, AM>(cpu: &mut Cpu<S, I, M>, _: AM)
        where S: Screen,
              I: Input,
              M: Memory<I, S>,
              AM: AddressingMode<S, I, M, Output = Self::Input>
    {
        cpu.registers.sp = cpu.registers.x;
        cpu.tick()
    }
}

struct Tya;

impl OpCode for Tya {
    type Input = ();

    fn execute<S, I, M, AM>(cpu: &mut Cpu<S, I, M>, _: AM)
        where S: Screen,
              I: Input,
              M: Memory<I, S>,
              AM: AddressingMode<S, I, M, Output = Self::Input>
    {
        cpu.registers.acc = cpu.registers.y;
        let acc = cpu.registers.acc;
        cpu.registers.set_sign_and_zero_flag(acc);
        cpu.tick()
    }
}
