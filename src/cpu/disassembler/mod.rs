static INSTR_MASK: u8 = 0b111;
static INSTR_FAMILY_MASK: u8 = 0b11;
static ADDRESSING_MODE_MASK: u8 = 0b111;


pub struct Decoder<'a> {
    bytes: &'a [u8],
    pc: usize,
}

impl<'a> Decoder<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Decoder {
            bytes: bytes,
            pc: 0,
        }
    }

    pub fn read(&mut self) -> Option<String> {
        self.read_instruction().and_then(|(opcode, instr)| {
            self.read_addressing_mode(opcode).map(|am| format!("{} {}", instr, am))
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

    fn read_indexed_indirect(&mut self) -> Option<String> {
        self.read8().map(|val| format!("(${:0>2X},X)", val))
    }

    fn read_indirect_indexed(&mut self) -> Option<String> {
        self.read8().map(|val| format!("(${:0>2X}),Y", val))
    }

    fn read_zp(&mut self) -> Option<String> {
        self.read8().map(|val| format!("${:0>2X}", val))
    }

    fn read_immediate(&mut self) -> Option<String> {
        self.read8().map(|val| format!("#${:0>2X}", val))
    }

    fn read_abs(&mut self) -> Option<String> {
        self.read16().map(|val| format!("${:0>4X}", val))
    }

    fn read_zpx(&mut self) -> Option<String> {
        self.read8().map(|val| format!("${:0>2X},X", val))
    }

    fn read_absy(&mut self) -> Option<String> {
        self.read16().map(|val| format!("${:0>4X},Y", val))
    }

    fn read_absx(&mut self) -> Option<String> {
        self.read16().map(|val| format!("${:0>4X},X", val))

    }

    fn read_relative(&mut self) -> Option<String> {
        self.read8().map(|val| format!("${:0>2X}", val))
    }

    fn read_instruction(&mut self) -> Option<(u8, String)> {
        self.read8().and_then(|opcode| {
            match opcode {
                    0x0 => {
                        self.pc += 1; // Break has an additional padding byte
                        Some("BRK".to_string())
                    }
                    0x40 => Some("RTI".to_string()),
                    0x60 => Some("RTS".to_string()),
                    0x08 => Some("PHP".to_string()),
                    0x28 => Some("PLP".to_string()),
                    0x48 => Some("PHA".to_string()),
                    0x68 => Some("PLA".to_string()),
                    0x88 => Some("DEY".to_string()),
                    0xa8 => Some("TAY".to_string()),
                    0xc8 => Some("INY".to_string()),
                    0xe8 => Some("INX".to_string()),
                    0x18 => Some("CLC".to_string()),
                    0x38 => Some("SEC".to_string()),
                    0x58 => Some("CLI".to_string()),
                    0x78 => Some("SEI".to_string()),
                    0x98 => Some("TYA".to_string()),
                    0xb8 => Some("CLV".to_string()),
                    0xd9 => Some("CLD".to_string()),
                    0xf8 => Some("SED".to_string()),
                    0x8a => Some("TXA".to_string()),
                    0x9a => Some("TXS".to_string()),
                    0xaa => Some("TAX".to_string()),
                    0xba => Some("TSX".to_string()),
                    0xca => Some("DEX".to_string()),
                    0xea => Some("NOP".to_string()),
                    0x10 => Some("BPL".to_string()),
                    0x30 => Some("BMI".to_string()),
                    0x50 => Some("BVC".to_string()),
                    0x70 => Some("BVS".to_string()),
                    0x90 => Some("BCC".to_string()),
                    0xb0 => Some("BCS".to_string()),
                    0xd0 => Some("BNE".to_string()),
                    0xf0 => Some("BEQ".to_string()),
                    0x20 => Some("JSR".to_string()),
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
                .map(|instr| (opcode, instr))
        })
    }

    fn read_addressing_mode(&mut self, opcode: u8) -> Option<String> {
        match opcode {
            0x0 | 0x40 | 0x60 | 0x08 | 0x28 | 0x48 | 0x68 | 0x88 | 0xa8 | 0xc8 | 0xe8 | 0x18 |
            0x38 | 0x58 | 0x78 | 0x98 | 0xb8 | 0xd8 | 0xf8 | 0x8a | 0x9a | 0xaa | 0xba | 0xca |
            0xea => Some("".to_string()), // implied mode
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

    fn decode_family01_addressing_mode(&mut self, opcode: u8) -> Option<String> {
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

    fn decode_family10_addressing_mode(&mut self, opcode: u8) -> Option<String> {
        let am = (opcode >> 2) & ADDRESSING_MODE_MASK;
        match am {
            0b0 => self.read_immediate(),
            0b1 => self.read_zp(),
            0b10 => Some("A".to_string()),
            0b11 => self.read_abs(),
            0b100 => None,
            0b101 => self.read_zpx(),
            0b110 => None,
            0b111 => self.read_absx(),
            _ => None,
        }
    }

    fn decode_family00_addressing_mode(&mut self, opcode: u8) -> Option<String> {
        let am = (opcode >> 2) & ADDRESSING_MODE_MASK;
        match am {
            0b0 => self.read_immediate(),
            0b1 => self.read_zp(),
            0b10 => None,
            0b11 => self.read_abs(),
            0b100 => None,
            0b101 => self.read_zpx(),
            0b110 => None,
            0b111 => self.read_absx(),
            _ => None,
        }
    }

    fn decode_family01_instruction(opcode: u8) -> Option<String> {
        let instr = (opcode >> 5) & INSTR_MASK;
        match instr {
            0b0 => Some("ORA".to_string()),
            0b1 => Some("AND".to_string()),
            0b10 => Some("EOR".to_string()),
            0b11 => Some("ADC".to_string()),
            0b100 => Some("STA".to_string()),
            0b101 => Some("LDA".to_string()),
            0b110 => Some("CMP".to_string()),
            0b111 => Some("SBC".to_string()),
            _ => None,
        }
    }

    fn decode_family00_instruction(opcode: u8) -> Option<String> {
        let instr = (opcode >> 5) & INSTR_MASK;
        match instr {
            0b0 => None,
            0b1 => Some("BIT".to_string()),
            0b10 => Some("JMP".to_string()),
            0b11 => Some("JMP".to_string()),
            0b100 => Some("STY".to_string()),
            0b101 => Some("LDY".to_string()),
            0b110 => Some("CPY".to_string()),
            0b111 => Some("CPX".to_string()),
            _ => None,
        }
    }

    fn decode_family10_instruction(opcode: u8) -> Option<String> {
        let instr = (opcode >> 5) & INSTR_MASK;
        match instr {
            0b0 => Some("ASL".to_string()),
            0b1 => Some("ROL".to_string()),
            0b10 => Some("LSR".to_string()),
            0b11 => Some("ROR".to_string()),
            0b100 => Some("STX".to_string()),
            0b101 => Some("LDX".to_string()),
            0b110 => Some("DEC".to_string()),
            0b111 => Some("INC".to_string()),
            _ => None,
        }
    }
}

impl<'a> Iterator for Decoder<'a> {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        self.read()
    }
}
