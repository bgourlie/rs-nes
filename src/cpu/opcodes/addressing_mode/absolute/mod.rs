use std::marker::PhantomData;
use super::AddressingMode;
use cpu::Cpu;
use memory::Memory;

pub struct Absolute<M: Memory, F: Fn(&Cpu<M>)> {
    addr: u16,
    value: u8,
    tick_handler: F,
    phantom: PhantomData<M>,
}

impl<M: Memory, F: Fn(&Cpu<M>)> Absolute<M, F> {
    pub fn new(cpu: &mut Cpu<M>, tick_handler: F) -> Self {
        let low_byte = cpu.read_op();
        tick_handler(cpu);
        let high_byte = cpu.read_op();
        tick_handler(cpu);
        let addr = low_byte as u16 | (high_byte as u16) << 8;
        let operand = cpu.memory.load(addr);
        tick_handler(cpu);

        Absolute {
            addr: addr,
            value: operand,
            tick_handler: tick_handler,
            phantom: PhantomData,
        }
    }
}

impl<M: Memory, F: Fn(&Cpu<M>)> AddressingMode<M> for Absolute<M, F> {
    fn operand(&self) -> u8 {
        self.value
    }

    fn write(&self, cpu: &mut Cpu<M>, value: u8) {
        (self.tick_handler)(cpu);
        cpu.memory.store(self.addr, value);
        (self.tick_handler)(cpu);
    }
}
