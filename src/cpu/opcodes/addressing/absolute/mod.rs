use cpu::Cpu;
use cpu::opcodes::addressing::AddressingMode;
use memory::Memory;

pub struct Absolute {
    addr: u16,
    value: u8,
}

impl Absolute {
    pub fn init<M: Memory, F: Fn(&Cpu<M>)>(cpu: &mut Cpu<M>, tick_handler: F) -> Self {
        let addr = cpu.read_pc16(&tick_handler);
        let value = cpu.read_memory(addr, &tick_handler);

        Absolute {
            addr: addr,
            value: value,
        }
    }
}

impl<M: Memory> AddressingMode<M> for Absolute {
    type Output = u8;

    fn read(&self) -> Self::Output {
        self.value
    }

    fn write<F: Fn(&Cpu<M>)>(&self, cpu: &mut Cpu<M>, value: u8, tick_handler: F) {
        cpu.write_memory(self.addr, value, &tick_handler);
    }
}
