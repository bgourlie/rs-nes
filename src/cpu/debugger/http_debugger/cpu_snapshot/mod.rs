use std::io::Cursor;

use byteorder::{LittleEndian, ReadBytesExt};
use serde::{Serialize, Serializer};
use cpu::registers::Registers;

use memory::{Memory, ADDRESSABLE_MEMORY};

pub struct CpuSnapshot<M: Memory> {
    pub memory: M,
    pub registers: Registers,
    pub cycles: u64,
}

impl<M: Memory> CpuSnapshot<M> {
    pub fn new() -> Self {
        CpuSnapshot {
            memory: Default::default(),
            registers: Default::default(),
            cycles: 0,
        }
    }
}

impl<M: Memory> Serialize for CpuSnapshot<M> {
    fn serialize<S: Serializer>(&self, serializer: &mut S) -> Result<(), S::Error> {
        let mut buf = Vec::with_capacity(ADDRESSABLE_MEMORY);
        self.memory.dump(&mut buf);
        let packed = pack_memory(&buf);
        let mut state = try!(serializer.serialize_struct("Snapshot", 3));
        try!(serializer.serialize_struct_elt(&mut state, "cycles", self.cycles));
        try!(serializer.serialize_struct_elt(&mut state, "registers", &self.registers));
        try!(serializer.serialize_struct_elt(&mut state, "memory", packed));
        serializer.serialize_struct_end(state)
    }
}

impl Serialize for Registers {
    fn serialize<S: Serializer>(&self, serializer: &mut S) -> Result<(), S::Error> {
        let mut state = try!(serializer.serialize_struct("Registers", 6));
        try!(serializer.serialize_struct_elt(&mut state, "acc", self.acc));
        try!(serializer.serialize_struct_elt(&mut state, "x", self.irx));
        try!(serializer.serialize_struct_elt(&mut state, "y", self.iry));
        try!(serializer.serialize_struct_elt(&mut state, "pc", self.pc));
        try!(serializer.serialize_struct_elt(&mut state, "sp", self.sp));
        try!(serializer.serialize_struct_elt(&mut state, "stat", self.stat));
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
