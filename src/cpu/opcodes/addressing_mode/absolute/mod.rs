use super::AddressingMode;
use cpu::Cpu;
use memory::Memory;

#[derive(Default)]
pub struct Absolute {
    addr: u16,
    operand: u8,
}

impl Absolute {
    pub fn new<F: Fn(&Cpu<M>), M: Memory>(cpu: &mut Cpu<M>, tick_handler: F) -> Self {
        let low_byte = cpu.read_op();
        tick_handler(cpu);
        let high_byte = cpu.read_op();
        tick_handler(cpu);
        let addr = low_byte as u16 | (high_byte as u16) << 8;
        let operand = cpu.memory.load(addr);
        tick_handler(cpu);

        Absolute {
            addr: addr,
            operand: operand,
        }
    }
}

impl<M: Memory> AddressingMode<M> for Absolute {
    fn operand(&self) -> u8 {
        self.operand
    }

    //    fn write<F: Fn(&Cpu<M>)>(&self, cpu: &mut Cpu<M>, value: u8, tick_handler: F) {
    //        tick_handler(cpu);
    //        cpu.memory.store(self.addr, value);
    //        tick_handler(cpu);
    //    }
}
