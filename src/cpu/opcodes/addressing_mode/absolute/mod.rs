use super::AddressingMode;
use cpu::Cpu;
use memory::Memory;

#[derive(Default)]
pub struct Absolute {
    addr: u16,
}

impl<M: Memory> AddressingMode<M> for Absolute {
    fn operand<F: Fn(&Cpu<M>)>(&mut self, cpu: &mut Cpu<M>, tick_handler: F) -> u8 {
        let low_byte = cpu.read_op();
        tick_handler(cpu);
        let high_byte = cpu.read_op();
        tick_handler(cpu);
        let addr = low_byte as u16 | (high_byte as u16) << 8;
        self.addr = addr;
        let val = cpu.memory.load(addr);
        tick_handler(cpu);
        val
    }

    //    fn write<F: Fn(&Cpu<M>)>(&self, cpu: &mut Cpu<M>, value: u8, tick_handler: F) {
    //        tick_handler(cpu);
    //        cpu.memory.store(self.addr, value);
    //        tick_handler(cpu);
    //    }
}
