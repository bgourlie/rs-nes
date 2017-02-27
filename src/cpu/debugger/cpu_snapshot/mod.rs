
use base64;
use cpu::Registers;
use screen::Screen;
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;

pub enum MemorySnapshot {
    NoChange(u64), // If no change, just send the hash.
    Updated(u64, Vec<u8>), // Updated, send hash and memory
}

pub struct CpuSnapshot<S: Screen + Serialize> {
    registers: Registers,
    memory: MemorySnapshot,
    cycles: u64,
    screen: S,
}

impl<S: Screen + Serialize> CpuSnapshot<S> {
    pub fn new(mem_snapshot: MemorySnapshot, registers: Registers, screen: S, cycles: u64) -> Self {
        CpuSnapshot {
            registers: registers,
            memory: mem_snapshot,
            cycles: cycles,
            screen: screen,
        }
    }
}

impl Serialize for MemorySnapshot {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match *self {
            MemorySnapshot::NoChange(hash) => {
                let mut state = serializer.serialize_struct("Memory", 2)?;
                state.serialize_field("state", "NoChange")?;
                state.serialize_field("hash", &hash)?;
                state.end()
            }
            MemorySnapshot::Updated(hash, ref memory) => {
                let base64 = base64::encode(&memory);
                let mut state = serializer.serialize_struct("Memory", 3)?;
                state.serialize_field("state", "Updated")?;
                state.serialize_field("hash", &hash)?;
                state.serialize_field("base64", &base64)?;
                state.end()
            }
        }
    }
}

impl<Scr: Screen + Serialize> Serialize for CpuSnapshot<Scr> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("CpuSnapshot", 4)?;
        state.serialize_field("registers", &self.registers)?;
        state.serialize_field("memory", &self.memory)?;
        state.serialize_field("cycles", &self.cycles)?;
        state.serialize_field("screen", &self.screen)?;
        state.end()
    }
}
