use std::io::Cursor;

use byteorder::{LittleEndian, ReadBytesExt};
use serde::{Serialize, Serializer};

use memory::{Memory, ADDRESSABLE_MEMORY};
use cpu::registers::Registers;
use cpu::disassembler::{Instruction, InstructionDecoder};

pub struct CpuSnapshot {
    instructions: Vec<Instruction>,
    registers: Registers,
    cycles: u64,
    memory: Vec<i32>,
}

impl CpuSnapshot {
    pub fn new<Mem: Memory>(mem: Mem,
                            registers: Registers,
                            cycles: u64,
                            prg_offset: usize)
                            -> Self {
        let mut buf = Vec::with_capacity(ADDRESSABLE_MEMORY);
        mem.dump(&mut buf);
        let instructions = InstructionDecoder::window(&buf, prg_offset, registers.pc as usize, 128);
        let packed_bytes = pack_memory(&buf);

        CpuSnapshot {
            instructions: instructions,
            registers: registers,
            cycles: cycles,
            memory: packed_bytes,
        }
    }
}

impl Serialize for CpuSnapshot {
    fn serialize<S: Serializer>(&self, serializer: &mut S) -> Result<(), S::Error> {
        let mut state = serializer.serialize_struct("CpuSnapshot", 2)?;
        serializer.serialize_struct_elt(&mut state, "instructions", &self.instructions)?;
        serializer.serialize_struct_elt(&mut state, "registers", &self.registers)?;
        serializer.serialize_struct_elt(&mut state, "cycles", self.cycles)?;
        serializer.serialize_struct_elt(&mut state, "memory", &self.memory)?;
        serializer.serialize_struct_end(state)
    }
}

// Convert an array of bytes into an array 32-bit signed integers.
//
// This is done to reduce the json payload when serializing memory. Once elm supports binary data,
// this shouldn't be necessary.
fn pack_memory(rom: &[u8]) -> Vec<i32> {
    let buf_size = ADDRESSABLE_MEMORY / 4;
    let mut packed = Vec::<i32>::with_capacity(buf_size);
    for i in 0..(ADDRESSABLE_MEMORY / 4) {
        let bytes = {
            let index = i * 4;
            let b1 = rom[index];
            let b2 = rom[index + 1];
            let b3 = rom[index + 2];
            let b4 = rom[index + 3];
            [b1, b2, b3, b4]
        };

        let mut buffer = Cursor::new(&bytes[..]);
        let val = buffer.read_i32::<LittleEndian>().unwrap();
        packed.push(val);
    }
    packed
}
