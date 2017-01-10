use cpu::Cpu;
use memory::Memory;
use super::AddressingMode;

pub struct IndirectIndexed {
    addr: u16,
    value: u8,
}

impl IndirectIndexed {
    pub fn init<F: Fn(&Cpu<M>), M: Memory>(cpu: &mut Cpu<M>, tick_handler: F) -> Self {
        let base_addr = cpu.read_pc(&tick_handler);
        let y = cpu.registers.y;
        let addr = cpu.read_memory16_zp(base_addr, &tick_handler) + y as u16;
        let val = cpu.read_memory(addr, &tick_handler);
        IndirectIndexed {
            addr: addr,
            value: val,
        }
    }
}

impl<M: Memory> AddressingMode<M> for IndirectIndexed {
    type Output = u8;

    fn read(&self) -> Self::Output {
        self.value
    }

    fn write<F: Fn(&Cpu<M>)>(&self, cpu: &mut Cpu<M>, value: u8, tick_handler: F) {
        cpu.write_memory(self.addr, value, &tick_handler)
    }
}
