use cpu::Cpu;
use cpu::opcodes::addressing::AddressingMode;
use memory::Memory;

pub struct Immediate {
    value: u8,
}

impl Immediate {
    pub fn init<M: Memory>(cpu: &mut Cpu<M>) -> Result<Self, ()> {
        let val = cpu.read_pc()?;
        Ok(Immediate { value: val })
    }
}

impl<M: Memory> AddressingMode<M> for Immediate {
    type Output = u8;

    fn read(&self) -> u8 {
        self.value
    }
}
