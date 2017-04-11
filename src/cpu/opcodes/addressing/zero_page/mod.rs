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

        ZeroPage {
            addr: addr,
            value: 0x0, // Stores don't read memory, can cause illegal memory access if attempted
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
