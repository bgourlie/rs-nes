use cpu::Cpu;
use cpu::opcodes::addressing::AddressingMode;
use memory::Memory;

pub struct Accumulator {
    value: u8,
}

impl Accumulator {
    pub fn init<M: Memory>(cpu: &mut Cpu<M>) -> Result<Self, ()> {
        // dummy read cycle
        cpu.tick()?;
        Ok(Accumulator { value: cpu.registers.acc })
    }
}

impl<M: Memory> AddressingMode<M> for Accumulator {
    type Output = u8;

    fn read(&self) -> Self::Output {
        self.value
    }

    fn write(&self, cpu: &mut Cpu<M>, value: u8) -> Result<(), ()> {
        cpu.registers.acc = value;
        Ok(())
    }
}
