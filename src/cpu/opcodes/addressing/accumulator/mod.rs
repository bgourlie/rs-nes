use cpu::Cpu;
use memory::Memory;
use super::AddressingMode;

pub struct Accumulator {
    value: u8,
}

impl Accumulator {
    pub fn new<M: Memory>(cpu: &mut Cpu<M>) -> Self {
        Accumulator { value: cpu.registers.acc }
    }
}

impl<M: Memory> AddressingMode<M> for Accumulator {
    type Output = u8;

    fn read(&self) -> Self::Output {
        self.value
    }
}
