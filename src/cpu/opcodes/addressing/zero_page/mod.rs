use super::AddressingMode;
use cpu::Cpu;
use memory::Memory;

pub struct ZeroPage {
    addr: u16,
    value: u8,
}

impl ZeroPage {
    pub fn new<F: Fn(&Cpu<M>), M: Memory>(cpu: &mut Cpu<M>, tick_handler: F) -> Self {
        let addr = cpu.read_pc(&tick_handler) as u16;
        let val = cpu.read_memory(addr, &tick_handler);

        ZeroPage {
            addr: addr,
            value: val,
        }
    }
}

impl<M: Memory> AddressingMode<M> for ZeroPage {
    type Output = u8;

    fn read(&self) -> Self::Output {
        self.value
    }

    fn write<F: Fn(&Cpu<M>)>(&self, cpu: &mut Cpu<M>, value: u8, tick_handler: F) {
        cpu.write_memory(self.addr, value, &tick_handler)
    }
}
