static INSTR_MASK: u8 = 0b111;

pub fn decode(bytes: &[u8]) -> Vec<String> {
    let mut decoded = Vec::<String>::new();
    decoded
}

struct Decoder<'a> {
    bytes: &'a [u8],
    pc: usize
}

impl<'a> Decoder<'a> {
    fn read8(mut self) -> String {
        let pc = self.pc;
        let val = self.bytes[pc];
        self.pc += 1;
        format!("{:0>2X}", val)
    }

    fn read16(mut self) -> String {
        let pc = self.pc;
        let byte1 = self.bytes[pc - 1];
        let byte2 = self.bytes[pc];
        let val = byte1 as u16 | (byte2 as u16) << 8;
        self.pc += 2;
        format!("{:0>4X}", val)
    }
}

fn decode_family01_instruction(byte: u8) -> String {
    let instr = (byte >> 5) & INSTR_MASK;
    match instr {
        0b0 => "ORA",
        0b1 => "AND",
        0b10 => "EOR",
        0b11 => "ADC",
        0b100 => "STA",
        0b101 => "LDA",
        0b110 => "CMP",
        0b111 => "SBC",
        _ => "???"
    }.to_string()
}

fn decode_family00_instruction(byte: u8) -> String {
    let instr = (byte >> 5) & INSTR_MASK;
    match instr {
        0b0 => "???",
        0b1 => "BIT",
        0b10 => "JMP",
        0b11 => "JMP",
        0b100 => "STY",
        0b101 => "LDY",
        0b110 => "CPY",
        0b111 => "CPX",
        _ => "???"
    }.to_string()
}

fn decode_family10_instruction(byte: u8) -> String {
    let instr = (byte >> 5) & INSTR_MASK;
    match instr {
        0b0 => "ASL",
        0b1 => "ROL",
        0b10 => "LSR",
        0b11 => "ROR",
        0b100 => "STX",
        0b101 => "LDX",
        0b110 => "DEC",
        0b111 => "INC",
        _ => "???"
    }.to_string()
}
