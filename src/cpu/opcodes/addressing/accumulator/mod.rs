use cpu::Cpu;
use cpu::opcodes::addressing::AddressingMode;
use memory::Memory;
use screen::Screen;

pub struct Accumulator {
    value: u8,
}

impl Accumulator {
    pub fn init<S: Screen, M: Memory<S>>(cpu: &mut Cpu<S, M>) -> Self {
        // dummy read cycle
        cpu.tick();
        Accumulator { value: cpu.registers.acc }
    }
}

impl<S: Screen, M: Memory<S>> AddressingMode<S, M> for Accumulator {
    type Output = u8;

    fn read(&self) -> Self::Output {
        self.value
    }

    fn write(&self, cpu: &mut Cpu<S, M>, value: u8) {
        cpu.registers.acc = value;
    }
}
