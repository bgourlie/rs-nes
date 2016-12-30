pub mod adc;

use cpu::Cpu;
use memory::Memory;
use cpu::execution_context::ExecutionContext;

pub trait Instruction<M: Memory> {
    fn execute<EC: ExecutionContext<M>, F: Fn(&Cpu<M>)>(self,
                                                        cpu: &mut Cpu<M>,
                                                        context: EC,
                                                        tick_handler: F);
}

#[derive(Copy, Clone)]
pub enum AddressingMode {
    Implied,
    Accumulator,
    Immediate,
    ZeroPage,
    Absolute,
    Relative,
    ZeroPageX,
    ZeroPageY,
    AbsoluteX,
    AbsoluteY,
    IndexedIndirect,
    IndirectIndexed,
    Indirect,
}

#[derive(Copy, Clone)]
pub enum OpCode {
    Adc,
    And,
    Asl,
    Bcc,
    Bcs,
    Beq,
    Bit,
    Bmi,
    Bne,
    Bpl,
    Brk,
    Bvc,
    Bvs,
    Clc,
    Cld,
    Cli,
    Clv,
    Cmp,
    Cpx,
    Cpy,
    Dec,
    Dex,
    Dey,
    Eor,
    Inc,
    Inx,
    Iny,
    Jmp,
    Jsr,
    Lda,
    Ldx,
    Ldy,
    Lsr,
    Nop,
    Ora,
    Pha,
    Php,
    Pla,
    Plp,
    Rol,
    Ror,
    Rti,
    Rts,
    Sbc,
    Sec,
    Sed,
    Sei,
    Sta,
    Stx,
    Sty,
    Tax,
    Tay,
    Tsx,
    Txa,
    Txs,
    Tya,
}

pub fn decode(byte: u8) -> (OpCode, AddressingMode) {
    match byte {
        0xe8 => (OpCode::Inx, AddressingMode::Implied),
        0xca => (OpCode::Dex, AddressingMode::Implied),
        0xc8 => (OpCode::Iny, AddressingMode::Implied),
        0x88 => (OpCode::Dey, AddressingMode::Implied),
        0xaa => (OpCode::Tax, AddressingMode::Implied),
        0xa8 => (OpCode::Tay, AddressingMode::Implied),
        0x8a => (OpCode::Txa, AddressingMode::Implied),
        0x98 => (OpCode::Tya, AddressingMode::Implied),
        0x9a => (OpCode::Txs, AddressingMode::Implied),
        0xba => (OpCode::Tsx, AddressingMode::Implied),
        0x18 => (OpCode::Clc, AddressingMode::Implied),
        0x38 => (OpCode::Sec, AddressingMode::Implied),
        0x58 => (OpCode::Cli, AddressingMode::Implied),
        0x78 => (OpCode::Sei, AddressingMode::Implied),
        0xb8 => (OpCode::Clv, AddressingMode::Implied),
        0xd8 => (OpCode::Cld, AddressingMode::Implied),
        0xf8 => (OpCode::Sed, AddressingMode::Implied),
        0x60 => (OpCode::Rts, AddressingMode::Implied),
        0x00 => (OpCode::Brk, AddressingMode::Implied),
        0x40 => (OpCode::Rti, AddressingMode::Implied),
        0x48 => (OpCode::Pha, AddressingMode::Implied),
        0x68 => (OpCode::Pla, AddressingMode::Implied),
        0x08 => (OpCode::Php, AddressingMode::Implied),
        0x28 => (OpCode::Plp, AddressingMode::Implied),
        0xea => (OpCode::Nop, AddressingMode::Implied),
        0x10 => (OpCode::Bpl, AddressingMode::Relative),
        0x30 => (OpCode::Bmi, AddressingMode::Relative),
        0x50 => (OpCode::Bvc, AddressingMode::Relative),
        0x70 => (OpCode::Bvs, AddressingMode::Relative),
        0x90 => (OpCode::Bcc, AddressingMode::Relative),
        0xb0 => (OpCode::Bcs, AddressingMode::Relative),
        0xd0 => (OpCode::Bne, AddressingMode::Relative),
        0xf0 => (OpCode::Beq, AddressingMode::Relative),
        0xa1 => (OpCode::Lda, AddressingMode::IndexedIndirect),
        0xa5 => (OpCode::Lda, AddressingMode::ZeroPage),
        0xa9 => (OpCode::Lda, AddressingMode::Immediate),
        0xb1 => (OpCode::Lda, AddressingMode::IndirectIndexed),
        0xb5 => (OpCode::Lda, AddressingMode::ZeroPageX),
        0xad => (OpCode::Lda, AddressingMode::Absolute),
        0xb9 => (OpCode::Lda, AddressingMode::AbsoluteY),
        0xbd => (OpCode::Lda, AddressingMode::AbsoluteX),
        0xa2 => (OpCode::Ldx, AddressingMode::Immediate),
        0xa6 => (OpCode::Ldx, AddressingMode::ZeroPage),
        0xb6 => (OpCode::Ldx, AddressingMode::ZeroPageY),
        0xae => (OpCode::Ldx, AddressingMode::Absolute),
        0xbe => (OpCode::Ldx, AddressingMode::AbsoluteY),
        0xa0 => (OpCode::Ldy, AddressingMode::Immediate),
        0xa4 => (OpCode::Ldy, AddressingMode::ZeroPage),
        0xb4 => (OpCode::Ldy, AddressingMode::ZeroPageX),
        0xac => (OpCode::Ldy, AddressingMode::Absolute),
        0xbc => (OpCode::Ldy, AddressingMode::AbsoluteX),
        0x85 => (OpCode::Sta, AddressingMode::ZeroPage),
        0x95 => (OpCode::Sta, AddressingMode::ZeroPageX),
        0x81 => (OpCode::Sta, AddressingMode::IndexedIndirect),
        0x91 => (OpCode::Sta, AddressingMode::IndirectIndexed),
        0x8d => (OpCode::Sta, AddressingMode::Absolute),
        0x9d => (OpCode::Sta, AddressingMode::AbsoluteX),
        0x99 => (OpCode::Sta, AddressingMode::AbsoluteY),
        0x86 => (OpCode::Stx, AddressingMode::ZeroPage),
        0x96 => (OpCode::Stx, AddressingMode::ZeroPageY),
        0x8e => (OpCode::Stx, AddressingMode::Absolute),
        0x84 => (OpCode::Sty, AddressingMode::ZeroPage),
        0x94 => (OpCode::Sty, AddressingMode::ZeroPageX),
        0x8c => (OpCode::Sty, AddressingMode::Absolute),
        0x69 => (OpCode::Adc, AddressingMode::Immediate),
        0x65 => (OpCode::Adc, AddressingMode::ZeroPage),
        0x75 => (OpCode::Adc, AddressingMode::ZeroPageX),
        0x61 => (OpCode::Adc, AddressingMode::IndexedIndirect),
        0x71 => (OpCode::Adc, AddressingMode::IndirectIndexed),
        0x6d => (OpCode::Adc, AddressingMode::Absolute),
        0x7d => (OpCode::Adc, AddressingMode::AbsoluteX),
        0x79 => (OpCode::Adc, AddressingMode::AbsoluteY),
        0xe9 => (OpCode::Sbc, AddressingMode::Immediate),
        0xe5 => (OpCode::Sbc, AddressingMode::ZeroPage),
        0xf5 => (OpCode::Sbc, AddressingMode::ZeroPageX),
        0xe1 => (OpCode::Sbc, AddressingMode::IndexedIndirect),
        0xf1 => (OpCode::Sbc, AddressingMode::IndirectIndexed),
        0xed => (OpCode::Sbc, AddressingMode::Absolute),
        0xfd => (OpCode::Sbc, AddressingMode::AbsoluteX),
        0xf9 => (OpCode::Sbc, AddressingMode::AbsoluteY),
        0xc9 => (OpCode::Cmp, AddressingMode::Immediate),
        0xc5 => (OpCode::Cmp, AddressingMode::ZeroPage),
        0xd5 => (OpCode::Cmp, AddressingMode::ZeroPageX),
        0xc1 => (OpCode::Cmp, AddressingMode::IndexedIndirect),
        0xd1 => (OpCode::Cmp, AddressingMode::IndirectIndexed),
        0xcd => (OpCode::Cmp, AddressingMode::Absolute),
        0xdd => (OpCode::Cmp, AddressingMode::AbsoluteX),
        0xd9 => (OpCode::Cmp, AddressingMode::AbsoluteY),
        0xe0 => (OpCode::Cpx, AddressingMode::Immediate),
        0xe4 => (OpCode::Cpx, AddressingMode::ZeroPage),
        0xec => (OpCode::Cpx, AddressingMode::Absolute),
        0xc0 => (OpCode::Cpy, AddressingMode::Immediate),
        0xc4 => (OpCode::Cpy, AddressingMode::ZeroPage),
        0xcc => (OpCode::Cpy, AddressingMode::Absolute),
        0x29 => (OpCode::And, AddressingMode::Immediate),
        0x25 => (OpCode::And, AddressingMode::ZeroPage),
        0x35 => (OpCode::And, AddressingMode::ZeroPageX),
        0x21 => (OpCode::And, AddressingMode::IndexedIndirect),
        0x31 => (OpCode::And, AddressingMode::IndirectIndexed),
        0x2d => (OpCode::And, AddressingMode::Absolute),
        0x3d => (OpCode::And, AddressingMode::AbsoluteX),
        0x39 => (OpCode::And, AddressingMode::AbsoluteY),
        0x09 => (OpCode::Ora, AddressingMode::Immediate),
        0x05 => (OpCode::Ora, AddressingMode::ZeroPage),
        0x15 => (OpCode::Ora, AddressingMode::ZeroPageX),
        0x01 => (OpCode::Ora, AddressingMode::IndexedIndirect),
        0x11 => (OpCode::Ora, AddressingMode::IndirectIndexed),
        0x0d => (OpCode::Ora, AddressingMode::Absolute),
        0x1d => (OpCode::Ora, AddressingMode::AbsoluteX),
        0x19 => (OpCode::Ora, AddressingMode::AbsoluteY),
        0x49 => (OpCode::Eor, AddressingMode::Immediate),
        0x45 => (OpCode::Eor, AddressingMode::ZeroPage),
        0x55 => (OpCode::Eor, AddressingMode::ZeroPageX),
        0x41 => (OpCode::Eor, AddressingMode::IndexedIndirect),
        0x51 => (OpCode::Eor, AddressingMode::IndirectIndexed),
        0x4d => (OpCode::Eor, AddressingMode::Absolute),
        0x5d => (OpCode::Eor, AddressingMode::AbsoluteX),
        0x59 => (OpCode::Eor, AddressingMode::AbsoluteY),
        0x24 => (OpCode::Bit, AddressingMode::ZeroPage),
        0x2c => (OpCode::Bit, AddressingMode::Absolute),
        0x2a => (OpCode::Rol, AddressingMode::Accumulator),
        0x26 => (OpCode::Rol, AddressingMode::ZeroPage),
        0x36 => (OpCode::Rol, AddressingMode::ZeroPageX),
        0x2e => (OpCode::Rol, AddressingMode::Absolute),
        0x3e => (OpCode::Rol, AddressingMode::AbsoluteX),
        0x6a => (OpCode::Ror, AddressingMode::Accumulator),
        0x66 => (OpCode::Ror, AddressingMode::ZeroPage),
        0x76 => (OpCode::Ror, AddressingMode::ZeroPageX),
        0x6e => (OpCode::Ror, AddressingMode::Absolute),
        0x7e => (OpCode::Ror, AddressingMode::AbsoluteX),
        0x0a => (OpCode::Asl, AddressingMode::Accumulator),
        0x06 => (OpCode::Asl, AddressingMode::ZeroPage),
        0x16 => (OpCode::Asl, AddressingMode::ZeroPageX),
        0x0e => (OpCode::Asl, AddressingMode::Absolute),
        0x1e => (OpCode::Asl, AddressingMode::AbsoluteX),
        0x4a => (OpCode::Lsr, AddressingMode::Accumulator),
        0x46 => (OpCode::Lsr, AddressingMode::ZeroPage),
        0x56 => (OpCode::Lsr, AddressingMode::ZeroPageX),
        0x4e => (OpCode::Lsr, AddressingMode::Absolute),
        0x5e => (OpCode::Lsr, AddressingMode::AbsoluteX),
        0xe6 => (OpCode::Inc, AddressingMode::ZeroPage),
        0xf6 => (OpCode::Inc, AddressingMode::ZeroPageX),
        0xee => (OpCode::Inc, AddressingMode::Absolute),
        0xfe => (OpCode::Inc, AddressingMode::AbsoluteX),
        0xc6 => (OpCode::Dec, AddressingMode::ZeroPage),
        0xd6 => (OpCode::Dec, AddressingMode::ZeroPageX),
        0xce => (OpCode::Dec, AddressingMode::Absolute),
        0xde => (OpCode::Dec, AddressingMode::AbsoluteX),
        0x4c => (OpCode::Jmp, AddressingMode::Absolute),
        0x6c => (OpCode::Jmp, AddressingMode::Indirect),
        0x20 => (OpCode::Jsr, AddressingMode::Absolute),
        _ => {
            panic!("unexpected opcode encountered");
        }
    }
}
