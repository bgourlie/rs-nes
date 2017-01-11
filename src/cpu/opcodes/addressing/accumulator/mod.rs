use cpu::Cpu;
use memory::Memory;
use super::AddressingMode;

pub struct Accumulator {
    value: u8,
}

impl Accumulator {
    pub fn init<M: Memory>(cpu: &mut Cpu<M>) -> Self {
        Accumulator { value: cpu.registers.acc }
    }
}

impl<M: Memory> AddressingMode<M> for Accumulator {
    type Output = u8;

    fn read(&self) -> Self::Output {
        self.value
    }

    fn write<F: Fn(&Cpu<M>)>(&self, cpu: &mut Cpu<M>, value: u8, _: F) {
        cpu.registers.acc = value
    }
}
