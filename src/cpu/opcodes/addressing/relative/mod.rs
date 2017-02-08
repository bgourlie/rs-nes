use cpu::Cpu;
use cpu::opcodes::addressing::AddressingMode;
use errors::*;
use memory::Memory;

pub struct Relative {
    offset: i8,
}

impl Relative {
    pub fn init<M: Memory>(cpu: &mut Cpu<M>) -> Result<Self> {
        let offset = cpu.read_pc()? as i8;
        Ok(Relative { offset: offset })
    }
}

impl<M: Memory> AddressingMode<M> for Relative {
    type Output = i8;

    fn read(&self) -> Self::Output {
        self.offset
    }
}
