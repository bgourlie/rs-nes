use cpu::Cpu;
use cpu::opcodes::addressing::AddressingMode;
use errors::*;
use memory::Memory;

pub struct Absolute {
    addr: u16,
    value: u8,
    is_store: bool,
}

impl Absolute {
    pub fn init<M: Memory>(cpu: &mut Cpu<M>) -> Result<Self> {
        let addr = cpu.read_pc16()?;
        let value = cpu.read_memory(addr)?;

        Ok(Absolute {
            addr: addr,
            value: value,
            is_store: false,
        })
    }

    pub fn init_store<M: Memory>(cpu: &mut Cpu<M>) -> Result<Self> {
        let addr = cpu.read_pc16()?;

        // Read must consume a cycle for stores, so we call cpu.memory.load() directly
        let value = cpu.memory.load(addr)?;

        Ok(Absolute {
            addr: addr,
            value: value,
            is_store: true,
        })
    }
}

impl<M: Memory> AddressingMode<M> for Absolute {
    type Output = u8;

    fn read(&self) -> Self::Output {
        self.value
    }

    fn write(&self, cpu: &mut Cpu<M>, value: u8) -> Result<()> {
        if !self.is_store {
            // Dummy write cycle
            cpu.tick()?;
        }
        cpu.write_memory(self.addr, value)
    }
}
