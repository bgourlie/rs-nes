use super::AddressingMode;
use super::absolute_base::read_address;
use cpu::Cpu;
use memory::Memory;

pub struct AbsoluteX {
    addr: u16,
    value: u8,
}

impl AbsoluteX {
    pub fn new<F: Fn(&Cpu<M>), M: Memory>(cpu: &mut Cpu<M>, tick_handler: F) -> Self {
        let base_addr = read_address(cpu, &tick_handler);
        let target_addr = base_addr + cpu.registers.x as u16;

        // Conditional cycle if memory page crossed
        if base_addr & 0xff00 != target_addr & 0xff00 {
            tick_handler(cpu);
        }

        let val = cpu.read_memory(base_addr, &tick_handler);

        AbsoluteX {
            addr: target_addr,
            value: val,
        }
    }
}

impl<M: Memory> AddressingMode<M> for AbsoluteX {
    type Output = u8;

    fn read(&self) -> Self::Output {
        self.value
    }
}
