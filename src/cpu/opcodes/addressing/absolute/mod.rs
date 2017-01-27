use cpu::Cpu;
use cpu::opcodes::addressing::AddressingMode;
use memory::Memory;

pub struct Absolute {
    addr: u16,
    value: u8,
    is_store: bool,
}

impl Absolute {
    pub fn init<M: Memory, F: Fn(&Cpu<M>)>(cpu: &mut Cpu<M>, tick_handler: F) -> Self {
        let addr = cpu.read_pc16(&tick_handler);
        let value = cpu.read_memory(addr, &tick_handler);

        Absolute {
            addr: addr,
            value: value,
            is_store: false,
        }
    }

    pub fn init_store<M: Memory, F: Fn(&Cpu<M>)>(cpu: &mut Cpu<M>, tick_handler: F) -> Self {
        let addr = cpu.read_pc16(&tick_handler);

        // Read must consume a cycle for stores, so we call cpu.memory.load() directly
        let value = cpu.memory.load(addr);

        Absolute {
            addr: addr,
            value: value,
            is_store: true,
        }
    }
}

impl<M: Memory> AddressingMode<M> for Absolute {
    type Output = u8;

    fn read(&self) -> Self::Output {
        self.value
    }

    fn write<F: Fn(&Cpu<M>)>(&self, cpu: &mut Cpu<M>, value: u8, tick_handler: F) {
        if !self.is_store {
            // Dummy write cycle
            tick_handler(cpu);
        }
        cpu.write_memory(self.addr, value, &tick_handler);
    }
}
