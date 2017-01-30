use cpu::Cpu;
use cpu::opcodes::addressing::AddressingMode;
use memory::Memory;

pub struct ZeroPage {
    addr: u16,
    value: u8,
    is_store: bool,
}

impl ZeroPage {
    pub fn init<M: Memory>(cpu: &mut Cpu<M>) -> Self {
        let addr = cpu.read_pc() as u16;
        let val = cpu.read_memory(addr);

        ZeroPage {
            addr: addr,
            value: val,
            is_store: false,
        }
    }

    pub fn init_store<M: Memory>(cpu: &mut Cpu<M>) -> Self {
        let addr = cpu.read_pc() as u16;

        // Read must consume a cycle for stores, so we call cpu.memory.load() directly
        let val = cpu.memory.load(addr);

        ZeroPage {
            addr: addr,
            value: val,
            is_store: true,
        }
    }
}

impl<M: Memory> AddressingMode<M> for ZeroPage {
    type Output = u8;

    fn read(&self) -> Self::Output {
        self.value
    }

    fn write(&self, cpu: &mut Cpu<M>, value: u8) {
        if !self.is_store {
            // Dummy write cycle
            cpu.tick();
        }
        cpu.write_memory(self.addr, value)
    }
}
