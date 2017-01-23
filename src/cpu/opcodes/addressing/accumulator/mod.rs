use cpu::Cpu;
use cpu::opcodes::addressing::AddressingMode;
use memory::Memory;

pub struct Accumulator {
    value: u8,
}

impl Accumulator {
    pub fn init<M: Memory, F: Fn(&Cpu<M>)>(cpu: &mut Cpu<M>, tick_handler: F) -> Self {
        // dummy read cycle
        tick_handler(cpu);
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
