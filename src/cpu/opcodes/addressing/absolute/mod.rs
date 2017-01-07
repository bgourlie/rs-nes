use std::marker::PhantomData;
use super::AddressingMode;
use super::absolute_base::read_address;
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
        let addr = read_address(cpu, &tick_handler);
        let operand = cpu.read_memory(addr, &tick_handler);

        Absolute {
            addr: addr,
            value: operand,
            tick_handler: tick_handler,
            phantom: PhantomData,
        }
    }
}

impl<M: Memory, F: Fn(&Cpu<M>)> AddressingMode<M> for Absolute<M, F> {
    type Output = u8;

    fn read(&self) -> Self::Output {
        self.value
    }

    fn write(&self, cpu: &mut Cpu<M>, value: u8) {
        cpu.write_memory(self.addr, value, &self.tick_handler);
    }
}
