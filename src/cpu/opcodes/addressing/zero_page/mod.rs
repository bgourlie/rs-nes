use super::AddressingMode;
use cpu::Cpu;
use memory::Memory;

pub struct ZeroPage {
    addr: u16,
    value: u8,
}

impl ZeroPage {
    pub fn new<F: Fn(&Cpu<M>), M: Memory>(cpu: &mut Cpu<M>, _: F) -> Self {
        let addr = cpu.read_op() as u16;
        let val = cpu.memory.load(addr);

        ZeroPage {
            addr: addr,
            value: val,
        }
    }
}

impl<M: Memory> AddressingMode<M> for ZeroPage {
    type Output = u8;

    fn read(&self) -> Self::Output {
        self.value
    }
}
