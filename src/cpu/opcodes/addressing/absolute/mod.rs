use cpu::Cpu;
use cpu::opcodes::addressing::AddressingMode;
use memory::Memory;
use screen::Screen;

pub struct Absolute {
    addr: u16,
    value: u8,
    is_store: bool,
}

impl Absolute {
    pub fn init<S: Screen, M: Memory<S>>(cpu: &mut Cpu<S, M>) -> Self {
        let addr = cpu.read_pc16();
        let value = cpu.read_memory(addr);

        Absolute {
            addr: addr,
            value: value,
            is_store: false,
        }
    }

    pub fn init_store<S: Screen, M: Memory<S>>(cpu: &mut Cpu<S, M>) -> Self {
        let addr = cpu.read_pc16();

        Absolute {
            addr: addr,
            value: 0, // Stores don't use the value and can cause illegal memory access if attempted
            is_store: true,
        }
    }
}

impl<S: Screen, M: Memory<S>> AddressingMode<S, M> for Absolute {
    type Output = u8;

    fn read(&self) -> Self::Output {
        self.value
    }

    fn write(&self, cpu: &mut Cpu<S, M>, value: u8) {
        if !self.is_store {
            // Dummy write cycle
            cpu.tick();
        }
        cpu.write_memory(self.addr, value)
    }
}
