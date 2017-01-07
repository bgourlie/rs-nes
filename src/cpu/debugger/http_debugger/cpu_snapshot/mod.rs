use std::io::Cursor;

use byteorder::{LittleEndian, ReadBytesExt};
use serde::{Serialize, Serializer};

use memory::{Memory, ADDRESSABLE_MEMORY};
use cpu::Registers;

pub enum MemorySnapshot<Mem: Memory> {
    NoChange(u64), // If no change, just send the hash.
    Updated(u64, Mem), // Updated, send hash and memory
}

pub struct CpuSnapshot<Mem: Memory> {
    registers: Registers,
    cycles: u64,
    memory: MemorySnapshot<Mem>,
}

impl<Mem: Memory> CpuSnapshot<Mem> {
    pub fn new(mem_snapshot: MemorySnapshot<Mem>, registers: Registers, cycles: u64) -> Self {

        CpuSnapshot {
            registers: registers,
            cycles: cycles,
            memory: mem_snapshot,
        }
    }
}

impl<Mem: Memory> Serialize for MemorySnapshot<Mem> {
    fn serialize<S: Serializer>(&self, serializer: &mut S) -> Result<(), S::Error> {
        match *self {
            MemorySnapshot::NoChange(hash) => {
                let mut state = serializer.serialize_struct("Memory", 2)?;
                serializer.serialize_struct_elt(&mut state, "state", "NoChange")?;
                serializer.serialize_struct_elt(&mut state, "hash", hash)?;
                serializer.serialize_struct_end(state)
            }
            MemorySnapshot::Updated(hash, ref memory) => {
                let mut buf = Vec::with_capacity(ADDRESSABLE_MEMORY);
                memory.dump(&mut buf);
                let packed_bytes = pack_memory(&buf);
                let mut state = serializer.serialize_struct("Memory", 3)?;
                serializer.serialize_struct_elt(&mut state, "state", "Updated")?;
                serializer.serialize_struct_elt(&mut state, "hash", hash)?;
                serializer.serialize_struct_elt(&mut state, "packed_bytes", packed_bytes)?;
                serializer.serialize_struct_end(state)
            }
        }
    }
}

impl<Mem: Memory> Serialize for CpuSnapshot<Mem> {
    fn serialize<S: Serializer>(&self, serializer: &mut S) -> Result<(), S::Error> {
        let mut state = serializer.serialize_struct("CpuSnapshot", 2)?;
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
