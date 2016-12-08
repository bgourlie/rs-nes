use serde::{Serialize, Serializer};

static INSTR_MASK: u8 = 0b111;
static INSTR_FAMILY_MASK: u8 = 0b11;
static ADDRESSING_MODE_MASK: u8 = 0b111;


pub struct InstructionDecoder<'a> {
    bytes: &'a [u8],
    pc: usize,
    start_offset: u16,
}

#[derive(Copy, Clone)]
pub struct Instruction {
    pub offset: u16,
    pub mnemonic: Mnemonic,
    pub addressing_mode: AddressingMode,
}

impl Serialize for Instruction {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        let mut state = serializer.serialize_struct("Instruction", 3)?;
        serializer.serialize_struct_elt(&mut state, "offset", self.offset)?;
        serializer.serialize_struct_elt(&mut state, "mnemonic", self.mnemonic)?;
        serializer.serialize_struct_elt(&mut state, "addressing_mode", self.addressing_mode)?;
        serializer.serialize_struct_end(state)
    }
}

#[derive(Copy, Clone)]
pub enum AddressingMode {
    IndexedIndirect(u8),
    IndirectIndexed(u8),
    ZeroPage(u8),
    Immediate(u8),
    Absolute(u16),
    AbsoluteX(u16),
    AbsoluteY(u16),
    ZeroPageX(u8),
    Relative(i8),
    Implied,
    Accumulator,
}

impl Serialize for AddressingMode {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        let mut state = serializer.serialize_struct("AddressingMode", 2)?;
        match *self {
            AddressingMode::IndexedIndirect(addr) => {
                serializer.serialize_struct_elt(&mut state, "mode", "IndexedIndirect")?;
                serializer.serialize_struct_elt(&mut state, "operand", addr)?;
            }
            AddressingMode::IndirectIndexed(addr) => {
                serializer.serialize_struct_elt(&mut state, "mode", "IndirectIndexed")?;
                serializer.serialize_struct_elt(&mut state, "operand", addr)?;
            }
            AddressingMode::ZeroPage(addr) => {
                serializer.serialize_struct_elt(&mut state, "mode", "ZeroPage")?;
                serializer.serialize_struct_elt(&mut state, "operand", addr)?;
            }
            AddressingMode::Immediate(val) => {
                serializer.serialize_struct_elt(&mut state, "mode", "Immediate")?;
                serializer.serialize_struct_elt(&mut state, "operand", val)?;
            }
            AddressingMode::Absolute(addr) => {
                serializer.serialize_struct_elt(&mut state, "mode", "Absolute")?;
                serializer.serialize_struct_elt(&mut state, "operand", addr)?;
            }
            AddressingMode::AbsoluteX(addr) => {
                serializer.serialize_struct_elt(&mut state, "mode", "AbsoluteX")?;
                serializer.serialize_struct_elt(&mut state, "operand", addr)?;
            }
            AddressingMode::AbsoluteY(addr) => {
                serializer.serialize_struct_elt(&mut state, "mode", "AbsoluteY")?;
                serializer.serialize_struct_elt(&mut state, "operand", addr)?;
            }
            AddressingMode::ZeroPageX(addr) => {
                serializer.serialize_struct_elt(&mut state, "mode", "ZeroPageX")?;
                serializer.serialize_struct_elt(&mut state, "operand", addr)?;
            }
            AddressingMode::Relative(offset) => {
                serializer.serialize_struct_elt(&mut state, "mode", "Relative")?;
                serializer.serialize_struct_elt(&mut state, "operand", offset)?;
            }
            AddressingMode::Implied => {
                serializer.serialize_struct_elt(&mut state, "mode", "Implied")?;
            }
            AddressingMode::Accumulator => {
                serializer.serialize_struct_elt(&mut state, "mode", "Accumulator")?;
            }
        }
        serializer.serialize_struct_end(state)
    }
}

#[derive(Copy, Clone, Serialize)]
pub enum Mnemonic {
    ADC,
    AND,
    ASL,
    BCC,
    BCS,
    BEQ,
    BIT,
    BMI,
    BNE,
    BPL,
    BRK,
    BVC,
    BVS,
    CLC,
    CLD,
    CLI,
    CLV,
    CMP,
    CPX,
    CPY,
    DEC,
    DEX,
    DEY,
    EOR,
    INC,
    INX,
    INY,
    JMP,
    JSR,
    LDA,
    LDX,
    LDY,
    LSR,
    NOP,
    ORA,
    PHA,
    PHP,
    PLA,
    PLP,
    ROL,
    ROR,
    RTI,
    RTS,
    SBC,
    SEC,
    SED,
    SEI,
    STA,
    STX,
    STY,
    TAX,
    TAY,
    TSX,
    TXA,
    TXS,
    TYA,
}

impl ToString for Mnemonic {
    fn to_string(&self) -> String {
        match *self {
                Mnemonic::ADC => "ADC",
                Mnemonic::AND => "AND",
                Mnemonic::ASL => "ASL",
                Mnemonic::BCC => "BCC",
                Mnemonic::BCS => "BCS",
                Mnemonic::BEQ => "BEQ",
                Mnemonic::BIT => "BIT",
                Mnemonic::BMI => "BMI",
                Mnemonic::BNE => "BNE",
                Mnemonic::BPL => "BPL",
                Mnemonic::BRK => "BRK",
                Mnemonic::BVC => "BVC",
                Mnemonic::BVS => "BVS",
                Mnemonic::CLC => "CLC",
                Mnemonic::CLD => "CLD",
                Mnemonic::CLI => "CLI",
                Mnemonic::CLV => "CLV",
                Mnemonic::CMP => "CMP",
                Mnemonic::CPX => "CPX",
                Mnemonic::CPY => "CPU",
                Mnemonic::DEC => "DEC",
                Mnemonic::DEX => "DEX",
                Mnemonic::DEY => "DEY",
                Mnemonic::EOR => "EOR",
                Mnemonic::INC => "INC",
                Mnemonic::INX => "INX",
                Mnemonic::INY => "INY",
                Mnemonic::JMP => "JMP",
                Mnemonic::JSR => "JSR",
                Mnemonic::LDA => "LDA",
                Mnemonic::LDX => "LDX",
                Mnemonic::LDY => "LDY",
                Mnemonic::LSR => "LSR",
                Mnemonic::NOP => "NOP",
                Mnemonic::ORA => "ORA",
                Mnemonic::PHA => "PHA",
                Mnemonic::PHP => "PHP",
                Mnemonic::PLA => "PLA",
                Mnemonic::PLP => "PLP",
                Mnemonic::ROL => "ROL",
                Mnemonic::ROR => "ROR",
                Mnemonic::RTI => "RTI",
                Mnemonic::RTS => "RTS",
                Mnemonic::SBC => "SBC",
                Mnemonic::SEC => "SEC",
                Mnemonic::SED => "SED",
                Mnemonic::SEI => "SEI",
                Mnemonic::STA => "STA",
                Mnemonic::STX => "STX",
                Mnemonic::STY => "STY",
                Mnemonic::TAX => "TAX",
                Mnemonic::TAY => "TAY",
                Mnemonic::TSX => "TSX",
                Mnemonic::TXA => "TXA",
                Mnemonic::TXS => "TXS",
                Mnemonic::TYA => "TYA",
            }
            .to_string()
    }
}

impl<'a> InstructionDecoder<'a> {
    pub fn new(bytes: &'a [u8], start_offset: usize) -> Self {

        InstructionDecoder {
            bytes: &bytes[start_offset..],
            pc: 0,
            start_offset: start_offset as u16,
        }
    }


    pub fn read(&mut self) -> Option<Instruction> {
        self.read_instruction().and_then(|(opcode, instr, offset)| {
            self.read_addressing_mode(opcode).map(|am| {
                Instruction {
                    offset: self.start_offset + offset,
                    mnemonic: instr,
                    addressing_mode: am,
                }
            })
        })
    }

    fn read8(&mut self) -> Option<u8> {
        let pc = self.pc;
        if pc < self.bytes.len() {
            let val = self.bytes[pc];
            self.pc += 1;
            Some(val)
        } else {
            None
        }
    }

    fn read16(&mut self) -> Option<u16> {
        let pc = self.pc;
        if pc + 1 < self.bytes.len() {
            let byte1 = self.bytes[pc];
            let byte2 = self.bytes[pc + 1];
            let val = byte1 as u16 | (byte2 as u16) << 8;
            self.pc += 2;
            Some(val)
        } else {
            None
        }
    }

    fn read_indexed_indirect(&mut self) -> Option<AddressingMode> {
        self.read8().map(AddressingMode::IndexedIndirect)
    }

    fn read_indirect_indexed(&mut self) -> Option<AddressingMode> {
        self.read8().map(AddressingMode::IndirectIndexed)
    }

    fn read_zp(&mut self) -> Option<AddressingMode> {
        self.read8().map(AddressingMode::ZeroPage)
    }

    fn read_immediate(&mut self) -> Option<AddressingMode> {
        self.read8().map(AddressingMode::Immediate)
    }

    fn read_abs(&mut self) -> Option<AddressingMode> {
        self.read16().map(AddressingMode::Absolute)
    }

    fn read_zpx(&mut self) -> Option<AddressingMode> {
        self.read8().map(AddressingMode::ZeroPageX)
    }

    fn read_absy(&mut self) -> Option<AddressingMode> {
        self.read16().map(AddressingMode::AbsoluteY)
    }

    fn read_absx(&mut self) -> Option<AddressingMode> {
        self.read16().map(AddressingMode::AbsoluteX)

    }

    fn read_relative(&mut self) -> Option<AddressingMode> {
        self.read8().map(|i| i as i8).map(AddressingMode::Relative)
    }

    fn read_instruction(&mut self) -> Option<(u8, Mnemonic, u16)> {
        self.read8().and_then(|opcode| {
            let offset = (self.pc as u16) - 1;
            match opcode {
                    0x0 => {
                        self.pc += 1; // Break has an additional padding byte
                        Some(Mnemonic::BRK)
                    }
                    0x40 => Some(Mnemonic::RTI),
                    0x60 => Some(Mnemonic::RTS),
                    0x08 => Some(Mnemonic::PHP),
                    0x28 => Some(Mnemonic::PLP),
                    0x48 => Some(Mnemonic::PHA),
                    0x68 => Some(Mnemonic::PLA),
                    0x88 => Some(Mnemonic::DEY),
                    0xa8 => Some(Mnemonic::TAY),
                    0xc8 => Some(Mnemonic::INY),
                    0xe8 => Some(Mnemonic::INX),
                    0x18 => Some(Mnemonic::CLC),
                    0x38 => Some(Mnemonic::SEC),
                    0x58 => Some(Mnemonic::CLI),
                    0x78 => Some(Mnemonic::SEI),
                    0x98 => Some(Mnemonic::TYA),
                    0xb8 => Some(Mnemonic::CLV),
                    0xd8 => Some(Mnemonic::CLD),
                    0xf8 => Some(Mnemonic::SED),
                    0x8a => Some(Mnemonic::TXA),
                    0x9a => Some(Mnemonic::TXS),
                    0xaa => Some(Mnemonic::TAX),
                    0xba => Some(Mnemonic::TSX),
                    0xca => Some(Mnemonic::DEX),
                    0xea => Some(Mnemonic::NOP),
                    0x10 => Some(Mnemonic::BPL),
                    0x30 => Some(Mnemonic::BMI),
                    0x50 => Some(Mnemonic::BVC),
                    0x70 => Some(Mnemonic::BVS),
                    0x90 => Some(Mnemonic::BCC),
                    0xb0 => Some(Mnemonic::BCS),
                    0xd0 => Some(Mnemonic::BNE),
                    0xf0 => Some(Mnemonic::BEQ),
                    0x20 => Some(Mnemonic::JSR),
                    _ => {
                        let instr_fam = opcode & INSTR_FAMILY_MASK;
                        match instr_fam {
                            0b01 => Self::decode_family01_instruction(opcode),
                            0b10 => Self::decode_family10_instruction(opcode),
                            0b00 => Self::decode_family00_instruction(opcode),
                            _ => None,
                        }
                    }
                }
                .map(|instr| (opcode, instr, offset))
        })
    }

    fn read_addressing_mode(&mut self, opcode: u8) -> Option<AddressingMode> {
        match opcode {
            0x0 | 0x40 | 0x60 | 0x08 | 0x28 | 0x48 | 0x68 | 0x88 | 0xa8 | 0xc8 | 0xe8 | 0x18 |
            0x38 | 0x58 | 0x78 | 0x98 | 0xb8 | 0xd8 | 0xf8 | 0x8a | 0x9a | 0xaa | 0xba | 0xca |
            0xea => Some(AddressingMode::Implied),
            0x10 | 0x30 | 0x50 | 0x70 | 0x90 | 0xb0 | 0xd0 | 0xf0 => self.read_relative(),
            0x20 => self.read_abs(),
            _ => {
                let instr_fam = opcode & INSTR_FAMILY_MASK;
                match instr_fam {
                    0b01 => self.decode_family01_addressing_mode(opcode),
                    0b10 => self.decode_family10_addressing_mode(opcode),
                    0b00 => self.decode_family00_addressing_mode(opcode),
                    _ => None,
                }
            }
        }
    }

    fn decode_family01_addressing_mode(&mut self, opcode: u8) -> Option<AddressingMode> {
        let am = (opcode >> 2) & ADDRESSING_MODE_MASK;
        match am {
            0b0 => self.read_indexed_indirect(),
            0b1 => self.read_zp(),
            0b10 => self.read_immediate(),
            0b11 => self.read_abs(),
            0b100 => self.read_indirect_indexed(),
            0b101 => self.read_zpx(),
            0b110 => self.read_absy(),
            0b111 => self.read_absx(),
            _ => None,
        }
    }

    fn decode_family10_addressing_mode(&mut self, opcode: u8) -> Option<AddressingMode> {
        let am = (opcode >> 2) & ADDRESSING_MODE_MASK;
        match am {
            0b0 => self.read_immediate(),
            0b1 => self.read_zp(),
            0b10 => Some(AddressingMode::Accumulator),
            0b11 => self.read_abs(),
            0b101 => self.read_zpx(),
            0b111 => self.read_absx(),
            _ => None,
        }
    }

    fn decode_family00_addressing_mode(&mut self, opcode: u8) -> Option<AddressingMode> {
        let am = (opcode >> 2) & ADDRESSING_MODE_MASK;
        match am {
            0b0 => self.read_immediate(),
            0b1 => self.read_zp(),
            0b11 => self.read_abs(),
            0b101 => self.read_zpx(),
            0b111 => self.read_absx(),
            _ => None,
        }
    }

    fn decode_family01_instruction(opcode: u8) -> Option<Mnemonic> {
        let instr = (opcode >> 5) & INSTR_MASK;
        match instr {
            0b0 => Some(Mnemonic::ORA),
            0b1 => Some(Mnemonic::AND),
            0b10 => Some(Mnemonic::EOR),
            0b11 => Some(Mnemonic::ADC),
            0b100 => Some(Mnemonic::STA),
            0b101 => Some(Mnemonic::LDA),
            0b110 => Some(Mnemonic::CMP),
            0b111 => Some(Mnemonic::SBC),
            _ => None,
        }
    }

    fn decode_family00_instruction(opcode: u8) -> Option<Mnemonic> {
        let instr = (opcode >> 5) & INSTR_MASK;
        match instr {
            0b1 => Some(Mnemonic::BIT),
            0b10 | 0b11 => Some(Mnemonic::JMP),
            0b100 => Some(Mnemonic::STY),
            0b101 => Some(Mnemonic::LDY),
            0b110 => Some(Mnemonic::CPY),
            0b111 => Some(Mnemonic::CPX),
            _ => None,
        }
    }

    fn decode_family10_instruction(opcode: u8) -> Option<Mnemonic> {
        let instr = (opcode >> 5) & INSTR_MASK;
        match instr {
            0b0 => Some(Mnemonic::ASL),
            0b1 => Some(Mnemonic::ROL),
            0b10 => Some(Mnemonic::LSR),
            0b11 => Some(Mnemonic::ROR),
            0b100 => Some(Mnemonic::STX),
            0b101 => Some(Mnemonic::LDX),
            0b110 => Some(Mnemonic::DEC),
            0b111 => Some(Mnemonic::INC),
            _ => None,
        }
    }
}

impl<'a> Iterator for InstructionDecoder<'a> {
    type Item = Instruction;

    fn next(&mut self) -> Option<Instruction> {
        self.read()
    }
}
