use super::AddressingMode;
use cpu::Cpu;
use memory::Memory;

#[derive(Default)]
pub struct AbsoluteX {
    addr: u16,
}

impl<M: Memory> AddressingMode<M> for AbsoluteX {
    fn operand<F: Fn(&Cpu<M>)>(&mut self, cpu: &mut Cpu<M>, tick_handler: F) -> u8 {
        let low_byte = cpu.read_op();
        tick_handler(cpu);
        let high_byte = cpu.read_op();
        tick_handler(cpu);
        let base_addr = low_byte as u16 | (high_byte as u16) << 8;
        let target_addr = base_addr + cpu.registers.x as u16;

        // Conditional cycle if memory page crossed
        if base_addr & 0xff00 != target_addr & 0xff00 {
            tick_handler(cpu);
        }

        let val = cpu.memory.load(base_addr);
        tick_handler(cpu);
        self.addr = target_addr;
        val
    }

    //    fn write<F: Fn(&Cpu<M>)>(&self, cpu: &mut Cpu<M>, value: u8, tick_handler: F) {
    //        tick_handler(cpu);
    //        cpu.memory.store(self.addr, value);
    //        tick_handler(cpu);
    //    }
}
