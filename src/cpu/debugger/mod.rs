#[derive(Debug)]
pub enum Opcode {
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

pub enum AddressingMode {
    Accumulator,
    Immediate(u8),
    Implied,
    Relative(u8),
    Absolute(u16),
    ZeroPage(u8),
    Indirect(u16),
    AbsoluteIndexedX(u16),
    AbsoluteIndexedY(u16),
    ZeroPageIndexedX(u8),
    ZeroPageIndexedY(u8),
    IndexedIndirect(u8),
    IndirectIndexed(u8),
}

pub struct Instruction {
    op_raw: u8,
    opcode: Opcode,
    addressing_mode: AddressingMode,
}

impl ToString for Instruction {
    fn to_string(&self) -> String {
        match self.addressing_mode {
            AddressingMode::Accumulator => {
                format!("{:0>2X} {:?} A         ", self.op_raw, self.opcode)
            }
            AddressingMode::Immediate(val) => {
                format!("{:0>2X} {:?} #${:0>2X}    ", self.op_raw, self.opcode, val)
            }
            AddressingMode::Implied => format!("{:0>2X} {:?}         ", self.op_raw, self.opcode),
            AddressingMode::Relative(val) => {
                format!("{:0>2X} {:?} {:0>3}     ",
                        self.op_raw,
                        self.opcode,
                        val as i8)
            }
            AddressingMode::Absolute(val) => {
                format!("{:0>2X} {:?} ${:0>4X}   ", self.op_raw, self.opcode, val)
            }
            AddressingMode::ZeroPage(val) => {
                format!("{:0>2X} {:?} ${:0>2X}     ", self.op_raw, self.opcode, val)
            }
            AddressingMode::Indirect(val) => {
                format!("{:0>2X} {:?} (${:0>4X}) ", self.op_raw, self.opcode, val)
            }
            AddressingMode::AbsoluteIndexedX(val) => {
                format!("{:0>2X} {:?} ${:0>4X},X ", self.op_raw, self.opcode, val)
            }
            AddressingMode::AbsoluteIndexedY(val) => {
                format!("{:0>2X} {:?} ${:0>4X},Y ", self.op_raw, self.opcode, val)
            }
            AddressingMode::ZeroPageIndexedX(val) => {
                format!("{:0>2X} {:?} ${:0>2X},X   ", self.op_raw, self.opcode, val)
            }
            AddressingMode::ZeroPageIndexedY(val) => {
                format!("{:0>2X} {:?} ${:0>2X},Y   ", self.op_raw, self.opcode, val)
            }
            AddressingMode::IndexedIndirect(val) => {
                format!("{:0>2X} {:?} (${:0>2X},X) ", self.op_raw, self.opcode, val)
            }
            AddressingMode::IndirectIndexed(val) => {
                format!("{:0>2X} {:?} (${:0>2X},Y) ", self.op_raw, self.opcode, val)
            }
        }
    }
}

impl Instruction {
    pub fn new1(opcode: u8) -> Instruction {
        let op = Instruction::get_opcode(opcode);
        let am = Instruction::get_addressing_mode1(opcode);
        Instruction {
            op_raw: opcode,
            opcode: op,
            addressing_mode: am,
        }
    }

    pub fn new2(opcode: u8, operand: u8) -> Instruction {
        let op = Instruction::get_opcode(opcode);
        let am = Instruction::get_addressing_mode2(opcode, operand);
        Instruction {
            op_raw: opcode,
            opcode: op,
            addressing_mode: am,
        }
    }

    pub fn new3(opcode: u8, operand: u16) -> Instruction {
        let op = Instruction::get_opcode(opcode);
        let am = Instruction::get_addressing_mode3(opcode, operand);
        Instruction {
            op_raw: opcode,
            opcode: op,
            addressing_mode: am,
        }
    }

    fn get_addressing_mode1(opcode: u8) -> AddressingMode {
        match opcode {
            0x0a | 0x4a | 0x2a | 0x6a => AddressingMode::Accumulator,

            0x00 | 0x18 | 0xd8 | 0x58 | 0xb8 | 0xca | 0x88 | 0xe8 | 0xc8 | 0xea | 0x48 | 0x08 |
            0x68 | 0x28 | 0x40 | 0x60 | 0x38 | 0xf8 | 0x78 | 0xaa | 0xa8 | 0xba | 0x8a | 0x9a |
            0x98 => AddressingMode::Implied,

            _ => {
                panic!("unexpected 1-byte instruction encountered.");
            }
        }
    }

    fn get_addressing_mode2(opcode: u8, operand: u8) -> AddressingMode {
        match opcode {
            0x65 | 0x25 | 0x06 | 0x24 | 0xc5 | 0xe4 | 0xc4 | 0xc6 | 0x45 | 0x55 | 0xe6 | 0xa5 |
            0xa6 | 0xa4 | 0x46 | 0x05 | 0x26 | 0x66 | 0xe5 | 0x85 | 0x86 | 0x84 => {
                AddressingMode::ZeroPage(operand)
            }

            0x75 | 0x35 | 0x16 | 0xd5 | 0xd6 | 0xf6 | 0xb5 | 0xb4 | 0x56 | 0x15 | 0x36 | 0x76 |
            0xf5 | 0x95 | 0x94 => AddressingMode::ZeroPageIndexedX(operand),

            0x69 | 0x29 | 0xc9 | 0xe0 | 0xc0 | 0x49 | 0xa9 | 0xa2 | 0xa0 | 0x09 | 0xe9 => {
                AddressingMode::Immediate(operand)
            }

            0x61 | 0x21 | 0xc1 | 0x41 | 0xa1 | 0x01 | 0xe1 | 0x81 => {
                AddressingMode::IndexedIndirect(operand)
            }

            0x71 | 0x31 | 0xd1 | 0x51 | 0xb1 | 0x11 | 0xf1 | 0x91 => {
                AddressingMode::IndirectIndexed(operand)
            }

            0x90 | 0xb0 | 0xf0 | 0x30 | 0xd0 | 0x10 | 0x50 | 0x70 => {
                AddressingMode::Relative(operand)
            }

            0xb6 | 0x96 => AddressingMode::ZeroPageIndexedY(operand),

            _ => {
                panic!("unexpected 2-byte instruction encountered.");
            }
        }
    }

    fn get_addressing_mode3(opcode: u8, operand: u16) -> AddressingMode {
        match opcode {
            0x6d | 0x2d | 0x0e | 0x2c | 0xcd | 0xec | 0xcc | 0xce | 0x4d | 0xee | 0x4c | 0x20 |
            0xad | 0xae | 0xac | 0x4e | 0x0d | 0x2e | 0x6e | 0xed | 0x8d | 0x8e | 0x8c => {
                AddressingMode::Absolute(operand)
            }

            0x7d | 0x3d | 0x1e | 0xdd | 0xde | 0x5d | 0xfe | 0xbd | 0xbc | 0x5e | 0x1d | 0x3e |
            0x7e | 0xfd | 0x9d => AddressingMode::AbsoluteIndexedX(operand),

            0x79 | 0x39 | 0xd9 | 0x59 | 0xb9 | 0xbe | 0x19 | 0xf9 | 0x99 => {
                AddressingMode::AbsoluteIndexedY(operand)
            }

            0x6c => AddressingMode::Indirect(operand),

            _ => {
                panic!("unexpected 3-byte instruction encountered.");
            }
        }
    }

    fn get_opcode(opcode: u8) -> Opcode {
        match opcode {
            0xa1 | 0xa5 | 0xa9 | 0xad | 0xb1 | 0xb5 | 0xb9 | 0xbd => Opcode::Lda,
            0xa2 | 0xa6 | 0xb6 | 0xae | 0xbe => Opcode::Ldx,
            0xa0 | 0xa4 | 0xb4 | 0xac | 0xbc => Opcode::Ldy,
            0x85 | 0x95 | 0x8d | 0x9d | 0x99 | 0x81 | 0x91 => Opcode::Sta,
            0x86 | 0x96 | 0x8e => Opcode::Stx,
            0x84 | 0x94 | 0x8c => Opcode::Sty,
            0x69 | 0x65 | 0x75 | 0x6d | 0x7d | 0x79 | 0x61 | 0x71 => Opcode::Adc,
            0xe9 | 0xe5 | 0xf5 | 0xed | 0xfd | 0xf9 | 0xe1 | 0xf1 => Opcode::Sbc,
            0xc9 | 0xc5 | 0xd5 | 0xcd | 0xdd | 0xd9 | 0xc1 | 0xd1 => Opcode::Cmp,
            0xe0 | 0xe4 | 0xec => Opcode::Cpx,
            0xc0 | 0xc4 | 0xcc => Opcode::Cpy,
            0x29 | 0x25 | 0x35 | 0x2d | 0x3d | 0x39 | 0x21 | 0x31 => Opcode::And,
            0x09 | 0x05 | 0x15 | 0x0d | 0x1d | 0x19 | 0x01 | 0x11 => Opcode::Ora,
            0x49 | 0x45 | 0x55 | 0x4d | 0x5d | 0x59 | 0x41 | 0x51 => Opcode::Eor,
            0x24 | 0x2c => Opcode::Bit,
            0x2a | 0x26 | 0x36 | 0x2e | 0x3e => Opcode::Rol,
            0x6a | 0x66 | 0x76 | 0x6e | 0x7e => Opcode::Ror,
            0x0a | 0x06 | 0x16 | 0x0e | 0x1e => Opcode::Asl,
            0x4a | 0x46 | 0x56 | 0x4e | 0x5e => Opcode::Lsr,
            0xe6 | 0xf6 | 0xee | 0xfe => Opcode::Inc,
            0xc6 | 0xd6 | 0xce | 0xde => Opcode::Dec,
            0xe8 => Opcode::Inx,
            0xca => Opcode::Dex,
            0xc8 => Opcode::Iny,
            0x88 => Opcode::Dey,
            0xaa => Opcode::Tax,
            0xa8 => Opcode::Tay,
            0x8a => Opcode::Txa,
            0x98 => Opcode::Tya,
            0x9a => Opcode::Txs,
            0xba => Opcode::Tsx,
            0x18 => Opcode::Clc,
            0x38 => Opcode::Sec,
            0x58 => Opcode::Cli,
            0x78 => Opcode::Sei,
            0xb8 => Opcode::Clv,
            0xd8 => Opcode::Cld,
            0xf8 => Opcode::Sed,
            0x10 => Opcode::Bpl,
            0x30 => Opcode::Bmi,
            0x50 => Opcode::Bvc,
            0x70 => Opcode::Bvs,
            0x90 => Opcode::Bcc,
            0xb0 => Opcode::Bcs,
            0xd0 => Opcode::Bne,
            0xf0 => Opcode::Beq,
            0x4c | 0x6c => Opcode::Jmp,
            0x20 => Opcode::Jsr,
            0x60 => Opcode::Rts,
            0x00 => Opcode::Brk,
            0x40 => Opcode::Rti,
            0x48 => Opcode::Pha,
            0x68 => Opcode::Pla,
            0x08 => Opcode::Php,
            0x28 => Opcode::Plp,
            0xea => Opcode::Nop,
            _ => {
                panic!("unexpected opcode encountered");
            }
        }
    }
}
